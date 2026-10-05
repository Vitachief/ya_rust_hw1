use std::io::{Read, Write};
use crate::error::TransactionError;
use crate::model::{Transaction, TransactionKind};

/// Читает транзакции из бинарного формата.
pub fn read_bin<R: Read>(mut reader: R) -> Result<Vec<Transaction>, TransactionError> {
    let mut count_buf = [0u8; 4];
    reader.read_exact(&mut count_buf)?;
    let count = u32::from_le_bytes(count_buf);
    
    let mut transactions = Vec::with_capacity(count as usize);

    for _ in 0..count {
        let mut len_buf = [0u8; 2];
        
        reader.read_exact(&mut len_buf)?;
        let date_len = u16::from_le_bytes(len_buf) as usize;
        let mut date_buf = vec![0u8; date_len];
        reader.read_exact(&mut date_buf)?;
        let date = String::from_utf8(date_buf)
            .map_err(|_| TransactionError::ParseError("Неверный UTF-8 в дате".into()))?;

        reader.read_exact(&mut len_buf)?;
        let cat_len = u16::from_le_bytes(len_buf) as usize;
        let mut cat_buf = vec![0u8; cat_len];
        reader.read_exact(&mut cat_buf)?;
        let category = String::from_utf8(cat_buf)
            .map_err(|_| TransactionError::ParseError("Неверный UTF-8 в категории".into()))?;

        let mut kind_buf = [0u8; 1];
        reader.read_exact(&mut kind_buf)?;
        let kind = match kind_buf[0] {
            0 => TransactionKind::Income,
            1 => TransactionKind::Expense,
            _ => return Err(TransactionError::ParseError("Неверный код типа операции".into())),
        };

        let mut amount_buf = [0u8; 8];
        reader.read_exact(&mut amount_buf)?;
        let amount = i64::from_le_bytes(amount_buf);

        transactions.push(Transaction { date, category, kind, amount });
    }

    Ok(transactions)
}

/// Записывает транзакции в бинарный формат.
pub fn write_bin<W: Write>(mut writer: W, transactions: &[Transaction]) -> Result<(), TransactionError> {
    let count = transactions.len() as u32;
    writer.write_all(&count.to_le_bytes())?;

    for tx in transactions {
        let date_bytes = tx.date.as_bytes();
        writer.write_all(&(date_bytes.len() as u16).to_le_bytes())?;
        writer.write_all(date_bytes)?;

        let cat_bytes = tx.category.as_bytes();
        writer.write_all(&(cat_bytes.len() as u16).to_le_bytes())?;
        writer.write_all(cat_bytes)?;

        let kind_byte = match tx.kind {
            TransactionKind::Income => 0u8,
            TransactionKind::Expense => 1u8,
        };
        writer.write_all(&[kind_byte])?;

        writer.write_all(&tx.amount.to_le_bytes())?;
    }

    Ok(())
}