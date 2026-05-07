use std::fs;
use std::io;
use std::path::PathBuf;
use std::process;
use std::time::Duration;

use clap::Parser;
use judge::checker::{CheckVerdict, check};
use judge::runner::{RunLimits, RunRequest, RunStatus, run_bash_command};
use judge::test_data::{TestCase, discover_tests, read_test_file};

const ANSI_RESET: &str = "\x1b[0m";
const ANSI_GREEN: &str = "\x1b[32m";
const ANSI_YELLOW: &str = "\x1b[33m";
const ANSI_MAGENTA: &str = "\x1b[35m";
const ANSI_RED: &str = "\x1b[31m";

#[derive(Debug, Parser)]
#[command(author, version, about = "Local contest judge runner")]
struct Args {
    #[arg(short = 's', long = "solution")]
    solution: String,

    #[arg(long = "test-data", default_value = "test_data")]
    test_data_dir: PathBuf,

    #[arg(long = "stdin-file")]
    stdin_file: Option<PathBuf>,

    #[arg(long = "time-limit", default_value_t = 1.0)]
    time_limit_seconds: f64,

    #[arg(long = "memory-limit-mb", default_value_t = 512)]
    memory_limit_mb: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TestVerdict {
    Accepted,
    WrongAnswer,
    TimeLimit,
    MemoryLimit,
    RuntimeError,
    CheckerFailure,
}

impl TestVerdict {
    fn label(self) -> &'static str {
        match self {
            TestVerdict::Accepted => "OK",
            TestVerdict::WrongAnswer => "WA",
            TestVerdict::TimeLimit => "TL",
            TestVerdict::MemoryLimit => "ML",
            TestVerdict::RuntimeError => "RE",
            TestVerdict::CheckerFailure => "FL",
        }
    }

    fn color(self) -> &'static str {
        match self {
            TestVerdict::Accepted => ANSI_GREEN,
            TestVerdict::WrongAnswer => ANSI_RED,
            TestVerdict::TimeLimit => ANSI_YELLOW,
            TestVerdict::MemoryLimit => ANSI_MAGENTA,
            TestVerdict::RuntimeError => ANSI_RED,
            TestVerdict::CheckerFailure => ANSI_RED,
        }
    }

    fn colored_label(self) -> String {
        format!("{}{}{}", self.color(), self.label(), ANSI_RESET)
    }

    fn is_accepted(self) -> bool {
        self == TestVerdict::Accepted
    }
}

struct TestReport {
    name: String,
    verdict: TestVerdict,
    elapsed: Duration,
    peak_rss_bytes: Option<u64>,
    message: String,
}

fn main() {
    let args = Args::parse();
    let limits = RunLimits {
        time_limit: Some(Duration::from_secs_f64(args.time_limit_seconds)),
        memory_limit_bytes: Some(args.memory_limit_mb * 1024 * 1024),
    };

    if let Some(stdin_file) = args.stdin_file {
        run_single_input(&args.solution, &stdin_file, limits);
        return;
    }

    let tests = discover_tests(&args.test_data_dir).unwrap_or_else(|err| {
        eprintln!(
            "cannot discover tests in '{}': {}",
            args.test_data_dir.display(),
            err
        );
        process::exit(2);
    });

    if tests.is_empty() {
        eprintln!("no tests found in '{}'", args.test_data_dir.display());
        process::exit(2);
    }

    println!(
        "solution: {}\ntests: {} ({})\nlimits: time {}, memory {}",
        args.solution,
        args.test_data_dir.display(),
        tests.len(),
        format_duration(limits.time_limit.unwrap()),
        format_bytes(limits.memory_limit_bytes.unwrap())
    );

    let time_width = time_ms_integer_width(limits.time_limit);
    let mut reports = Vec::with_capacity(tests.len());
    for test in &tests {
        let report = run_test(test, &args.solution, limits).unwrap_or_else(|err| TestReport {
            name: test.name.clone(),
            verdict: TestVerdict::CheckerFailure,
            elapsed: Duration::ZERO,
            peak_rss_bytes: None,
            message: err.to_string(),
        });
        print_report(&report, time_width);
        reports.push(report);
    }

    let accepted = reports
        .iter()
        .filter(|report| report.verdict.is_accepted())
        .count();
    let max_elapsed = reports
        .iter()
        .map(|report| report.elapsed)
        .max()
        .unwrap_or(Duration::ZERO);
    let max_rss = reports
        .iter()
        .filter_map(|report| report.peak_rss_bytes)
        .max();

    println!(
        "\nsummary: {}/{} accepted, max time {}, max memory {}",
        accepted,
        reports.len(),
        format_duration(max_elapsed),
        max_rss
            .map(format_bytes)
            .unwrap_or_else(|| "n/a".to_owned())
    );

    if accepted == reports.len() {
        process::exit(0);
    }
    process::exit(1);
}

fn run_single_input(solution: &str, stdin_file: &PathBuf, limits: RunLimits) {
    let stdin = fs::read(stdin_file).unwrap_or_else(|err| {
        eprintln!("cannot read stdin file '{}': {}", stdin_file.display(), err);
        process::exit(2);
    });

    let outcome = run_bash_command(RunRequest {
        command: solution.to_owned(),
        stdin,
        limits,
    })
    .unwrap_or_else(|err| {
        eprintln!("cannot run solution: {}", err);
        process::exit(2);
    });

    println!("{}", outcome.colored_summary());
    if !outcome.stdout.is_empty() {
        println!("--- stdout ---");
        print!("{}", String::from_utf8_lossy(&outcome.stdout));
    }
    if !outcome.stderr.is_empty() {
        println!("--- stderr ---");
        eprint!("{}", String::from_utf8_lossy(&outcome.stderr));
    }

    process::exit(match outcome.status {
        RunStatus::Ok => 0,
        RunStatus::TimeLimit | RunStatus::MemoryLimit | RunStatus::RuntimeError => 1,
    });
}

fn run_test(test: &TestCase, solution: &str, limits: RunLimits) -> io::Result<TestReport> {
    let input = read_test_file(&test.input_path)?;
    let answer = read_test_file(&test.answer_path)?;

    let outcome = run_bash_command(RunRequest {
        command: solution.to_owned(),
        stdin: input.clone(),
        limits,
    })?;

    let mut report = TestReport {
        name: test.name.clone(),
        verdict: map_run_status(outcome.status),
        elapsed: outcome.elapsed,
        peak_rss_bytes: outcome.peak_rss_bytes,
        message: outcome.message.clone(),
    };

    if outcome.status != RunStatus::Ok {
        return Ok(report);
    }

    let input_text = std::str::from_utf8(&input).map_err(|err| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "input '{}' is not UTF-8: {}",
                test.input_path.display(),
                err
            ),
        )
    })?;
    let answer_text = std::str::from_utf8(&answer).map_err(|err| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "answer '{}' is not UTF-8: {}",
                test.answer_path.display(),
                err
            ),
        )
    })?;
    let output_text = match std::str::from_utf8(&outcome.stdout) {
        Ok(text) => text,
        Err(err) => {
            report.verdict = TestVerdict::WrongAnswer;
            report.message = format!("output is not UTF-8: {}", err);
            return Ok(report);
        }
    };

    let check_outcome = check(input_text, output_text, answer_text);
    report.verdict = match check_outcome.verdict {
        CheckVerdict::Accepted => TestVerdict::Accepted,
        CheckVerdict::WrongAnswer => TestVerdict::WrongAnswer,
        CheckVerdict::CheckerFailure => TestVerdict::CheckerFailure,
    };
    report.message = check_outcome.message;

    Ok(report)
}

fn map_run_status(status: RunStatus) -> TestVerdict {
    match status {
        RunStatus::Ok => TestVerdict::Accepted,
        RunStatus::TimeLimit => TestVerdict::TimeLimit,
        RunStatus::MemoryLimit => TestVerdict::MemoryLimit,
        RunStatus::RuntimeError => TestVerdict::RuntimeError,
    }
}

fn print_report(report: &TestReport, time_width: usize) {
    let memory = report
        .peak_rss_bytes
        .map(format_bytes)
        .unwrap_or_else(|| "n/a".to_owned());
    println!(
        "{} {:>3} {} {:>10}  {}",
        report.verdict.colored_label(),
        report.name,
        format_duration_ms(report.elapsed, time_width),
        memory,
        report.message
    );
}

fn format_duration(duration: Duration) -> String {
    format!("{:.3}s", duration.as_secs_f64())
}

fn time_ms_integer_width(time_limit: Option<Duration>) -> usize {
    let limit_ms = time_limit
        .map(|duration| duration.as_millis())
        .unwrap_or(1000)
        .max(1);
    limit_ms.to_string().len().max(3)
}

fn format_duration_ms(duration: Duration, integer_width: usize) -> String {
    let micros = duration.as_micros();
    let whole_ms = micros / 1000;
    let fractional_micros = micros % 1000;
    format!(
        "{:>width$},{:03}ms",
        whole_ms,
        fractional_micros,
        width = integer_width
    )
}

fn format_bytes(bytes: u64) -> String {
    const KIB: u64 = 1024;
    const MIB: u64 = 1024 * KIB;
    const GIB: u64 = 1024 * MIB;

    if bytes >= GIB {
        format!("{:.2}GiB", bytes as f64 / GIB as f64)
    } else if bytes >= MIB {
        format!("{:.2}MiB", bytes as f64 / MIB as f64)
    } else if bytes >= KIB {
        format!("{:.2}KiB", bytes as f64 / KIB as f64)
    } else {
        format!("{}B", bytes)
    }
}
