use super::*;

mod attackdash;
mod specialn;

pub fn install() {
    attackdash::install();
    specialn::install();
}