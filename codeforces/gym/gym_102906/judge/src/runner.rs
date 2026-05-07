use std::fmt;
use std::io::{self, Read, Write};
use std::os::unix::process::{CommandExt, ExitStatusExt};
use std::process::{Child, Command, ExitStatus, Stdio};
use std::thread;
use std::time::{Duration, Instant};

#[cfg(target_os = "linux")]
use std::fs;

const ANSI_RESET: &str = "\x1b[0m";
const ANSI_GREEN: &str = "\x1b[32m";
const ANSI_YELLOW: &str = "\x1b[33m";
const ANSI_MAGENTA: &str = "\x1b[35m";
const ANSI_RED: &str = "\x1b[31m";
const MEMORY_POLL_INTERVAL: Duration = Duration::from_millis(25);

#[derive(Debug, Clone)]
pub struct RunRequest {
    pub command: String,
    pub stdin: Vec<u8>,
    pub limits: RunLimits,
}

#[derive(Debug, Clone, Copy)]
pub struct RunLimits {
    pub time_limit: Option<Duration>,
    pub memory_limit_bytes: Option<u64>,
}

impl Default for RunLimits {
    fn default() -> Self {
        Self {
            time_limit: Some(Duration::from_secs(1)),
            memory_limit_bytes: Some(512 * 1024 * 1024),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunStatus {
    Ok,
    TimeLimit,
    MemoryLimit,
    RuntimeError,
}

impl RunStatus {
    pub fn label(self) -> &'static str {
        match self {
            RunStatus::Ok => "OK",
            RunStatus::TimeLimit => "TL",
            RunStatus::MemoryLimit => "ML",
            RunStatus::RuntimeError => "RE",
        }
    }

    pub fn ansi_color(self) -> &'static str {
        match self {
            RunStatus::Ok => ANSI_GREEN,
            RunStatus::TimeLimit => ANSI_YELLOW,
            RunStatus::MemoryLimit => ANSI_MAGENTA,
            RunStatus::RuntimeError => ANSI_RED,
        }
    }

    pub fn colored_label(self) -> String {
        format!("{}{}{}", self.ansi_color(), self.label(), ANSI_RESET)
    }
}

#[derive(Debug, Clone)]
pub struct RunOutcome {
    pub status: RunStatus,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
    pub elapsed: Duration,
    pub exit_code: Option<i32>,
    pub signal: Option<i32>,
    pub peak_rss_bytes: Option<u64>,
    pub message: String,
}

impl RunOutcome {
    pub fn plain_summary(&self) -> String {
        format!("{} {}", self.status.label(), self.message)
    }

    pub fn colored_summary(&self) -> String {
        format!("{} {}", self.status.colored_label(), self.message)
    }
}

#[derive(Debug)]
enum ForcedStop {
    TimeLimit,
    MemoryLimit,
}

pub fn run_bash_command(request: RunRequest) -> io::Result<RunOutcome> {
    let started_at = Instant::now();
    let mut command = Command::new("bash");
    command
        .arg("-lc")
        .arg(&request.command)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let memory_limit_bytes = request.limits.memory_limit_bytes;
    unsafe {
        command.pre_exec(move || {
            make_child_process_group()?;
            if let Some(bytes) = memory_limit_bytes {
                set_memory_limit(bytes)?;
            }
            Ok(())
        });
    }

    let mut child = command.spawn()?;
    let child_pgid = child.id();

    let mut child_stdin = child
        .stdin
        .take()
        .ok_or_else(|| io::Error::other("child stdin was not captured"))?;
    let child_stdout = child
        .stdout
        .take()
        .ok_or_else(|| io::Error::other("child stdout was not captured"))?;
    let child_stderr = child
        .stderr
        .take()
        .ok_or_else(|| io::Error::other("child stderr was not captured"))?;

    let input = request.stdin;
    let stdin_thread = thread::spawn(move || {
        let result = child_stdin.write_all(&input);
        match result {
            Ok(()) => Ok(()),
            Err(err) if err.kind() == io::ErrorKind::BrokenPipe => Ok(()),
            Err(err) => Err(err),
        }
    });
    let stdout_thread = thread::spawn(move || read_all(child_stdout));
    let stderr_thread = thread::spawn(move || read_all(child_stderr));

    let mut forced_stop = None;
    let mut peak_rss_bytes = None;
    let mut next_memory_check = Instant::now();

    let exit_status = loop {
        if let Some(status) = child.try_wait()? {
            break status;
        }

        let elapsed = started_at.elapsed();
        if let Some(limit) = request.limits.time_limit {
            if elapsed >= limit {
                forced_stop = Some(ForcedStop::TimeLimit);
                kill_child_tree(child_pgid, &mut child)?;
                break child.wait()?;
            }
        }

        if let Some(limit) = request.limits.memory_limit_bytes {
            if Instant::now() >= next_memory_check {
                if let Some(rss) = process_group_rss_bytes(child_pgid)? {
                    peak_rss_bytes = Some(peak_rss_bytes.map_or(rss, |peak: u64| peak.max(rss)));
                    if rss > limit {
                        forced_stop = Some(ForcedStop::MemoryLimit);
                        kill_child_tree(child_pgid, &mut child)?;
                        break child.wait()?;
                    }
                }
                next_memory_check = Instant::now() + MEMORY_POLL_INTERVAL;
            }
        }

        thread::sleep(Duration::from_millis(2));
    };

    join_stdin(stdin_thread)?;
    let stdout = join_reader(stdout_thread)?;
    let stderr = join_reader(stderr_thread)?;

    let elapsed = started_at.elapsed();
    let exit_code = exit_status.code();
    let signal = exit_signal(exit_status);
    let (status, message) = classify_outcome(
        forced_stop,
        exit_status,
        elapsed,
        request.limits,
        peak_rss_bytes,
    );

    Ok(RunOutcome {
        status,
        stdout,
        stderr,
        elapsed,
        exit_code,
        signal,
        peak_rss_bytes,
        message,
    })
}

fn read_all(mut stream: impl Read) -> io::Result<Vec<u8>> {
    let mut bytes = Vec::new();
    stream.read_to_end(&mut bytes)?;
    Ok(bytes)
}

fn join_stdin(handle: thread::JoinHandle<io::Result<()>>) -> io::Result<()> {
    handle
        .join()
        .map_err(|_| io::Error::other("stdin writer thread panicked"))?
}

fn join_reader(handle: thread::JoinHandle<io::Result<Vec<u8>>>) -> io::Result<Vec<u8>> {
    handle
        .join()
        .map_err(|_| io::Error::other("output reader thread panicked"))?
}

fn classify_outcome(
    forced_stop: Option<ForcedStop>,
    exit_status: ExitStatus,
    elapsed: Duration,
    limits: RunLimits,
    peak_rss_bytes: Option<u64>,
) -> (RunStatus, String) {
    match forced_stop {
        Some(ForcedStop::TimeLimit) => {
            let limit = limits.time_limit.unwrap_or(elapsed);
            return (
                RunStatus::TimeLimit,
                format!(
                    "{} elapsed {}",
                    format_duration(limit),
                    format_duration(elapsed)
                ),
            );
        }
        Some(ForcedStop::MemoryLimit) => {
            let limit = limits.memory_limit_bytes.unwrap_or_default();
            let peak = peak_rss_bytes.unwrap_or_default();
            return (
                RunStatus::MemoryLimit,
                format!("{} peak {}", format_bytes(limit), format_bytes(peak)),
            );
        }
        None => {}
    }

    if exit_status.success() {
        return (
            RunStatus::Ok,
            format!("{} elapsed", format_duration(elapsed)),
        );
    }

    let message = if let Some(code) = exit_status.code() {
        format!("exit code {} after {}", code, format_duration(elapsed))
    } else if let Some(signal) = exit_signal(exit_status) {
        match signal_name(signal) {
            Some(name) => format!(
                "signal {} ({}) after {}",
                signal,
                name,
                format_duration(elapsed)
            ),
            None => format!("signal {} after {}", signal, format_duration(elapsed)),
        }
    } else {
        format!("process failed after {}", format_duration(elapsed))
    };
    (RunStatus::RuntimeError, message)
}

fn format_duration(duration: Duration) -> String {
    format!("{:.3}s", duration.as_secs_f64())
}

fn format_bytes(bytes: u64) -> ByteSize {
    ByteSize(bytes)
}

struct ByteSize(u64);

impl fmt::Display for ByteSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KIB: u64 = 1024;
        const MIB: u64 = 1024 * KIB;
        const GIB: u64 = 1024 * MIB;

        let bytes = self.0;
        if bytes >= GIB {
            write!(f, "{:.2} GiB", bytes as f64 / GIB as f64)
        } else if bytes >= MIB {
            write!(f, "{:.2} MiB", bytes as f64 / MIB as f64)
        } else if bytes >= KIB {
            write!(f, "{:.2} KiB", bytes as f64 / KIB as f64)
        } else {
            write!(f, "{} B", bytes)
        }
    }
}

fn make_child_process_group() -> io::Result<()> {
    let rc = unsafe { libc::setpgid(0, 0) };
    if rc == 0 {
        Ok(())
    } else {
        Err(io::Error::last_os_error())
    }
}

fn set_memory_limit(bytes: u64) -> io::Result<()> {
    #[cfg(not(target_os = "linux"))]
    {
        let _ = bytes;
        return Ok(());
    }

    #[cfg(target_os = "linux")]
    {
        let limit = libc::rlimit {
            rlim_cur: bytes as libc::rlim_t,
            rlim_max: bytes as libc::rlim_t,
        };

        let resource = libc::RLIMIT_AS;

        let rc = unsafe { libc::setrlimit(resource, &limit) };
        if rc == 0 {
            Ok(())
        } else {
            Err(io::Error::last_os_error())
        }
    }
}

fn kill_child_tree(pgid: u32, child: &mut Child) -> io::Result<()> {
    let rc = unsafe { libc::killpg(pgid as libc::pid_t, libc::SIGKILL) };
    if rc == 0 {
        return Ok(());
    }

    let err = io::Error::last_os_error();
    if err.raw_os_error() == Some(libc::ESRCH) {
        return Ok(());
    }

    child.kill()
}

#[cfg(target_os = "linux")]
fn process_group_rss_bytes(pgid: u32) -> io::Result<Option<u64>> {
    let page_size = page_size_bytes();
    let mut total = 0u64;

    for entry in fs::read_dir("/proc")? {
        let entry = entry?;
        let file_name = entry.file_name();
        let Some(file_name) = file_name.to_str() else {
            continue;
        };
        if !file_name.bytes().all(|byte| byte.is_ascii_digit()) {
            continue;
        }

        let stat = match fs::read_to_string(entry.path().join("stat")) {
            Ok(stat) => stat,
            Err(err)
                if matches!(
                    err.kind(),
                    io::ErrorKind::NotFound | io::ErrorKind::PermissionDenied
                ) =>
            {
                continue;
            }
            Err(err) => return Err(err),
        };

        let Some((process_pgid, rss_bytes)) = parse_linux_stat(&stat, page_size) else {
            continue;
        };
        if process_pgid == pgid {
            total = total.saturating_add(rss_bytes);
        }
    }

    if total == 0 {
        Ok(None)
    } else {
        Ok(Some(total))
    }
}

#[cfg(target_os = "macos")]
fn process_group_rss_bytes(pgid: u32) -> io::Result<Option<u64>> {
    let output = Command::new("ps").args(["-axo", "pgid=,rss="]).output()?;
    if !output.status.success() {
        return Ok(None);
    }

    let mut total_kib = 0u64;
    let text = String::from_utf8_lossy(&output.stdout);
    for line in text.lines() {
        let mut parts = line.split_whitespace();
        let Some(raw_pgid) = parts.next() else {
            continue;
        };
        let Some(raw_rss) = parts.next() else {
            continue;
        };
        if raw_pgid.parse::<u32>().ok() == Some(pgid) {
            if let Ok(rss_kib) = raw_rss.parse::<u64>() {
                total_kib = total_kib.saturating_add(rss_kib);
            }
        }
    }

    if total_kib == 0 {
        Ok(None)
    } else {
        Ok(Some(total_kib.saturating_mul(1024)))
    }
}

#[cfg(target_os = "linux")]
fn parse_linux_stat(stat: &str, page_size: u64) -> Option<(u32, u64)> {
    let command_end = stat.rfind(") ")?;
    let rest = &stat[command_end + 2..];
    let mut fields = rest.split_whitespace();

    fields.next()?;
    fields.next()?;
    let pgrp = fields.next()?.parse::<u32>().ok()?;

    for _ in 0..18 {
        fields.next()?;
    }

    let rss_pages = fields.next()?.parse::<i64>().ok()?;
    let rss_pages = u64::try_from(rss_pages).ok()?;
    Some((pgrp, rss_pages.saturating_mul(page_size)))
}

#[cfg(target_os = "linux")]
fn page_size_bytes() -> u64 {
    let page_size = unsafe { libc::sysconf(libc::_SC_PAGESIZE) };
    if page_size > 0 {
        page_size as u64
    } else {
        4096
    }
}

fn exit_signal(status: ExitStatus) -> Option<i32> {
    status.signal()
}

fn signal_name(signal: i32) -> Option<&'static str> {
    match signal {
        libc::SIGABRT => Some("SIGABRT"),
        libc::SIGBUS => Some("SIGBUS"),
        libc::SIGFPE => Some("SIGFPE"),
        libc::SIGILL => Some("SIGILL"),
        libc::SIGKILL => Some("SIGKILL"),
        libc::SIGPIPE => Some("SIGPIPE"),
        libc::SIGSEGV => Some("SIGSEGV"),
        libc::SIGTERM => Some("SIGTERM"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn captures_stdout_from_stdin() {
        let outcome = run_bash_command(RunRequest {
            command: "cat".to_owned(),
            stdin: b"hello\n".to_vec(),
            limits: RunLimits {
                time_limit: Some(Duration::from_secs(1)),
                memory_limit_bytes: Some(128 * 1024 * 1024),
            },
        })
        .unwrap();

        assert_eq!(outcome.status, RunStatus::Ok);
        assert_eq!(outcome.stdout, b"hello\n");
    }

    #[test]
    fn reports_time_limit() {
        let outcome = run_bash_command(RunRequest {
            command: "sleep 1".to_owned(),
            stdin: Vec::new(),
            limits: RunLimits {
                time_limit: Some(Duration::from_millis(30)),
                memory_limit_bytes: Some(128 * 1024 * 1024),
            },
        })
        .unwrap();

        assert_eq!(outcome.status, RunStatus::TimeLimit);
    }

    #[test]
    #[cfg(target_os = "macos")]
    fn reports_memory_limit() {
        let outcome = run_bash_command(RunRequest {
            command: "buf=$(yes x | head -c 67108864); sleep 1".to_owned(),
            stdin: Vec::new(),
            limits: RunLimits {
                time_limit: Some(Duration::from_secs(2)),
                memory_limit_bytes: Some(16 * 1024 * 1024),
            },
        })
        .unwrap();

        assert_eq!(outcome.status, RunStatus::MemoryLimit);
        assert!(outcome.peak_rss_bytes.unwrap_or_default() > 16 * 1024 * 1024);
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn parses_linux_proc_stat() {
        let stat = "123 (name with space) S 1 456 456 0 -1 4194560 100 0 0 0 1 2 0 0 20 0 1 0 10 4096 7 0 0 0";
        let parsed = parse_linux_stat(stat, 4096).unwrap();

        assert_eq!(parsed, (456, 7 * 4096));
    }
}
