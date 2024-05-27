pub enum Error {
    IOError(std::io::Error),
    Exit,
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IOError(value)
    }
}
