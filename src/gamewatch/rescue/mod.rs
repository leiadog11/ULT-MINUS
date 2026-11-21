use super::*;

mod acmd;
mod attack_air_b;
mod attack_hi3;

pub fn install() {
    acmd::install();
    attack_air_b::install();
    attack_hi3::install();
}