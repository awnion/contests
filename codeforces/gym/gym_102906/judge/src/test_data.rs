use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Clone)]
pub struct TestCase {
    pub name: String,
    pub input_path: PathBuf,
    pub answer_path: PathBuf,
}

pub fn discover_tests(test_data_dir: &Path) -> io::Result<Vec<TestCase>> {
    let mut tests = Vec::new();

    for entry in fs::read_dir(test_data_dir)? {
        let entry = entry?;
        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        let Some(file_name) = path.file_name().and_then(|file_name| file_name.to_str()) else {
            continue;
        };
        let Some((name, compression_ext)) = split_test_file_name(file_name, ".in") else {
            continue;
        };

        let answer_path = test_data_dir.join(format!("{}.out{}", name, compression_ext));
        if !answer_path.exists() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!(
                    "missing answer for test '{}': expected '{}'",
                    name,
                    answer_path.display()
                ),
            ));
        }

        tests.push(TestCase {
            name: name.to_owned(),
            input_path: path,
            answer_path,
        });
    }

    tests.sort_by(|left, right| left.name.cmp(&right.name));
    Ok(tests)
}

pub fn read_test_file(path: &Path) -> io::Result<Vec<u8>> {
    match compression_extension(path) {
        None => fs::read(path),
        Some("lzma") | Some("xz") => read_from_decompressor("xz", &["-dc"], path),
        Some("gz") => read_from_decompressor("gzip", &["-dc"], path),
        Some("zst") => read_from_decompressor("zstd", &["-dcq"], path),
        Some(ext) => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("unsupported compressed test file extension '.{}'", ext),
        )),
    }
}

fn split_test_file_name<'a>(file_name: &'a str, marker: &str) -> Option<(&'a str, &'a str)> {
    for compression_ext in ["", ".lzma", ".xz", ".gz", ".zst"] {
        let suffix = format!("{}{}", marker, compression_ext);
        if let Some(name) = file_name.strip_suffix(&suffix) {
            return Some((name, compression_ext));
        }
    }
    None
}

fn compression_extension(path: &Path) -> Option<&str> {
    let file_name = path.file_name()?.to_str()?;
    if file_name.ends_with(".lzma") {
        Some("lzma")
    } else if file_name.ends_with(".xz") {
        Some("xz")
    } else if file_name.ends_with(".gz") {
        Some("gz")
    } else if file_name.ends_with(".zst") {
        Some("zst")
    } else {
        None
    }
}

fn read_from_decompressor(program: &str, args: &[&str], path: &Path) -> io::Result<Vec<u8>> {
    let output = Command::new(program).args(args).arg(path).output()?;
    if output.status.success() {
        Ok(output.stdout)
    } else {
        Err(io::Error::other(format!(
            "{} failed for '{}': {}",
            program,
            path.display(),
            String::from_utf8_lossy(&output.stderr).trim()
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_lzma_input_name() {
        assert_eq!(
            split_test_file_name("01.in.lzma", ".in"),
            Some(("01", ".lzma"))
        );
    }
}
