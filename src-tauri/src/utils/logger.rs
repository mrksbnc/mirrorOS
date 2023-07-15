use flexi_logger::{FileSpec, Logger};
use log::info;

pub fn init_logger() -> Result<(), flexi_logger::FlexiLoggerError> {
    Logger::try_with_str("info")?
        .append()
        .print_message()
        .start()?;

    info!("Logger initialized");
    Ok(())
}
