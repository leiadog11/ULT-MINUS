use super::*;

mod checkattack;
mod specialn;

pub fn install() {
    checkattack::install();
    specialn::install();
}