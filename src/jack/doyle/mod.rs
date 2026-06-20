use super::*;

mod acmd;
mod specials;

pub fn install() {
    acmd::install();
    specials::install();
}