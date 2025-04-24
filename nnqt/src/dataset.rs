use csv::ReaderBuilder;
use ndarray::{Array2, s};
use std::error::Error;
use std::fs::File;

pub fn load_dataset(path: &str) -> Result<(Array2<f32>, Vec<usize>, Array2<f32>, Vec<usize>), Box<dyn Error>> {
    let mut reader = ReaderBuilder::new().from_path(path)?;
    let mut features = Vec::new();
    let mut labels = Vec::new();

    for result in reader.records() {
        let record = result?;
        let mass: f32 = record[0].parse()?;
        let width: f32 = record[1].parse()?;
        let height: f32 = record[2].parse()?;
        let color_score: f32 = record[3].parse()?;
        let label: usize = record[4].parse()?;
        features.push(vec![mass, width, height, color_score]);
        labels.push(label - 1);
    }

    let n = features.len();
    let split = n * 80 / 100;
    let feature_array = Array2::from_shape_vec((n, 4), features.into_iter().flatten().collect())?;
    let train = feature_array.slice(s![..split, ..]).to_owned();
    let test = feature_array.slice(s![split.., ..]).to_owned();
    let train_labels = labels[..split].to_vec();
    let test_labels = labels[split..].to_vec();

    Ok((train, train_labels, test, test_labels))
}
