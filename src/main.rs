use windows::core::Result;
use windows::Win32::Foundation::BOOL;
use windows::Win32::System::Threading::{GetCurrentProcess, IsWow64Process};

fn main() -> Result<()> {
    let mut wow64process = BOOL::default();
    unsafe {
        IsWow64Process(GetCurrentProcess(), &mut wow64process)?;
    }
    println!("{}", wow64process.as_bool());
    Ok(())
}
