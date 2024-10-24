use super::*;

mod specialn;
mod speciallw;
mod specialhi;
mod specialhi2;

pub fn install() {
    specialn::install();
    speciallw::install();
    specialhi::install();
    specialhi2::install();
}