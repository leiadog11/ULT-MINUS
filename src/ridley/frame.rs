use super::*;

// OPFF
pub unsafe extern "C" fn ridley_frame(fighter: &mut L2CFighterCommon) {
    unsafe { 

    }
}

// ON START
pub unsafe extern "C" fn ridley_start(fighter: &mut L2CFighterCommon) {
    unsafe { 

    }
}

pub fn install() {
    Agent::new("ridley")
        .on_line(Main, ridley_frame)
        .on_start(ridley_start)
        .install();
}