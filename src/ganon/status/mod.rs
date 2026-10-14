use super::*;

mod checkdamage;
mod specialn;
mod speciallw;
mod specialhi;
mod specialhi2;

pub fn install() {
    checkdamage::install();
    specialn::install();
    speciallw::install();
    specialhi::install();
    specialhi2::install();
}