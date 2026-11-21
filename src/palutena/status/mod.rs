use super::*;

mod check_damage;
mod special_n;
mod special_s;
mod special_lw;
mod special_hi;

pub fn install() {
    check_damage::install();
    special_n::install();
    special_lw::install();
    special_s::install();
    special_hi::install();
}