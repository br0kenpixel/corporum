/// Convert an Error to an std::io::Error.
macro_rules! into_stderr {
    ($code: expr) => {
        $code.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))
    };
}

/* /// Creates an `std::io::Error` with a custom message.
macro_rules! stderr_with_message {
    ($message: literal) => {
        std::io::Error::new(std::io::ErrorKind::Other, $message)
    };
} */

/// Opens a file for writing, as well as creating it if it doesn’t exist.
macro_rules! create_file {
    ($path: expr) => {
        std::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open($path)
    };
}

pub(crate) use create_file;
pub(crate) use into_stderr;
//pub(crate) use stderr_with_message;
