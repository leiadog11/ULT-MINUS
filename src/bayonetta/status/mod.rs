use super::*;

mod attackhi3;
mod checkattack;
mod specialhi;
mod specialn;

pub fn install() {
    attackhi3::install();
    checkattack::install();
    specialhi::install();
    specialn::install();
}