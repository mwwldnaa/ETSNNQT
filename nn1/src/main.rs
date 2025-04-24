use ndarray::{Array2, Array1};
use rand::prelude::*;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Aktivasi Sigmoid
fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

/// Struktur jaringan saraf sederhana
struct NeuralNetwork {
    weights: Array2<f64>,
    bias: Array1<f64>,
}

impl NeuralNetwork {
    /// Inisialisasi jaringan saraf
    fn new(input_size: usize, output_size: usize) -> Self {
        let mut rng = rand::thread_rng();
        let weights = Array2::from_shape_fn((output_size, input_size), |_| rng.gen_range(-1.0..1.0));
        let bias = Array1::from_shape_fn(output_size, |_| rng.gen_range(-1.0..1.0));
        Self { weights, bias }
    }

    /// Prediksi output dari input
    fn predict(&self, input: &Array1<f64>) -> usize {
        let z = self.weights.dot(input) + &self.bias;
        let prob = z.mapv(sigmoid);
        
        // Ambil indeks dengan nilai probabilitas tertinggi sebagai kelas prediksi
        prob.iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(idx, _)| idx)
            .unwrap_or(0)
    }
}

/// Load dataset dari CSV
fn load_dataset(filename: &str) -> Result<Vec<(Array1<f64>, usize)>, Box<dyn Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut dataset = Vec::new();

    for line in reader.lines().skip(1) {
        let line = line?;
        let values: Result<Vec<f64>, _> = line.split(',').map(|s| s.parse::<f64>()).collect();
        let values = match values {
            Ok(v) => v,
            Err(_) => continue, // Lewati baris jika ada error parsing
        };

        if values.len() < 4 {
            continue; // Lewati baris dengan data tidak lengkap
        }

        let (features, label) = values.split_at(values.len() - 1);
        let input = Array1::from_vec(features.to_vec());
        let output = label[0] as usize; // Ubah label menjadi integer
        dataset.push((input, output));
    }
    Ok(dataset)
}

/// Fungsi untuk mengonversi label angka ke teks
fn label_to_text(label: usize) -> &'static str {
    match label {
        0 => "Mentah",
        1 => "Matang",
        2 => "Busuk",
        _ => "Tidak Diketahui",
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let dataset = load_dataset("dataset.csv")?;
    
    if dataset.is_empty() {
        println!("Dataset kosong atau tidak valid!");
        return Ok(());
    }

    let input_size = dataset[0].0.len();
    let output_size = 3; // Ada 3 kelas (Mentah, Matang, Busuk)
    let nn = NeuralNetwork::new(input_size, output_size);

    for (input, actual_label) in &dataset {
        let predicted_label = nn.predict(input);
        println!(
            "Actual: {}, Predicted: {}",
            label_to_text(*actual_label),
            label_to_text(predicted_label)
        );
    }
    Ok(())
}
