use super::*;

//GRAB
unsafe extern "C" fn bayonetta_catch(agent: &mut L2CAgentBase) {

}

//DASH GRAB
unsafe extern "C" fn bayonetta_catchdash(agent: &mut L2CAgentBase) {

}

//PIVOT GRAB
unsafe extern "C" fn bayonetta_catchturn(agent: &mut L2CAgentBase) {
    
}

pub fn install() {
    Agent::new("bayonetta")
        .game_acmd("game_catch", bayonetta_catch, Low)

        .game_acmd("game_catchdash", bayonetta_catchdash, Low)

        .game_acmd("game_catchturn", bayonetta_catchturn, Low)

        .install();
}