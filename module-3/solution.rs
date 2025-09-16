/*
    Tugas:
    Lengkapi fungsi-fungsi berikut agar:
    - Fungsi `transfer_ownership` menerima String dan memindahkan kepemilikannya.
    - Fungsi `borrow_data` menerima reference ke String dan mengembalikan panjang karakter.
    - Fungsi `mutate_string` menerima mutable reference dan menambahkan kata " Rustacean".
*/

// TODO: Lengkapi fungsi ini agar memindahkan ownership dari parameter dan mencetaknya
fn transfer_ownership(s: String) {
    println!("{}", s);
    // s keluar scope di sini -> ownership dilepas
}

// TODO: Lengkapi fungsi ini agar menggunakan reference (borrow) dan mengembalikan panjang string (data.len())
fn borrow_data(data: &String) -> usize {
    data.len()
}

// TODO: Lengkapi fungsi ini agar menerima mutable reference dan memodifikasi string
fn mutate_string(text: &mut String) {
    text.push_str(" Rustacean");
}

fn main() {
    let mut original = String::from("Hello");

    // Peminjaman immutable
    let len = borrow_data(&original);
    println!("Length: {}", len);

    // Peminjaman mutable
    mutate_string(&mut original);
    println!("Modified: {}", original);

    // Transfer ownership
    transfer_ownership(original.clone());
}
