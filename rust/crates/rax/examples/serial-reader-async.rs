use clerk::LogLevel;
use miette::IntoDiagnostic;
use rax::io::{AsyncIRaxReader, AsyncRaxReader};
use tokio::io::BufReader;
use tokio_serial::SerialPortBuilderExt;
#[tokio::main]
async fn main() -> miette::Result<()> {
    clerk::init_log_with_level(LogLevel::TRACE);
    let port = "COM5";
    let serial = tokio_serial::new(port, 9600)
        .open_native_async()
        .into_diagnostic()?;
    let mut reader = AsyncRaxReader::new(BufReader::new(serial));

    while let Some(sentence) = reader.read_line().await? {
        println!("{sentence}")
    }
    Ok(())
}
