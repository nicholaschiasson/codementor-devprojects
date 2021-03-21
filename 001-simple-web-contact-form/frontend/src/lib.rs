#![recursion_limit = "1024"]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::eval_order_dependence)]

mod app;
mod components;
mod routes;

use wasm_bindgen::prelude::*;

use app::App;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
	wasm_logger::init(wasm_logger::Config::default());
	yew::start_app::<App>();
	Ok(())
}
