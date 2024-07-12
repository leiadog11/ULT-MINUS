use super::*;

mod checkattack;
mod specialhi;

pub fn install() {
    checkattack::install();
    specialhi::install();
}