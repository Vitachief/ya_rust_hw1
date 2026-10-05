use std::fmt;

/// Типы ошибок, возникающих при обработке транзакций.
#[derive(Debug)]
pub enum TransactionError {
    /// Ошибка ввода-вывода.
    Io(std::io::Error),
    /// Неожиданный конец файла или потока данных.
    UnexpectedEof,
    /// Ошибка парсинга данных с описанием причины.
    ParseError(String),
}

impl fmt::Display for TransactionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TransactionError::Io(e) => write!(f, "Ошибка ввода-вывода: {}", e),
            TransactionError::UnexpectedEof => write!(f, "Неожиданный конец данных"),
            TransactionError::ParseError(msg) => write!(f, "Ошибка парсинга: {}", msg),
        }
    }
}

impl std::error::Error for TransactionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            TransactionError::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for TransactionError {
    fn from(err: std::io::Error) -> Self {
        if err.kind() == std::io::ErrorKind::UnexpectedEof {
            TransactionError::UnexpectedEof
        } else {
            TransactionError::Io(err)
        }
    }
}