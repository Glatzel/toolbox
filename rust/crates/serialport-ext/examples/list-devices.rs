use clerk::Level;
use serialport_ext::{DeviceFilter, list_devices};

fn main() -> mischief::Result<()> {
    clerk::init_log_with_level(LevelFilter::TRACE);
    let devices = list_devices(DeviceFilter::all)?;
    for d in devices {
        println!("{d:#?}");
    }
    Ok(())
}
