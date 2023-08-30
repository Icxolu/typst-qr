use qrcode_rs::{render::svg, QrCode};
use wasm_minimal_protocol::*;

initiate_protocol!();

#[derive(minicbor::Decode)]
struct Options<'a> {
    #[b(0)]
    data: &'a str,
    #[b(1)]
    dark_color: &'a str,
    #[b(2)]
    light_color: &'a str,
    #[n(3)]
    has_quiet_zone: bool,
}

#[wasm_func]
pub fn qr(options: &[u8]) -> Result<Vec<u8>, String> {
    let Options {
        data,
        dark_color,
        light_color,
        has_quiet_zone,
    } = minicbor::decode(options).map_err(|e| e.to_string())?;

    let code = QrCode::new(data).map_err(|e| e.to_string())?;
    Ok(code
        .render::<svg::Color>()
        .dark_color(svg::Color(&dark_color))
        .light_color(svg::Color(&light_color))
        .quiet_zone(has_quiet_zone)
        .build()
        .into())
}
