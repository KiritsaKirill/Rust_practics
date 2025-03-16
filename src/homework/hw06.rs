fn draw_tree(triangles: usize) {
    let max_width = 2 * triangles - 1; // Максимальна ширина на останньому рівні

    // Проходимо через кожен рівень трикутників
    for t in 0..triangles {
        let star_count = 2 * t + 1; // Кількість зірочок для цього трикутника
        // Для кожного трикутника малюємо кілька рядків
        for i in 0..=t {
            let spaces = " ".repeat((max_width - (2 * i + 1)) / 2); // Пробіли для вирівнювання
            let stars = "*".repeat(2 * i + 1); // Зірочки на поточному рівні
            println!("{}{}", spaces, stars); // Виводимо пробіли та зірочки
        }
    }

    // Малюємо стовбур
    let trunk_width = 1; // Ширина стовбура
    let trunk_height = triangles; // Висота стовбура = кількість трикутників
    let trunk_spaces = " ".repeat((max_width - trunk_width) / 2); // Пробіли для вирівнювання стовбура по центру
    for _ in 0..trunk_height {
        println!("{}{}", trunk_spaces, "*"); // Виводимо стовбур
    }
}

fn main() {
    let triangles = 5; // Кількість трикутників
    draw_tree(triangles); // Викликаємо функцію для малювання ялинки
}
