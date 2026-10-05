/// Тип финансовой операции.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransactionKind {
    Income,
    Expense,
}

/// Единое внутреннее представление транзакции.
#[derive(Debug, Clone, PartialEq)]
pub struct Transaction {
    pub date: String,
    pub category: String,
    pub kind: TransactionKind,
    pub amount: i64,
}

/// Поддерживаемые форматы данных.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    Csv,
    Text,
    Bin,
}

impl std::str::FromStr for Format {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "csv" => Ok(Format::Csv),
            "text" => Ok(Format::Text),
            "bin" => Ok(Format::Bin),
            _ => Err(format!("Неизвестный формат: {}", s)),
        }
    }
}