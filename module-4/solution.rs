/*
    Tugas:
    Lengkapi struktur dan fungsi berikut agar:
    - Struct `User` memiliki field `name: String` dan `age: u8`.
    - Enum `Status` berisi varian `Active`, `Inactive`, dan `Banned`.
    - Fungsi `describe_user` menerima struct dan enum lalu mencetak deskripsi sesuai status.
    - Fungsi `calculate_discount` menerima umur dan mengembalikan nilai diskon (10 jika umur >= 18, jika tidak 0).
    - Fungsi `handle_discount` mencetak pesan berdasarkan hasil dari `calculate_discount`.
*/

// TODO: Buat struct User dengan name dan age
struct User {
    name: String,
    age: u8,
}

// TODO: Buat enum Status
enum Status {
    Active,
    Inactive,
    Banned,
}

// TODO: Implementasi describe_user
fn describe_user(user: &User, status: &Status) {
    let status_str = match status {
        Status::Active => "Active",
        Status::Inactive => "Inactive",
        Status::Banned => "Banned",
    };
    println!("{} ({}) is {}", user.name, user.age, status_str);
}

// TODO: Jika umur >= 18 kembalikan 10, jika tidak 0
fn calculate_discount(age: u8) -> u8 {
    if age >= 18 { 10 } else { 0 }
}

// TODO: Cetak "Discount: 10%" jika diskon > 0, jika tidak "No discount"
fn handle_discount(age: u8) {
    let d = calculate_discount(age);
    if d > 0 {
        println!("Discount: {}%", d);
    } else {
        println!("No discount");
    }
}

fn main() {
    let user1 = User {
        name: String::from("Alice"),
        age: 20,
    };
    let user2 = User {
        name: String::from("Bob"),
        age: 12,
    };

    let status1 = Status::Active;
    let status2 = Status::Banned;

    describe_user(&user1, &status1);
    handle_discount(user1.age);

    describe_user(&user2, &status2);
    handle_discount(user2.age);
}
