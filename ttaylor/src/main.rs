use std::io;
use std::f64::consts::PI;

/// Fungsi untuk menghitung faktorial
fn factorial(n: u64) -> u64 {
    (1..=n).product()
}

/// Implementasi sin(x) dengan deret Taylor
fn taylor_sin(x: f64, terms: u64) -> f64 {
    let mut result = 0.0;
    for n in 0..terms {
        let term = ((-1.0_f64).powi(n as i32) * x.powi(2 * n as i32 + 1)) / factorial(2 * n + 1) as f64;
        result += term;
    }
    result
}

/// Implementasi cos(x) dengan deret Taylor
fn taylor_cos(x: f64, terms: u64) -> f64 {
    let mut result = 0.0;
    for n in 0..terms {
        let term = ((-1.0_f64).powi(n as i32) * x.powi(2 * n as i32)) / factorial(2 * n) as f64;
        result += term;
    }
    result
}

//fungsi untuk menghitung taylor
fn main() {
    let mut input = String::new();

    // Meminta input nilai x dari pengguna
    println!("Masukkan nilai x dalam derajat: ");
    io::stdin().read_line(&mut input).expect("Gagal membaca input");
    let x: f64 = input.trim().parse().expect("Harap masukkan angka!");

    // Konversi derajat ke radian (karena trigonometri bekerja dalam radian)
    let x_radian = x * (PI / 180.0);

    let terms = 10; // Jumlah suku dalam deret Taylor

    // Hitung nilai sin dan cos berdasarkan input
    let sin_approx = taylor_sin(x_radian, terms);
    let cos_approx = taylor_cos(x_radian, terms);

    println!("sin({:.5}) ≈ {:.5}", x, sin_approx);
    println!("cos({:.5}) ≈ {:.5}", x, cos_approx);
}
