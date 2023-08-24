use qrcode_rs::{render::svg, QrCode};
use wasm_minimal_protocol::*;

initiate_protocol!();

#[wasm_func]
pub fn qr(
    data: &[u8],
    dark_color: &[u8],
    light_color: &[u8],
    quiet_zone: &[u8],
) -> Result<Vec<u8>, String> {
    let code = QrCode::new(data).map_err(|e| e.to_string())?;

    Ok(code
        .render::<svg::Color>()
        .dark_color(svg::Color(
            std::str::from_utf8(dark_color).map_err(|e| e.to_string())?,
        ))
        .light_color(svg::Color(
            std::str::from_utf8(light_color).map_err(|e| e.to_string())?,
        ))
        .quiet_zone(quiet_zone[0] != 0)
        .build()
        .into())
}
