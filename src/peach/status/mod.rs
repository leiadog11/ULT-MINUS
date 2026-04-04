use super::*;

mod checkattack;
mod specialhi;
mod speciallw;
mod specialn;

pub fn install() {
    checkattack::install();
    specialhi::install();
    speciallw::install();
    specialn::install();
}