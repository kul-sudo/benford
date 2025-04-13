use plotters::prelude::*;

pub fn save_histogram(
    base: u32,
    data: Vec<f64>,
    output_file: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let benford_predictions = (1..base).map(|d| {
        (
            d,
            ((((d + 1) as f64).log(base as f64) - (d as f64).log(base as f64)) * 100.0).round()
                as u32,
        )
    });

    let data = data
        .iter()
        .enumerate()
        .map(|(index, x)| (index as u32, (*x * 100.0).round() as u32));

    let benford_max = benford_predictions.clone().map(|(_, x)| x).max().unwrap();
    let data_max = data.clone().map(|(_, x)| x).max().unwrap();
    let max = benford_max.max(data_max);

    let root = BitMapBackend::new(output_file, (640, 480)).into_drawing_area();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(35)
        .y_label_area_size(40)
        .margin(5)
        .caption(
            "Red: Reality / Black: Benford's law prediction",
            ("sans-serif", 20.0),
        )
        .build_cartesian_2d((1..base - 1).into_segmented(), 0u32..max + 5)?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .bold_line_style(WHITE.mix(0.3))
        .y_desc("%")
        .x_desc("First digit")
        .axis_desc_style(("sans-serif", 15))
        .draw()?;

    chart.draw_series(Histogram::vertical(&chart).style(RED.filled()).data(data))?;

    chart.draw_series(
        Histogram::vertical(&chart)
            .style(BLACK.filled())
            .data(benford_predictions)
            .margin(20),
    )?;

    Ok(())
}
