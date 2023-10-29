use crate::csv_parser::ParseData;
use crate::data_visualization;

pub struct PlotData {
    pub title: String,
    pub rate: String,
    pub unit: String,
    pub time: String,
    pub ns_acc: Vec<f64>,
    pub ew_acc: Vec<f64>,
    pub ud_acc: Vec<f64>,
    pub ns_vel: Vec<f64>,
    pub ew_vel: Vec<f64>,
    pub ud_vel: Vec<f64>,
    pub ns_disp: Vec<f64>,
    pub ew_disp: Vec<f64>,
    pub ud_disp: Vec<f64>,
}

// 台形公式を用いた加速度数値積分
pub fn integral(data: &ParseData) {
    let s_late: f64 = 0.01; // SAMPLING RATE = 100Hz

    let ns_acc = &data.ns_acc; // NS Acceleration
    let ew_acc = &data.ew_acc; // EW Acceleration
    let ud_acc = &data.ud_acc; // UD Acceleration

    let mut ns_vel: Vec<f64> = Vec::new(); // NS Velocity
    let mut ew_vel: Vec<f64> = Vec::new(); // EW Velocity
    let mut ud_vel: Vec<f64> = Vec::new(); // UD Velocity

    let mut ns_disp: Vec<f64> = Vec::new(); // NS Displacement
    let mut ew_disp: Vec<f64> = Vec::new(); // EW Displacement
    let mut ud_disp: Vec<f64> = Vec::new(); // UD Displacement

    // 全ての行は同じ長さであることが保証されているため、ns_acc.len()で長さを取得
    // 加速度から速度を求める
    for i in 0..ns_acc.len() {
        if i == 0 {
            ns_vel.push(0.0);
            ew_vel.push(0.0);
            ud_vel.push(0.0);
        } else {
            ns_vel.push((ns_vel[i - 1] + s_late * ns_acc[i] + ns_acc[i - 1]) / 2.0);
            ew_vel.push((ew_vel[i - 1] + s_late * ew_acc[i] + ew_acc[i - 1]) / 2.0);
            ud_vel.push((ud_vel[i - 1] + s_late * ud_acc[i] + ud_acc[i - 1]) / 2.0);
        }
    }

    // 速度から変位を求める
    for i in 0..ns_vel.len() {
        if i == 0 {
            ns_disp.push(0.0);
            ew_disp.push(0.0);
            ud_disp.push(0.0);
        } else {
            ns_disp.push((ns_disp[i - 1] + s_late * ns_vel[i] + ns_vel[i - 1]) / 2.0);
            ew_disp.push((ew_disp[i - 1] + s_late * ew_vel[i] + ew_vel[i - 1]) / 2.0);
            ud_disp.push((ud_disp[i - 1] + s_late * ud_vel[i] + ud_vel[i - 1]) / 2.0);
        }
    }

    let plot_data = PlotData {
        title: data.title.clone(),
        rate: data.rate.clone(),
        unit: data.unit.clone(),
        time: data.time.clone(),
        ns_acc: data.ns_acc.clone(),
        ew_acc: data.ew_acc.clone(),
        ud_acc: data.ud_acc.clone(),
        ns_vel: ns_vel,
        ew_vel: ew_vel,
        ud_vel: ud_vel,
        ns_disp: ns_disp,
        ew_disp: ew_disp,
        ud_disp: ud_disp,
    };

    let _ = data_visualization::main(&plot_data);
}