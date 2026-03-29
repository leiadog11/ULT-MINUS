use super::*;

mod checkattack;
mod specialhi;
mod specialn;

pub fn install() {
    checkattack::install();
    specialhi::install();
    specialn::install();
}