use serde::Deserialize;

#[derive(Deserialize)]
pub struct GcdParameters {
    pub n: u64,
    pub m: u64,
}