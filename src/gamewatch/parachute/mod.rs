use super::*;

mod acmd;
mod attacks4;

pub fn install() {
    acmd::install();
    attacks4::install();
}