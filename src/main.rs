use std::env;
use std::fs::File;
use std::io::{stdin, stdout};
use ypbank_tools::{read_transactions, write_transactions, Format};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 || args.len() > 4 {
        eprintln!("Использование: converter <входной_формат> <выходной_формат> [входной_файл]");
        eprintln!("Форматы: csv, text, bin");
        eprintln!("Если файл не указан, читается из stdin, результат пишется в stdout.");
        std::process::exit(1);
    }

    let in_format: Format = match args[1].parse() {
        Ok(fmt) => fmt,
        Err(e) => {
            eprintln!("Ошибка входного формата: {}", e);
            std::process::exit(1);
        }
    };

    let out_format: Format = match args[2].parse() {
        Ok(fmt) => fmt,
        Err(e) => {
            eprintln!("Ошибка выходного формата: {}", e);
            std::process::exit(1);
        }
    };

    // Чтение
    let transactions = if args.len() == 4 {
        let file_path = &args[3];
        let file = match File::open(file_path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Не удалось открыть файл '{}': {}", file_path, e);
                std::process::exit(1);
            }
        };
        match read_transactions(file, in_format) {
            Ok(txs) => txs,
            Err(e) => {
                eprintln!("Ошибка чтения файла: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        let stdin_lock = stdin().lock();
        match read_transactions(stdin_lock, in_format) {
            Ok(txs) => txs,
            Err(e) => {
                eprintln!("Ошибка чтения из stdin: {}", e);
                std::process::exit(1);
            }
        }
    };

    // Запись
    let stdout_lock = stdout().lock();
    if let Err(e) = write_transactions(stdout_lock, &transactions, out_format) {
        eprintln!("Ошибка записи: {}", e);
        std::process::exit(1);
    }
}
