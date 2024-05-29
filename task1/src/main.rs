use std::io::{stdin, stdout, Write};

fn main() {
    let mut s = String::new();

    print!("Введите кол-во сторон у фигуры: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut s).unwrap();
    
    let side_count = s
        .trim()
        .parse::<u32>().expect("Было введено не корректное число!");

    match side_count {
        3 => println!("Трёхугольник"),
        4 => println!("Четырёхугольник"),
        5 => println!("Пятиугольник"),
        6 => println!("Шестиугольник"),
        7 => println!("Семиугольник"),
        8 => println!("Восьмиугольник"),
        9 => println!("Девятиугольник"),
        10 => println!("Десятиугольник"),
        _ => println!("Такого не бывает!")
    }
}
