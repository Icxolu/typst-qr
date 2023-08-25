use qrcode_rs::{render::svg, QrCode};
use wasm_minimal_protocol::*;

initiate_protocol!();

#[derive(serde::Deserialize)]
struct Options<'a> {
    dark_color: &'a str,
    light_color: &'a str,
    has_quiet_zone: bool,
}

#[wasm_func]
pub fn qr(data: &[u8], options: &[u8]) -> Result<Vec<u8>, String> {
    let code = QrCode::new(data).map_err(|e| e.to_string())?;
    let Options {
        dark_color,
        light_color,
        has_quiet_zone,
    } = serde_json::from_slice::<Options>(options).map_err(|e| e.to_string())?;

    Ok(code
        .render::<svg::Color>()
        .dark_color(svg::Color(dark_color))
        .light_color(svg::Color(light_color))
        .quiet_zone(has_quiet_zone)
        .build()
        .into())
}
