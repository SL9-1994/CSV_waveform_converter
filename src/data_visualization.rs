use plotters::prelude::*;

use crate::integral::PlotData;

fn plot_disp(disp: &[f64], title: &str, interval: f64) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("disp.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let x_values: Vec<f64> = (0..disp.len()).map(|i| i as f64 * interval).collect();

    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif", 30))
        .margin(5)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_ranged(
            0.0..x_values[x_values.len() - 1],
            disp.iter().cloned().fold(f64::INFINITY, f64::min)
                ..disp.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
        )?;

    chart
        .configure_mesh()
        .x_labels(10)
        .y_labels(10)
        .disable_x_mesh()
        .disable_y_mesh()
        .draw()?;

    chart.draw_series(LineSeries::new(
        x_values
            .iter()
            .zip(disp.iter().cloned())
            .map(|(x, y)| (*x, y)),
        &BLUE,
    ))?;

    Ok(())
}

pub fn main(data: &PlotData) -> Result<(), Box<dyn std::error::Error>> {
    plot_disp(&data.ns_disp, "NS Displacement", 0.01)?;
    // plot_disp(&data.ew_disp, "EW Displacement", 0.01)?;
    // plot_disp(&data.ud_disp, "UD Displacement", 0.01)?;

    Ok(())
}
