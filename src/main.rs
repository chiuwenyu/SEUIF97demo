pub mod use_seuif97;
use use_seuif97::SteamProps;
use use_seuif97::call_seuif;

fn main() {
    // saturated steam test data
    let p: f64 = 0.00; // 壓力, 單位 MPa
    let t: f64 = 100.0; // 溫度, 單位攝氏度
    let res = call_seuif(p, t, 10);


    // super heating steam test data
    // let p: f64 = 16.10; // 壓力, 單位 MPa
    // let t: f64 = 535.10; // 溫度, 單位攝氏度
    // let res = call_seuif(p, t, 30);

    // water steam test data
    // let p: f64 = 0.101325; // 壓力, 單位 MPa
    // let t: f64 = 90.0; // 溫度, 單位攝氏度
    // let res = call_seuif(p, t, 40);

    println!("在壓力 {:.4} MPa 和溫度 {:.4} °C 下:", p, t);
    println!("密度 d = {:.4} kg/m³", res.d);
    println!("比容 v = {:.4} m³/kg", res.v);
    println!("比焓 h = {:.4} kJ/kg", res.h);
    println!("比熵 s = {:.4} kJ/(kg·K)", res.s);
    println!("比內能 u = {:.4} kJ/kg", res.u);
    println!("靜黏度 dv = {:.4} cP", res.dv * 1000.0);
    println!("動黏度 kv = {:.8} m2/s", res.kv);
    println!("熱傳導係數 k = {:.8} W/(m·K)", res.k);
    println!("熱擴散係數 td = {:.8} m2/s", res.td);
    println!("表面張力 st = {:.8} N/m", res.st);
    println!("潛熱 lat = {:.8} kJ/kg", res.lat);
}