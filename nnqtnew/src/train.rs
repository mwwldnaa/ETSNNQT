use crate::{data::load_dataset, model::TrafficModel, plot::plot_accuracy};
use burn::{
    module::Module,
    optim::Adam,
    record::FullPrecisionSettings,
    tensor::{backend::ndarray::NdArrayBackend, Tensor},
    train::{LearnerBuilder, ClassificationOutput, metric::AccuracyMetric}
};

type B = NdArrayBackend<f32>;

pub fn train_model() {
    let dataset = load_dataset("dataset.csv");

    let (inputs, targets): (Vec<_>, Vec<_>) = dataset.iter()
        .map(|s| {
            (vec![s.car, s.bike, s.bus, s.truck, s.total], s.label)
        })
        .unzip();

    let input_tensor = Tensor::<B, 2>::from_floats(inputs).reshape([inputs.len(), 5]);
    let target_tensor = Tensor::<B, 1>::from_data(targets.iter().map(|&v| v as i64).collect());

    let model = TrafficModel::new();
    let optimizer = Adam::new();

    let learner = LearnerBuilder::new(FullPrecisionSettings::default())
        .metric_train(AccuracyMetric::new())
        .loss_fn(burn::loss::CrossEntropyLoss::new())
        .optimizer(optimizer)
        .build(model, ());

    let output = learner.fit(input_tensor.clone(), target_tensor.clone(), 20);

    println!("✅ Final accuracy: {:.2}%", output.train_metric.unwrap().accuracy * 100.0);

    plot_accuracy(&output.train_metrics.unwrap());
}
