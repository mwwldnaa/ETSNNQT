use csv::Reader;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::error::Error;
use std::io;

#[derive(Debug, Deserialize)]
struct DataPoint {
    warna: f64,
    tekstur: f64,
    berat: f64,
    label: String,
}

// Fungsi untuk menghitung jarak Euclidean
fn euclidean_distance(a: &DataPoint, b: &DataPoint) -> f64 {
    ((a.warna - b.warna).powi(2) + (a.tekstur - b.tekstur).powi(2) + (a.berat - b.berat).powi(2)).sqrt()
}

// Fungsi untuk melakukan klasifikasi dengan KNN
fn knn_classify(dataset: &[DataPoint], test_point: &DataPoint, k: usize) -> String {
    let mut distances: Vec<(f64, &String)> = dataset
        .iter()
        .map(|data| (euclidean_distance(data, test_point), &data.label))
        .collect();

    distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let mut label_count = HashMap::new();
    for (_, label) in distances.iter().take(k) {
        *label_count.entry(label.to_string()).or_insert(0) += 1;
    }

    label_count.into_iter().max_by_key(|&(_, count)| count).unwrap().0
}

// Fungsi untuk membaca dataset dari file CSV
fn read_dataset(filename: &str) -> Result<Vec<DataPoint>, Box<dyn Error>> {
    let file = File::open(filename)?;
    let mut rdr = Reader::from_reader(file);
    let mut dataset = Vec::new();

    for result in rdr.deserialize() {
        let record: DataPoint = result?;
        dataset.push(record);
    }

    Ok(dataset)
}

fn main() {
    let filename = "dataset.csv";

    // Cek apakah file dataset tersedia
    let dataset = match read_dataset(filename) {
        Ok(data) => data,
        Err(_) => {
            eprintln!("Error: File {} tidak ditemukan! Pastikan dataset tersedia.", filename);
            return;
        }
    };

    let mut input = String::new();
    println!("Masukkan warna (0 atau 1): ");
    io::stdin().read_line(&mut input).expect("Gagal membaca input");
    let warna: f64 = input.trim().parse().expect("Input harus berupa angka");
    input.clear();

    println!("Masukkan tekstur (0 atau 1): ");
    io::stdin().read_line(&mut input).expect("Gagal membaca input");
    let tekstur: f64 = input.trim().parse().expect("Input harus berupa angka");
    input.clear();

    println!("Masukkan berat (gram): ");
    io::stdin().read_line(&mut input).expect("Gagal membaca input");
    let berat: f64 = input.trim().parse().expect("Input harus berupa angka");

    let test_point = DataPoint {
        warna,
        tekstur,
        berat,
        label: String::from(""),
    };

    let predicted_label = knn_classify(&dataset, &test_point, 3);
    println!("Buah diprediksi sebagai: {}", predicted_label);
}
