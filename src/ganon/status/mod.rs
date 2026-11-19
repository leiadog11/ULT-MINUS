use super::*;

mod attack_hi4;
mod attack_lw4;
mod attack_s4;
mod checkdamage;
mod specialn;
mod speciallw;
mod specialhi;
mod specialhi2;

pub fn install() {
    attack_hi4::install();
    attack_lw4::install();
    attack_s4::install();
    checkdamage::install();
    specialn::install();
    speciallw::install();
    specialhi::install();
    specialhi2::install();
}