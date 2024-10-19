use super::*;

mod checkattack;
mod specialn;
mod specialhi;
mod speciallw;

pub fn install() {
    checkattack::install();
    specialn::install();
    specialhi::install();
    speciallw::install();
}