use crate::csv_parser::ParseData;
use crate::data_visualization;

pub struct PlotData {
    pub file_name: String,
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
    pub ns_dist: Vec<f64>,
    pub ew_dist: Vec<f64>,
    pub ud_dist: Vec<f64>,
}

// 台形公式を用いた加速度数値積分
// 変位(cm) 速度(cm/sec) 加速度(gal) に変換
// Convert acceleration (in gal) to velocity (in cm/sec)
fn acc_to_vel(ns_acc: &[f64], ew_acc: &[f64], ud_acc: &[f64]) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    const G: f64 = 0.004937561699;
    const A1: f64 = -2.974867761716;
    const A2: f64 = 2.950050339269;
    const A3: f64 = -0.975180618018;
    const B0: f64 = 1.0;
    const B1: f64 = -1.0;
    const B2: f64 = -1.0;
    const B3: f64 = 1.0;

    let mut ns_vel: Vec<f64> = Vec::new(); // NS Velocity
    let mut ew_vel: Vec<f64> = Vec::new(); // EW Velocity
    let mut ud_vel: Vec<f64> = Vec::new(); // UD Velocity

    for i in 0..ns_acc.len() {
        if i <= 4 {
            ns_vel.push(0.0);
            ew_vel.push(0.0);
            ud_vel.push(0.0);
        } else {
            //     v(t) = G×{B0×a(t)+B1×a(t-1)+B2×a(t-2)+B3×a(t-3)} - {A1×v(t-1)+A2×v(t-2)+A3×v(t-3)}
            ns_vel.push(
                G * (B0 * ns_acc[i] + B1 * ns_acc[i - 1] + B2 * ns_acc[i - 2] + B3 * ns_acc[i - 3])
                    - (A1 * ns_vel[i - 1] + A2 * ns_vel[i - 2] + A3 * ns_vel[i - 3]),
            );
            ew_vel.push(
                G * (B0 * ew_acc[i] + B1 * ew_acc[i - 1] + B2 * ew_acc[i - 2] + B3 * ew_acc[i - 3])
                    - (A1 * ew_vel[i - 1] + A2 * ew_vel[i - 2] + A3 * ew_vel[i - 3]),
            );
            ud_vel.push(
                G * (B0 * ud_acc[i] + B1 * ud_acc[i - 1] + B2 * ud_acc[i - 2] + B3 * ud_acc[i - 3])
                    - (A1 * ud_vel[i - 1] + A2 * ud_vel[i - 2] + A3 * ud_vel[i - 3]),
            );
        }
    }
    (ns_vel, ew_vel, ud_vel)
}

// Convert velocity (in cm/sec) to displacement (in cm)
fn vel_to_disp(
    ns_vel: &[f64],
    ew_vel: &[f64],
    ud_vel: &[f64],
    dt: f64,
) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    const H: f64 = 1.0;
    const C1: f64 = -1.988438073558305;
    const C2: f64 = 0.9885471048650272;
    const D0: f64 = 0.00002485615736514583;
    const D1: f64 = 0.00004971231473029166;
    const D2: f64 = 0.00002485615736514583;

    let mut ns_disp: Vec<f64> = Vec::new(); // NS Displacement
    let mut ew_disp: Vec<f64> = Vec::new(); // EW Displacement
    let mut ud_disp: Vec<f64> = Vec::new(); // UD Displacement

    for i in 0..ns_vel.len() {
        if i <= 4 {
            ns_disp.push(0.0);
            ew_disp.push(0.0);
            ud_disp.push(0.0);
        } else {
            // 　　d(t) = H×{D0×a(t)+D1×a(t-1)+D2×a(t-2)} - {C1×d(t-1)+C2×d(t-2)}
            ns_disp.push(
                H * (D0 * ns_vel[i] + D1 * ns_vel[i - 1] + D2 * ns_vel[i - 2])
                    - (C1 * ns_disp[i - 1] + C2 * ns_disp[i - 2]),
            );
            ew_disp.push(
                H * (D0 * ew_vel[i] + D1 * ew_vel[i - 1] + D2 * ew_vel[i - 2])
                    - (C1 * ew_disp[i - 1] + C2 * ew_disp[i - 2]),
            );
            ud_disp.push(
                H * (D0 * ud_vel[i] + D1 * ud_vel[i - 1] + D2 * ud_vel[i - 2])
                    - (C1 * ud_disp[i - 1] + C2 * ud_disp[i - 2]),
            );
        }
    }
    let ns_dist = ns_disp.iter().map(|&d| d * dt).collect();
    let ew_dist = ew_disp.iter().map(|&d| d * dt).collect();
    let ud_dist = ud_disp.iter().map(|&d| d * dt).collect();
    (ns_dist, ew_dist, ud_dist)
    // (ns_disp, ew_disp, ud_disp)
}

pub fn main(data: &ParseData) {
    let dt = 0.01; //サンプリングレート

    let (ns_vel, ew_vel, ud_vel) = acc_to_vel(&data.ns_acc, &data.ew_acc, &data.ud_acc);

    let (ns_dist, ew_dist, ud_dist) = vel_to_disp(&ns_vel, &ew_vel, &ud_vel, dt);

    // println!("Acceleration: {:?}", acc);
    // println!("Velocity: {:?}", vel);
    // println!("Displacement: {:?}", disp);

    let plot_data = PlotData {
        file_name: data.file_name.clone(),
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
        ns_dist: ns_dist,
        ew_dist: ew_dist,
        ud_dist: ud_dist,
    };

    let _ = data_visualization::main(&plot_data);
}
