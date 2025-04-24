pub fn calculate_accuracy(predicted: &Vec<usize>, actual: &Vec<usize>) -> f32 {
    let correct = predicted.iter().zip(actual.iter()).filter(|(a, b)| a == b).count();
    correct as f32 / actual.len() as f32
}

pub fn evaluate_model(predicted: &Vec<usize>, actual: &Vec<usize>) {
    println!("\nEvaluasi Model:");
    for (i, (pred, act)) in predicted.iter().zip(actual.iter()).enumerate() {
        println!("Data {} => Prediksi: {}, Aktual: {}", i + 1, pred + 1, act + 1);
    }
}
