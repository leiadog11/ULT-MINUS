use super::*;

// OPFF
pub unsafe extern "C" fn agent_frame(fighter: &mut L2CFighterCommon) {
    unsafe { 

    }
}

// ON START
pub unsafe extern "C" fn agent_start(fighter: &mut L2CFighterCommon) {
    unsafe { 

    }
}

pub fn install() {
    Agent::new("mario")
        .on_line(Main, agent_frame)
        .on_start(agent_start)
        .install();
}