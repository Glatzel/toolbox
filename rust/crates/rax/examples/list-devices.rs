use clerk::LogLevel;
use rax::device::{DeviceFilter, list_devices};

fn main() -> miette::Result<()> {
    clerk::init_log_with_level(LogLevel::TRACE);
    let devices = list_devices(DeviceFilter::all)?;
    for d in devices {
        println!("{d:#?}");
    }
    Ok(())
}
