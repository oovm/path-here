pub use errors::XResult;

#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "macos")]
pub mod macos;
pub mod utils;
#[cfg(target_os = "windows")]
pub mod windows;

mod errors;

pub struct Runner {}

fn main() -> XResult {
    Runner {}.run()
}
