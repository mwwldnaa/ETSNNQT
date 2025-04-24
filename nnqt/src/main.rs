mod dataset;
mod model;
mod evaluation;
mod visualization;

use dataset::load_dataset;
use model::NeuralNetwork;
use evaluation::{evaluate_model, calculate_accuracy};
use visualization::{plot_accuracy_graph, visualize_dataset, confusion_matrix};

fn main() {
    let (train_features, train_labels, test_features, test_labels) = load_dataset("dataset.csv").expect("Gagal load dataset");

    visualize_dataset(&train_features, &train_labels, "output/dataset.png").unwrap();

    let mut nn = NeuralNetwork::new(4, 8, 3);
    let epochs = 100;
    let learning_rate = 0.1;

    let mut train_acc_history = Vec::new();
    let mut test_acc_history = Vec::new();

    for _ in 0..epochs {
        nn.train(&train_features, &train_labels, learning_rate, 1);
        let train_pred = nn.predict(&train_features);
        let test_pred = nn.predict(&test_features);
        train_acc_history.push(calculate_accuracy(&train_pred, &train_labels));
        test_acc_history.push(calculate_accuracy(&test_pred, &test_labels));
    }

    plot_accuracy_graph(&train_acc_history, &test_acc_history, "output/accuracy.png").unwrap();

    let predictions = nn.predict(&test_features);
    confusion_matrix(&predictions, &test_labels, "output/confusion_matrix.png").unwrap();

    let accuracy = calculate_accuracy(&predictions, &test_labels);
    println!("Akurasi: {:.2}%", accuracy * 100.0);

    evaluate_model(&predictions, &test_labels);
}
