use std::io::{self, Write};

fn invert_the_case(s: String) -> String {
    s.chars()
        .map(|c| {
            if c.is_uppercase() {
                c.to_lowercase().to_string()
            } else {
                c.to_uppercase().to_string()
            }
        })
        .collect()
}

fn main() {
    println!("Введіть текст (для завершення натискайте Ctrl+C):");

    loop {
        let mut input = String::new();

        // Зчитуємо введений текст
        print!("Введіть слово: ");
        io::stdout().flush().unwrap(); // Щоб миттєво вивести запит на екран
        io::stdin().read_line(&mut input).unwrap();

        // Очищаємо від символа нового рядка в кінці
        input = input.trim().to_string();

        // Перетворюємо регістр введеного слова
        let inverted = invert_the_case(input);

        // Виводимо результат
        println!("Змінений регістр: {}", inverted);
    }
}
