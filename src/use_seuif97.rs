extern crate seuif97;

use seuif97::*;

pub struct SteamProps {
    pub d: f64,     // 2. Density, kg/m³
    pub v: f64,     // 3. Specific Volume, m³/kg
    pub h: f64,     // 4. Specific enthalpy, kJ/kg
    pub s: f64,     // 5. Specific entropy, kJ/(kg·K)
    pub u: f64,     // 7. Specific internal energy, kJ/kg
    pub dv: f64,    // 24. Dynamic viscosity, Pa·s
    pub kv: f64,    // 25. Kinematic viscosity, m2/s
    pub k : f64,    // 26. Thermal conductivity, W/(m·K)
    pub td: f64,    // 27. Thermal diffusivity, m2/s
    pub st: f64,    // 29. Surface tension, N/m
    pub lat: f64,   // cal. property, Latent Heat
}

impl SteamProps {
    pub fn new() -> SteamProps{
        SteamProps {
            d: -999.0,
            v: -999.0,
            h: -999.0,
            s: -999.0,
            u: -999.0,
            dv: -999.0,
            kv: -999.0,
            k: -999.0,
            td: -999.0,
            st: -999.0,
            lat: -999.0,
        }
    }
}

pub fn call_seuif(p: f64, t: f64, mode:u32) -> SteamProps {
    // p: f64  // 壓力, 單位 MPa
    // t: f64  // 溫度, 單位攝氏度
    // mode: u32  // (10, 20, 30, 40)

    return match mode {
        10 => sat_steam_by_temp(t),
        20 => sat_steam_by_pres(p),
        30 => steam(p, t),
        40 => water(p,t),
        _ => SteamProps::new(),
    };
}

fn sat_steam_by_temp(t: f64) -> SteamProps {
    let mut sp= SteamProps:: new();

    sp.d = tx(t, 1.0 , 2);  // 計算在給定溫度下的密度
    sp.v = tx(t, 1.0, 3);   // 計算在給定溫度下的比容
    sp.h = tx(t, 1.0, 4);   // 計算在給定溫度下的比焓
    sp.s = tx(t, 1.0,5);    // 計算在給定溫度下的比熵
    sp.u = tx(t, 1.0, 7);   // 計算在給定溫度下的北內能
    sp.dv = tx(t, 1.0,24);  // 計算在給定溫度下的靜黏度
    sp.kv = tx(t, 1.0,25);  // 計算在給定溫度下的動黏度
    sp.k = tx(t, 1.0,26);   // 計算在給定溫度下的熱傳導係數
    sp.td = tx(t, 1.0, 27); // 計算在給定溫度下的熱擴散係數
    sp.st = tx(t, 1.0, 29);     // 計算在給定溫度下的表面張力
    sp.lat = tx(t, 1.0, 4) - tx(t, 0.0, 4);     // 計算在給定溫度下的潛熱 (蒸發熱)

    sp
}

fn sat_steam_by_pres(p: f64) -> SteamProps {
    let mut sp= SteamProps:: new();

    sp.d = px(p, 1.0 , 2);  // 計算在給定壓力下的密度
    sp.v = px(p, 1.0, 3);   // 計算在給定壓力下的比容
    sp.h = px(p, 1.0, 4);   // 計算在給定壓力下的比焓
    sp.s = px(p, 1.0,5);    // 計算在給定壓力下的比熵
    sp.u = px(p, 1.0, 7);   // 計算在給定壓力下的北內能
    sp.dv = px(p, 1.0,24);  // 計算在給定壓力下的靜黏度
    sp.kv = px(p, 1.0,25);  // 計算在給定壓力下的動黏度
    sp.k = px(p, 1.0,26);   // 計算在給定壓力下的熱傳導係數
    sp.td = px(p, 1.0, 27); // 計算在給定壓力下的熱擴散係數
    sp.st = px(p, 1.0, 29);     // 計算在給定壓力下的表面張力
    sp.lat = px(p, 1.0, 4) - px(p, 0.0, 4);     // 計算在給定壓力下的潛熱 (蒸發熱)

    sp
}

fn steam(p: f64, t: f64) -> SteamProps {
    let mut sp= SteamProps:: new();

    sp.d = pt(p, t, 2); // 計算在給定壓力和溫度下的密度
    sp.v = pt(p, t, 3); // 計算在給定壓力和溫度下的比容
    sp.h = pt(p, t, 4); // 計算在給定壓力和溫度下的比焓
    sp.s = pt(p, t, 5); // 計算在給定壓力和溫度下的比熵
    sp.dv = pt(p, t, 24);  // 計算在給定壓力和溫度下的靜黏度
    sp.kv = pt(p, t, 25);  // 計算在給定壓力和溫度下的動黏度
    sp.k = pt(p, t, 26);   // 計算在給定壓力和溫度下的熱傳導係數
    sp.td = pt(p, t, 27);  // 計算在給定壓力和溫度下的熱擴散係數
    sp.st = pt(p, t, 29);  // 計算在給定壓力和溫度下的表面張力

    sp
}

fn water(p: f64, t: f64) -> SteamProps {
    let mut sp= SteamProps:: new();

    sp.d = pt(p, t, 2); // 計算在給定壓力和溫度下的密度
    sp.v = pt(p, t, 3); // 計算在給定壓力和溫度下的比容
    sp.h = pt(p, t, 4); // 計算在給定壓力和溫度下的比焓
    sp.s = pt(p, t, 5); // 計算在給定壓力和溫度下的比熵
    sp.dv = pt(p, t, 24);  // 計算在給定壓力和溫度下的靜黏度
    sp.kv = pt(p, t, 25);  // 計算在給定壓力和溫度下的動黏度
    sp.k = pt(p, t, 26);   // 計算在給定壓力和溫度下的熱傳導係數
    sp.td = pt(p, t, 27);  // 計算在給定壓力和溫度下的熱擴散係數
    sp.st = pt(p, t, 29);  // 計算在給定壓力和溫度下的表面張力

    sp
}