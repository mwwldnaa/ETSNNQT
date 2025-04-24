use plotters::prelude::*;

pub fn plot_accuracy(accs: &Vec<f32>) {
    let root = BitMapBackend::new("accuracy.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let max_y = accs.iter().cloned().fold(0. / 0., f32::max);

    let mut chart = ChartBuilder::on(&root)
        .caption("Accuracy over Epochs", ("sans-serif", 30))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0..accs.len(), 0.0f32..max_y)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    chart.draw_series(LineSeries::new(
        accs.iter().enumerate().map(|(i, y)| (i, *y)),
        &BLUE,
    )).unwrap();
}
