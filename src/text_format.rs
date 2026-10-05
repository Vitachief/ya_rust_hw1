use std::io::{BufRead, BufReader, Read, Write};
use crate::error::TransactionError;
use crate::model::{Transaction, TransactionKind};

/// Читает транзакции из текстового формата (разделитель ';').
pub fn read_text<R: Read>(reader: R) -> Result<Vec<Transaction>, TransactionError> {
    let buf_reader = BufReader::new(reader);
    let mut transactions = Vec::new();

    for (line_num, line_res) in buf_reader.lines().enumerate() {
        let line = line_res?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split(';').collect();
        if parts.len() != 4 {
            return Err(TransactionError::ParseError(format!(
                "Ожидается 4 поля в text, найдено {} (строка {})",
                parts.len(),
                line_num + 1
            )));
        }

        let kind = parse_kind(parts[2])?;
        let amount = parts[3].trim().parse::<i64>().map_err(|_| {
            TransactionError::ParseError(format!("Неверная сумма в строке {}", line_num + 1))
        })?;

        transactions.push(Transaction {
            date: parts[0].trim().to_string(),
            category: parts[1].trim().to_string(),
            kind,
            amount,
        });
    }

    Ok(transactions)
}

/// Записывает транзакции в текстовый формат.
pub fn write_text<W: Write>(mut writer: W, transactions: &[Transaction]) -> Result<(), TransactionError> {
    for tx in transactions {
        let kind_str = match tx.kind {
            TransactionKind::Income => "income",
            TransactionKind::Expense => "expense",
        };
        writeln!(writer, "{};{};{};{}", tx.date, tx.category, kind_str, tx.amount)?;
    }
    Ok(())
}

fn parse_kind(s: &str) -> Result<TransactionKind, TransactionError> {
    match s.trim().to_lowercase().as_str() {
        "income" => Ok(TransactionKind::Income),
        "expense" => Ok(TransactionKind::Expense),
        _ => Err(TransactionError::ParseError(format!("Неверный тип операции: {}", s))),
    }
}