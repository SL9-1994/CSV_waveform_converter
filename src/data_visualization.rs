use crate::integral::PlotData;
use plotters::prelude::*;

fn plot_data(
    data: &[f64],
    title: &str,
    interval: f64,
    types: &str,
    file_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let output_file = format!("{}_{}.png", file_name, types);
    // 画像出力 (800x600)
    let root = BitMapBackend::new(&output_file, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?; // 背景を白にする

    let x_values: Vec<f64> = (0..data.len()).map(|i| i as f64 * interval).collect(); // 0.0, 0.01, 0.02, 0.03, 0.04, 0.05, ...

    let mut chart = ChartBuilder::on(&root) // グラフを描画するエリアを指定
        .caption(title, ("sans-serif", 30))
        .margin(10) // グラフの余白を設定
        .x_label_area_size(40) // X軸のラベルの余白を設定
        .y_label_area_size(40) // Y軸のラベルの余白を設定
        .build_cartesian_2d(
            0.0..x_values[x_values.len() - 1],
            data.iter().cloned().fold(f64::INFINITY, f64::min) - 1.0
                ..data.iter().cloned().fold(f64::NEG_INFINITY, f64::max) + 1.0,
        )?;

    chart
        .configure_mesh() // メッシュを設定
        .x_labels(5) // X軸のラベルを5個にする
        .y_labels(5) // Y軸のラベルを5個にする
        // .disable_x_mesh() // X軸のメッシュを消す
        // .disable_y_mesh() // Y軸のメッシュを消す
        .draw()?; // メッシュを描画

    let max = data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let min = data.iter().cloned().fold(f64::INFINITY, f64::min);
    // println!("Max: {}, Min: {}", max, min);

    // 最大値と最小値をグラフに表示
    let max_point = (x_values[data.iter().position(|&x| x == max).unwrap()], max);
    let min_point = (x_values[data.iter().position(|&x| x == min).unwrap()], min);
    chart.draw_series(
        [max_point, min_point]
            .iter()
            .map(|&(x, y)| Circle::new((x, y), 2, RED.filled())),
    )?;

    // グラフにデータをプロット （x y）のイテレーターを指定イテ
    chart.draw_series(LineSeries::new(
        x_values
            .iter()
            .zip(data.iter().cloned())
            .map(|(x, y)| (*x, y)),
        &BLACK,
    ))?;

    Ok(())
}

pub fn main(data: &PlotData) -> Result<(), Box<dyn std::error::Error>> {
    plot_data(
        &data.ns_acc,
        "NS acceleration",
        0.01,
        "ns_acc",
        &data.file_name,
    )?;
    plot_data(
        &data.ew_acc,
        "EW acceleration",
        0.01,
        "ew_acc",
        &data.file_name,
    )?;
    plot_data(
        &data.ud_acc,
        "UD acceleration",
        0.01,
        "ud_acc",
        &data.file_name,
    )?;
    plot_data(
        &data.ns_dist,
        "NS Displacement",
        0.01,
        "ns_dist",
        &data.file_name,
    )?;
    plot_data(
        &data.ew_dist,
        "EW Displacement",
        0.01,
        "ew_dist",
        &data.file_name,
    )?;
    plot_data(
        &data.ud_dist,
        "UD Displacement",
        0.01,
        "ud_dist",
        &data.file_name,
    )?;
    plot_data(&data.ns_vel, "NS Velocity", 0.01, "ns_vel", &data.file_name)?;
    plot_data(&data.ew_vel, "EW Velocity", 0.01, "ew_vel", &data.file_name)?;
    plot_data(&data.ud_vel, "UD Velocity", 0.01, "ud_vel", &data.file_name)?;
    let max = data
        .ud_acc
        .iter()
        .cloned()
        .fold(f64::NEG_INFINITY, f64::max);
    let min = data.ud_acc.iter().cloned().fold(f64::INFINITY, f64::min);
    println!("Max: {}, Min: {}", max, min);
    Ok(())
}

