use super::*;

pub mod status;
pub mod acmd;

pub fn install() {
    status::install();
    acmd::install();
}