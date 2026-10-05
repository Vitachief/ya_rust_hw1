use std::env;
use std::fs::File;
use ypbank_tools::{compare_transactions, read_transactions, Format};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        eprintln!("Использование: comparer <формат1> <файл1> <формат2> <файл2>");
        eprintln!("Форматы: csv, text, bin");
        std::process::exit(1);
    }

    let fmt1: Format = match args[1].parse() {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Ошибка формата 1: {}", e);
            std::process::exit(1);
        }
    };

    let fmt2: Format = match args[3].parse() {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Ошибка формата 2: {}", e);
            std::process::exit(1);
        }
    };

    let file1_path = &args[2];
    let file2_path = &args[4];

    let file1 = match File::open(file1_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Не удалось открыть '{}': {}", file1_path, e);
            std::process::exit(1);
        }
    };

    let file2 = match File::open(file2_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Не удалось открыть '{}': {}", file2_path, e);
            std::process::exit(1);
        }
    };

    let txs1 = match read_transactions(file1, fmt1) {
        Ok(txs) => txs,
        Err(e) => {
            eprintln!("Ошибка парсинга '{}': {}", file1_path, e);
            std::process::exit(1);
        }
    };

    let txs2 = match read_transactions(file2, fmt2) {
        Ok(txs) => txs,
        Err(e) => {
            eprintln!("Ошибка парсинга '{}': {}", file2_path, e);
            std::process::exit(1);
        }
    };

    match compare_transactions(&txs1, &txs2) {
        Ok(_) => println!("Файлы идентичны после парсинга."),
        Err(diff) => {
            eprintln!("Несовпадение: {}", diff);
            std::process::exit(1);
        }
    }
}
