//! Библиотека для чтения, записи и сравнения финансовых транзакций.
//! Поддерживает форматы CSV, Text и Bin через абстракции std::io::Read и std::io::Write.

pub mod error;
pub mod model;
pub mod csv_format;
pub mod text_format;
pub mod bin_format;

use std::io::{Read, Write};

pub use error::TransactionError;
pub use model::{Format, Transaction, TransactionKind};

/// Читает список транзакций из любого источника, реализующего Read.
pub fn read_transactions<R: Read>(reader: R, format: Format) -> Result<Vec<Transaction>, TransactionError> {
    match format {
        Format::Csv => csv_format::read_csv(reader),
        Format::Text => text_format::read_text(reader),
        Format::Bin => bin_format::read_bin(reader),
    }
}

/// Записывает список транзакций в любой приемник, реализующий Write.
pub fn write_transactions<W: Write>(writer: W, transactions: &[Transaction], format: Format) -> Result<(), TransactionError> {
    match format {
        Format::Csv => csv_format::write_csv(writer, transactions),
        Format::Text => text_format::write_text(writer, transactions),
        Format::Bin => bin_format::write_bin(writer, transactions),
    }
}

/// Сравнивает два списка транзакций поэлементно.
/// Возвращает Ok(()) если списки идентичны, или Err с описанием первого различия.
pub fn compare_transactions(txs1: &[Transaction], txs2: &[Transaction]) -> Result<(), String> {
    if txs1.len() != txs2.len() {
        return Err(format!(
            "Разное количество транзакций: {} и {}",
            txs1.len(),
            txs2.len()
        ));
    }

    for (i, (t1, t2)) in txs1.iter().zip(txs2.iter()).enumerate() {
        let pos = i + 1;
        if t1.date != t2.date {
            return Err(format!("Транзакции отличаются на позиции {}: date: {} != {}", pos, t1.date, t2.date));
        }
        if t1.category != t2.category {
            return Err(format!("Транзакции отличаются на позиции {}: category: {} != {}", pos, t1.category, t2.category));
        }
        if t1.kind != t2.kind {
            return Err(format!("Транзакции отличаются на позиции {}: kind: {:?} != {:?}", pos, t1.kind, t2.kind));
        }
        // Сравнение f64 с учетом эпсилон-погрешности
        if t1.amount != t2.amount{
            return Err(format!("Транзакции отличаются на позиции {}: amount: {} != {}", pos, t1.amount, t2.amount));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_csv_parse_success() {
        let data = b"date,category,kind,amount\n2026-04-01,salary,income,120000\n";
        let txs = read_transactions(Cursor::new(data), Format::Csv).unwrap();
        assert_eq!(txs.len(), 1);
        assert_eq!(txs[0].amount, 120000);
        assert_eq!(txs[0].category, "salary");
        assert_eq!(txs[0].kind, TransactionKind::Income);
        assert_eq!(txs[0].date, "2026-04-01");
    }

    #[test]
    fn test_csv_parse_error_no_amount() {
        let data = b"date,category,kind,amount\n2026-04-01,salary,income\n"; // missing amount
        assert!(read_transactions(Cursor::new(data), Format::Csv).is_err());
    }

    #[test]
    fn test_csv_parse_error_wrong_kind() {
        let data = b"date,category,kind,amount\n2026-04-01,salary,wtf,4\n"; // missing amount
        assert!(read_transactions(Cursor::new(data), Format::Csv).is_err());
    }

    #[test]
    fn test_text_parse_success() {
        let data = b"2026-04-01;salary;income;120000\n";
        let txs = read_transactions(Cursor::new(data), Format::Text).unwrap();
        assert_eq!(txs.len(), 1);
        assert_eq!(txs[0].category, "salary");
    }

    #[test]
    fn test_text_parse_error() {
        let data = b"2026-04-01;salary;invalid_kind;120000\n";
        assert!(read_transactions(Cursor::new(data), Format::Text).is_err());
    }

    #[test]
    fn test_bin_parse_success() {
        let txs_in = vec![Transaction {
            date: "2026-04-01".into(),
            category: "salary".into(),
            kind: TransactionKind::Income,
            amount: 120000,
        }];
        let mut buffer = Vec::new();
        write_transactions(&mut buffer, &txs_in, Format::Bin).unwrap();
        
        let txs_out = read_transactions(Cursor::new(buffer), Format::Bin).unwrap();
        assert_eq!(txs_out.len(), 1);
        assert_eq!(txs_out[0].amount, 120000);
    }

    #[test]
    fn test_bin_parse_error() {
        let data = b"\x01\x00\x00\x00"; // count = 1, but no data follows
        assert!(read_transactions(Cursor::new(data), Format::Bin).is_err());
    }

    #[test]
    fn test_conversion() {
        let input = b"2026-04-01;rent;expense;40000\n";
        let txs = read_transactions(Cursor::new(input), Format::Text).unwrap();
        
        let mut output = Vec::new();
        write_transactions(&mut output, &txs, Format::Csv).unwrap();
        
        let expected = b"date,category,kind,amount\n2026-04-01,rent,expense,40000\n";
        assert_eq!(output, expected);
    }

    #[test]
    fn test_comparer_match() {
        let txs1 = vec![Transaction {
            date: "2026-04-01".into(),
            category: "salary".into(),
            kind: TransactionKind::Income,
            amount: 120000,
        }];
        let txs2 = txs1.clone();
        assert!(compare_transactions(&txs1, &txs2).is_ok());
    }

    #[test]
    fn test_comparer_mismatch() {
        let txs1 = vec![Transaction {
            date: "2026-04-01".into(),
            category: "salary".into(),
            kind: TransactionKind::Income,
            amount: 6500,
        }];
        let txs2 = vec![Transaction {
            date: "2026-04-01".into(),
            category: "salary".into(),
            kind: TransactionKind::Income,
            amount: 6504,
        }];
        let err = compare_transactions(&txs1, &txs2).unwrap_err();
        assert!(err.contains("позиции 1") && err.contains("6500") && err.contains("6504"));
    }
}