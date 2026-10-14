use super::*;

mod checkattack;
mod checkdamage;
mod specialhi;

pub fn install() {
    checkattack::install();
    checkdamage::install();
    specialhi::install();
}