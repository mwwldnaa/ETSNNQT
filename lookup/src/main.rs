use std::io;
use std::f64::consts::PI;

const TABLE_SIZE: usize = 361; // Lookup table untuk 0° - 360°
const DEGREE_TO_RAD: f64 = PI / 180.0; // Konversi derajat ke radian

// Membuat lookup table untuk sin dan cos
fn generate_lookup_table() -> ([f64; TABLE_SIZE], [f64; TABLE_SIZE]) {
    let mut sin_table = [0.0; TABLE_SIZE];
    let mut cos_table = [0.0; TABLE_SIZE];

    for i in 0..TABLE_SIZE {
        let rad = (i as f64) * DEGREE_TO_RAD;
        sin_table[i] = rad.sin();
        cos_table[i] = rad.cos();
    }

    (sin_table, cos_table)
}

fn main() {
    let (sin_table, cos_table) = generate_lookup_table();

    // Input dari user
    let mut input = String::new();
    println!("Masukkan sudut dalam derajat (0-360): ");
    io::stdin().read_line(&mut input).expect("Gagal membaca input");

    // Parsing nilai input
    let angle: usize = match input.trim().parse() {
        Ok(num) if num < TABLE_SIZE => num,
        _ => {
            println!("Masukkan angka antara 0-360!");
            return;
        }
    };

    // Mengambil nilai dari lookup table
    let sin_value = sin_table[angle];
    let cos_value = cos_table[angle];

    println!("sin({}) = {:.6}", angle, sin_value);
    println!("cos({}) = {:.6}", angle, cos_value);
}
