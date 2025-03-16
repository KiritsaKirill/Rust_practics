use std::io;

// Функция для проверки, простое ли число
fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false; // Число меньше или равно 1 не является простым
    }

    // Проверка на делимость числа от 2 до √n
    for i in 2..=(n as f64).sqrt() as u32 {
        if n % i == 0 {
            return false; // Если число делится на i, то оно не простое
        }
    }
    
    true // Если не делится на ни одно число, то простое
}

fn main() {
    println!("Введіть число для перевірки на простоту:");

    let mut input = String::new();

    // Считывание числа с клавиатуры
    io::stdin()
        .read_line(&mut input)
        .expect("Не вдалося прочитати рядок");

    // Преобразуем строку в число
    let number: u32 = input.trim().parse().expect("Будь ласка, введіть дійсне число");

    // Выводим результат
    if is_prime(number) {
        println!("Число {} є простим.", number);
    } else {
        println!("Число {} не є простим.", number);
    }
}
