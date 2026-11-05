use super::*;

mod attackhi3;
mod checkattack;

pub fn install() {
    checkattack::install();
    attackhi3::install();
}