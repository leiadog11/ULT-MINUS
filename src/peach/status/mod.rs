use super::*;

mod check_attack;
mod special_lw;
mod special_n;

pub fn install() {
    check_attack::install();
    special_lw::install();
    special_n::install();
}