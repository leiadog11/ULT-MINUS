use super::*;

mod acmd;
mod attackairb;

pub fn install() {
    acmd::install();
    attackairb::install();
}