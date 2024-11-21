use super::*;

pub mod acmd;
pub mod frame;
pub mod status;

pub fn install() {
    acmd::install();
    frame::install();
    status::install();
}