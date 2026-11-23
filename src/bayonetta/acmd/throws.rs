use super::*;

//GRAB
unsafe extern "C" fn bayonetta_catch(agent: &mut L2CAgentBase) {

}

pub fn install() {
    Agent::new("bayonetta")
        .game_acmd("game_catch", bayonetta_catch, Low)
        .game_acmd("game_catchdash", bayonetta_catch, Low)
        .game_acmd("game_catchturn", bayonetta_catch, Low)
    .install();
}