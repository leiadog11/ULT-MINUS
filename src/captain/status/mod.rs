use super::*;

mod checkattack;
mod specialn;
mod specialhi;

pub fn install() {
    checkattack::install();
    specialn::install();
    specialhi::install();
}