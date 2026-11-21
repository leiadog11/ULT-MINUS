use super::*;

mod check_attack;
mod special_hi;

pub fn install() {
    check_attack::install();
    special_hi::install();
}