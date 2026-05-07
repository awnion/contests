#[cfg(not(any(target_os = "linux", target_os = "macos")))]
compile_error!("judge only supports Linux and macOS");

pub mod runner;

pub mod checker;
pub mod test_data;
