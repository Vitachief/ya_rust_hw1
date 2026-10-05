# YPBank Tools

Инструменты для конвертации и сравнения финансовых транзакций.

## Форматы данных
1. **csv**: `date,category,kind,amount` (первая строка — заголовок).
2. **text**: `date;category;kind;amount` (одна операция на строку).
3. **bin**: Бинарный формат.
   - `u32` (Little Endian): количество транзакций.
   - Для каждой транзакции:
     - `u16` (LE): длина строки `date`.
     - `bytes`: строка `date` (UTF-8).
     - `u16` (LE): длина строки `category`.
     - `bytes`: строка `category` (UTF-8).
     - `u8`: тип операции (0 = income, 1 = expense).
     - `f64` (LE): сумма операции.

## Использование

### Конвертер
Читает из файла или stdin, конвертирует и выводит в stdout.
```bash
# Из файла в файл
cargo run --bin converter -- csv text input.csv > output.txt

# Из stdin в stdout
echo "2026-04-01;salary;income;120000" | cargo run --bin converter -- text csv