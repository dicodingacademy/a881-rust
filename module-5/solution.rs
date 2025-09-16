/*
    Tugas:
    Lengkapi fungsi-fungsi berikut untuk:
    - Membaca angka dari string menggunakan parsing dan mengembalikan Result
    - Menjumlahkan dua string yang merepresentasikan angka (dalam Result)
    - Menampilkan hasil jika parsing sukses, atau pesan error jika gagal
    - Menangani kasus None saat mengambil elemen dari vector

    Contoh Output:
    Sum is: 30
    Failed to parse input.
    Found: Some(99)
    No value at index
*/

fn parse_number(s: &str) -> Result<i32, String> {
    // Gunakan match untuk menangani hasil parsing string ke i32
    match s.trim().parse::<i32>() {
        Ok(n) => Ok(n),
        Err(_) => Err("Failed to parse input.".to_string()),
    }
}

fn add_numbers(a: &str, b: &str) -> Result<i32, String> {
    // Gunakan parse_number dan operator ? untuk menjumlahkan dua input string
    let x = parse_number(a)?;
    let y = parse_number(b)?;
    Ok(x + y)
}

fn get_element(vec: &Vec<i32>, index: usize) -> Option<i32> {
    // Kembalikan Some(nilai) jika index valid; jika tidak, None
    vec.get(index).copied()
}

fn main() {
    let result = add_numbers("10", "20");
    match result {
        Ok(sum) => println!("Sum is: {}", sum),
        Err(e) => println!("{}", e),
    }

    let failed = add_numbers("10", "abc");
    match failed {
        Ok(sum) => println!("Sum is: {}", sum),
        Err(e) => println!("{}", e),
    }

    let data = vec![11, 22, 33, 99];
    let found = get_element(&data, 3);
    match found {
        Some(value) => println!("Found: Some({})", value),
        None => println!("No value at index"),
    }

    let not_found = get_element(&data, 10);
    match not_found {
        Some(value) => println!("Found: Some({})", value),
        None => println!("No value at index"),
    }
}
