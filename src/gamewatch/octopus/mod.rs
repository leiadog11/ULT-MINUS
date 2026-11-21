use super::*;

mod attack_air_n;
mod acmd;

pub fn install() {
    attack_air_n::install();
    acmd::install();
}