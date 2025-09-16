// TODO: Import modul `math_utils` dan `string_utils`
mod math_utils;
mod string_utils;

fn main() {
    let sum = math_utils::add(7, 5);
    let upper = string_utils::to_uppercase("rustacean");

    println!("Sum: {}", sum);
    println!("Upper: {}", upper);
}
