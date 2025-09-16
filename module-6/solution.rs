use std::fmt::Display;

// TODO: Buat struct generic Wrapper<T>
struct Wrapper<T> {
    value: T,
}

// TODO: Implementasi fungsi yang menerima Wrapper<T> di mana T: Display
fn print_wrapper<T: Display>(item: Wrapper<T>) {
    println!("Wrapper contains: {}", item.value);
}

// TODO: Buat struct Pair<T> dengan field x dan y
struct Pair<T> {
    x: T,
    y: T,
}

// TODO: Implementasi fungsi compare yang mengembalikan referensi ke nilai terbesar
// Clue: jangan lupa untuk menerapkan trait yang mendukung operator perbandingan
impl<T: PartialOrd> Pair<T> {
    fn compare(&self) -> &T {
        if self.x >= self.y {
            &self.x
        } else {
            &self.y
        }
    }
}

// TODO: Fungsi untuk mengambil string terpanjang (menggunakan lifetime)
fn longest_str<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() {
        a
    } else {
        b
    }
}

fn main() {
    let int_wrapper = Wrapper { value: 42 };
    let str_wrapper = Wrapper { value: "Hello" };
    print_wrapper(int_wrapper);
    print_wrapper(str_wrapper);

    let num_pair = Pair { x: 42, y: 99 };
    println!("Bigger: {}", num_pair.compare());

    let s1 = String::from("Rust");
    let s2 = String::from("programming");
    let result = longest_str(&s1, &s2);
    println!("Longer string: {}", result);
}
