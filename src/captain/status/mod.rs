use super::*;

mod checkattack;
mod checkdamage;
mod specialn;
mod specialhi;

pub fn install() {
    checkattack::install();
    checkdamage::install();
    specialn::install();
    specialhi::install();
}