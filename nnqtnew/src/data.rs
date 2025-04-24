use polars::prelude::*;

#[derive(Debug)]
pub struct TrafficSample {
    pub car: f32,
    pub bike: f32,
    pub bus: f32,
    pub truck: f32,
    pub total: f32,
    pub label: usize, // 0 = low, 1 = normal, 2 = high
}

pub fn load_dataset(path: &str) -> Vec<TrafficSample> {
    let df = CsvReader::from_path(path)
        .unwrap()
        .has_header(true)
        .finish()
        .unwrap();

    let mut data = Vec::new();

    for i in 0..df.height() {
        let car: f32 = df.column("CarCount").unwrap().get(i).unwrap().to_string().parse().unwrap();
        let bike: f32 = df.column("BikeCount").unwrap().get(i).unwrap().to_string().parse().unwrap();
        let bus: f32 = df.column("BusCount").unwrap().get(i).unwrap().to_string().parse().unwrap();
        let truck: f32 = df.column("TruckCount").unwrap().get(i).unwrap().to_string().parse().unwrap();
        let total: f32 = df.column("Total").unwrap().get(i).unwrap().to_string().parse().unwrap();

        let label_str = df.column("Traffic Situation").unwrap().get(i).unwrap().to_string();
        let label = match label_str.trim_matches('"') {
            "low" => 0,
            "normal" => 1,
            "high" => 2,
            _ => 1,
        };

        data.push(TrafficSample { car, bike, bus, truck, total, label });
    }

    data
}
