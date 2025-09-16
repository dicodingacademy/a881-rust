// TODO: Buatlah konstanta PI
const PI: f32 = 3.14;

// TODO: Buatlah fungsi untuk menghitung luas lingkaran
fn luas_lingkaran(r: f32) -> f32 {
    PI * r * r
}

fn main() {
    // TODO: Deklarasikan variabel radius immutable
    let radius: f32 = 4.0;

    // TODO: Lakukan percabangan if/else untuk memvalidasi radius
    if radius > 0.0 {
        println!("{:.2}", luas_lingkaran(radius));
    } else {
        // radius tidak valid; tidak mencetak apa-apa
    }

    // TODO: Lakukan for loop untuk mencetak angka 1 sampai 5
    for i in 1..=5 {
        println!("{}", i);
    }

    // TODO: Lakukan match untuk mencetak kategori berdasarkan nilai radius
    match radius {
        r if r <= 5.0 => println!("Kecil"),
        r if r <= 10.0 => println!("Sedang"),
        _ => println!("Besar"),
    }
}

/*
* Expected output:
* 50.24
* 1
* 2
* 3
* 4
* 5
* Kecil
*
*/
