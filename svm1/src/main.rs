use linfa::prelude::*;
use linfa_svm::Svm;
use ndarray::array;

fn main() {
    // Dataset sederhana (warna, tekstur, ukuran)
    let features = array![
        [1.0, 0.5, 3.0],  // Mentah
        [1.5, 0.6, 3.2],  // Mentah
        [2.0, 0.8, 4.0],  // Matang
        [2.2, 0.9, 4.1],  // Matang
        [1.8, 0.7, 3.5],  // Matang
        [1.0, 0.4, 2.8],  // Busuk
        [1.2, 0.5, 2.9],  // Busuk
    ];

    let labels = array![0.0, 0.0, 1.0, 1.0, 1.0, 2.0, 2.0]; // Pakai `f64`

    // Membuat dataset
    let dataset = Dataset::new(features, labels);

    // Melatih model SVM
    let model = Svm::<f64, f64>::params() // Ganti `i32` menjadi `f64`
    .fit(&dataset)
    .expect("Gagal melatih model!");



    // Contoh prediksi
    let test_data = array![[1.9, 0.8, 3.8]]; // Buah baru
    let prediction = model.predict(&test_data);

    println!("Prediksi kelas: {:?}", prediction);
}

