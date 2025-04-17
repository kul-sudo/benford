use crate::int_to_base;
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

pub fn save_histogram_split(
    data: Vec<u128>,
    output_file: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(output_file, (1280, 960))
        .into_drawing_area()
        .titled(
            "Red: Reality / Black: Benford's law prediction",
            ("sans-serif", 15.0),
        )
        .unwrap();
    root.fill(&WHITE)?;

    let areas = root.split_evenly((4, 3));

    for (i, area) in areas.iter().enumerate() {
        let base = i as u128 + 3;
        let benford_predictions = (1..base)
            .map(|d| {
                (
                    d,
                    ((((d + 1) as f64).log(base as f64) - (d as f64).log(base as f64)) * 100.0)
                        .round() as u128,
                )
            })
            .collect::<Vec<_>>();
        let benford_max = benford_predictions.iter().map(|(_, x)| *x).max().unwrap();

        let mut statistics = vec![0; base as usize];
        for x in &data {
            statistics[*int_to_base(*x, base).last().unwrap() as usize] += 1;
        }

        let statistics_percent = statistics
            .iter()
            .map(|x| {
                ((*x as f64 / statistics[1..].iter().sum::<u128>() as f64) * 100.0).round() as u128
            })
            .collect::<Vec<_>>();

        let data_max = statistics_percent.iter().max().unwrap();
        let max = benford_max.max(*data_max);

        let mut chart = ChartBuilder::on(area)
            .x_label_area_size(35)
            .y_label_area_size(40)
            .margin(5)
            .caption(base.to_string(), ("sans-serif", 15.0))
            .build_cartesian_2d((1..base - 1).into_segmented(), 0u128..max + 5)?;

        chart
            .configure_mesh()
            .disable_x_mesh()
            .bold_line_style(WHITE.mix(0.3))
            .y_desc("%")
            .x_desc("First digit")
            .axis_desc_style(("sans-serif", 12))
            .draw()?;

        chart.draw_series(
            Histogram::vertical(&chart).style(RED.filled()).data(
                statistics_percent
                    .iter()
                    .enumerate()
                    .map(|(index, x)| (index as u128, *x)),
            ),
        )?;

        chart.draw_series(
            Histogram::vertical(&chart)
                .style(BLACK.filled())
                .data(benford_predictions.iter().map(|&(index, x)| (index, x)))
                .margin(12),
        )?;
    }

    Ok(())
}
