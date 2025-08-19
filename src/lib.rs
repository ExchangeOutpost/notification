mod exchange_outpost;
use crate::exchange_outpost::{FinData, schedule_webhook};
use extism_pdk::{encoding, plugin_fn, FnResult, Json, ToBytes};
use serde::Serialize;

#[derive(Serialize, ToBytes)]
#[encoding(Json)]
pub struct Output {}

#[plugin_fn]
pub fn run(fin_data: FinData) -> FnResult<Output> {
    schedule_webhook("/", "{}")?;
    Ok(Output {})
}