fn draw_filled_diamond(size: usize) {
    let max_width = size * 2 - 1;

    // Верхняя половина ромба
    for i in 0..size {
        for j in 0..max_width {
            if j >= size - 1 - i && j <= size - 1 + i {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }

    // Нижняя половина ромба
    for i in (0..size - 1).rev() {
        for j in 0..max_width {
            if j >= size - 1 - i && j <= size - 1 + i {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn main() {
    let size = 7; // Размер ромба
    draw_filled_diamond(size); // Рисуем заполненный ромб
}
