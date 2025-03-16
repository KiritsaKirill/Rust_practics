fn draw_envelope(size: usize) {
    let enlarged_size = size * 3; // Увеличиваем размер в 3 раза
    for i in 0..enlarged_size {
        for j in 0..enlarged_size {
            // Умова для рамки
            if i == 0 || i == enlarged_size - 1 || j == 0 || j == enlarged_size - 1 {
                print!("*");
            }
            // Умова для перехрестя (діагоналі)
            else if i == j || i + j == enlarged_size - 1 {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn main() {
    let size = 7; // Исходный размер квадрата
    draw_envelope(size); // Рисуем увеличенную форму
}
