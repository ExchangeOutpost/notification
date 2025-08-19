mod exchange_outpost;
use crate::exchange_outpost::{FinData, send_notification};
use extism_pdk::{encoding, plugin_fn, FnResult, Json, ToBytes};
use serde::Serialize;

#[derive(Serialize, ToBytes)]
#[encoding(Json)]
pub struct Output {}

#[plugin_fn]
pub fn run(fin_data: FinData) -> FnResult<Output> {
    send_notification("this is a test".into());
    Ok(Output {})
}