use plotters::prelude::*;
use std::error::Error;
use ndarray::Array2;

pub fn visualize_dataset(features: &Array2<f32>, labels: &Vec<usize>, output_path: &str) -> Result<(), Box<dyn Error>> {
    let root = BitMapBackend::new(output_path, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Visualisasi Dataset", ("sans-serif", 30))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0f32..1f32, 0f32..1f32)?;

    chart.configure_mesh().draw()?;

    let colors = [RED, BLUE, GREEN, MAGENTA];

    for ((x, y), label) in features.outer_iter().map(|row| (row[0], row[1])).zip(labels.iter()) {
        chart.draw_series(std::iter::once(Circle::new((x, y), 5, colors[*label % colors.len()].filled())))?;
    }

    Ok(())
}

pub fn plot_accuracy_graph(train_acc: &Vec<f32>, test_acc: &Vec<f32>, output_path: &str) -> Result<(), Box<dyn Error>> {
    let root = BitMapBackend::new(output_path, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let max_epoch = train_acc.len();
    let mut chart = ChartBuilder::on(&root)
        .caption("Akurasi Training vs Testing", ("sans-serif", 30))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0..max_epoch, 0f32..1.0)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            train_acc.iter().enumerate().map(|(i, &y)| (i, y)),
            &RED,
        ))?
        .label("Train Accuracy")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .draw_series(LineSeries::new(
            test_acc.iter().enumerate().map(|(i, &y)| (i, y)),
            &BLUE,
        ))?
        .label("Test Accuracy")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    chart.configure_series_labels().background_style(&WHITE.mix(0.8)).draw()?;

    Ok(())
}

pub fn confusion_matrix(predicted: &Vec<usize>, actual: &Vec<usize>, output_path: &str) -> Result<(), Box<dyn Error>> {
    let classes = 3;
    let mut matrix = vec![vec![0u8; classes]; classes];

    for (&a, &p) in actual.iter().zip(predicted.iter()) {
        matrix[a][p] += 1;
    }

    let cell_size = 100;
    let image_size = cell_size * classes;

    let root = BitMapBackend::new(output_path, (image_size as u32, image_size as u32)).into_drawing_area();
    root.fill(&WHITE)?;

    for i in 0..classes {
        for j in 0..classes {
            let count = matrix[i][j];

            let x0 = (j * cell_size) as i32;
            let y0 = (i * cell_size) as i32;
            let x1 = ((j + 1) * cell_size) as i32;
            let y1 = ((i + 1) * cell_size) as i32;

            root.draw(&Rectangle::new(
                [(x0, y0), (x1, y1)],
                RGBColor(200 - (count * 10), 200, 255).filled(),
            ))?;

            root.draw(&Text::new(
                format!("{}", count),
                (x0 + 30, y0 + 30),
                ("sans-serif", 20).into_font().color(&BLACK),
            ))?;
        }
    }

    Ok(())
}
