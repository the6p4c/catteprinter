pub mod command;
pub mod commands;
pub mod printer;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Bluetooth(btleplug::Error),
    PrinterNotFound,
}

impl From<btleplug::Error> for Error {
    fn from(e: btleplug::Error) -> Error {
        Error::Bluetooth(e)
    }
}
