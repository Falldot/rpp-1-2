use std::io::{stdin, stdout, Write};


fn main() {    
    let mut s = String::new();

    let mut data: Vec<(&str, u32)> = vec![
        ("Отбойный молоток", 130),
        ("Газонокосилка", 106),
        ("Будильник", 70),
        ("Шелест травы", 40),
    ];

    print!("Введите уровень шума в децибелах: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut s).unwrap();

    let noise_level = s
        .trim()
        .parse::<u32>().expect("Было введено не корректное число!");

    data 
        .sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());

    let max_pos = data.iter().position(|(_, v)| *v > noise_level);

    match max_pos {
        Some(index) => match index {
            0 => {
                let (k, _) = data[0];
                println!("Уровень децибел меньше чем у `{}`", k)
            },
            index => {
                let (k1, _) = data[index];
                let (k2, _) = data[index - 1];
                println!("Уровень децибел меньше чем у `{}`, но больше чем у `{}`", k1, k2)
            }
        },
        None => {
            let (k, _) = data.last().unwrap();
            println!("Уровень децибел больше чем у `{}`", k)
        }
    }
}
