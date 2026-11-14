use super::*;

mod attackhi3;
mod checkattack;
mod specialhi;

pub fn install() {
    attackhi3::install();
    checkattack::install();
    specialhi::install();
}