use super::*;

// OPFF
pub unsafe extern "C" fn jack_frame(fighter: &mut L2CFighterCommon) {
    unsafe { 

    }
}

// ON START
pub unsafe extern "C" fn jack_start(fighter: &mut L2CFighterCommon) {
    unsafe { 

    }
}

pub fn install() {
    Agent::new("jack")
        .on_line(Main, jack_frame)
        .on_start(jack_start)
        .install();
}