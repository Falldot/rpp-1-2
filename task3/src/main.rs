use std::io::{stdin, stdout, Write};

fn shift_char(c: char, shift: i32) -> char {
    let mut new_char = c as u8;
    let c_shift = if shift.is_negative() { 24 - shift } else { shift };

    if new_char.is_ascii_uppercase() {
      new_char = (new_char - b'A' + c_shift as u8 + 26) % 26 + b'A';
    } else if new_char.is_ascii_lowercase() {
      new_char = (new_char - b'a' + c_shift as u8 + 26) % 26 + b'a';
    }

    return new_char as char;
}

fn main() {
    let mut text: String = String::new();
    let mut shift_text: String = String::new();

    print!("Введите что нужно защифровать: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut text).unwrap();

    print!("Введите кол-во симвлов сдвига: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut shift_text).unwrap();

    let shift_count = shift_text
        .trim()
        .parse::<i32>().expect("Было введено не корректное число!");

    let encrypted_text: String = text
        .trim()
        .chars()
        .map(|v| shift_char(v, shift_count))
        .collect();

    println!("{}", encrypted_text)
}
