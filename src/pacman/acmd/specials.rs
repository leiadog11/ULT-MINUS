use super::*;

//--------------------SPECIALS-----------------------

// SIDE B DASH
unsafe extern "C" fn pacman_specialsdash(agent: &mut L2CAgentBase) {
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_PACMAN_STATUS_SPECIAL_S_WORK_FLAG_EAT_POWER_ESA) {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("pizzapacman"), 1.0, 50, 108, 0, 50, 6.0, 0.0, 2.8, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            wait(agent.lua_state_agent, 6.0);
            if smash::app::stage::get_stage_id() == 0x0 {
                //Battlefield
                if PostureModule::pos_x(agent.module_accessor) >= 170.0 {
                    PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: -170.0 , y: PostureModule::pos_y(agent.module_accessor), z: PostureModule::pos_z(agent.module_accessor)});
                    AttackModule::clear_all(agent.module_accessor);
                    GroundModule::set_collidable(agent.module_accessor, false);
                }
                else if PostureModule::pos_x(agent.module_accessor) <= -170.0 {
                    PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: 170.0 , y: PostureModule::pos_y(agent.module_accessor), z: PostureModule::pos_z(agent.module_accessor)});
                    AttackModule::clear_all(agent.module_accessor);
                    GroundModule::set_collidable(agent.module_accessor, false);
                }
                else if PostureModule::pos_y(agent.module_accessor) <= -80.0 {
                    PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: PostureModule::pos_x(agent.module_accessor) , y: 130.0, z: PostureModule::pos_z(agent.module_accessor)});
                    AttackModule::clear_all(agent.module_accessor);
                    GroundModule::set_collidable(agent.module_accessor, false);
                }
                else if PostureModule::pos_y(agent.module_accessor) >= 130.0 {
                    PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: PostureModule::pos_x(agent.module_accessor) , y: -80.0, z: PostureModule::pos_z(agent.module_accessor)});
                    AttackModule::clear_all(agent.module_accessor);
                    GroundModule::set_collidable(agent.module_accessor, false);
                }
            }
            else if smash::app::stage::get_stage_id() == 0x3 {
                //FD
                if PostureModule::pos_x(agent.module_accessor) >= 180.0 {
                    PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: -180.0 , y: PostureModule::pos_y(agent.module_accessor), z: PostureModule::pos_z(agent.module_accessor)});
                    AttackModule::clear_all(agent.module_accessor);
                    GroundModule::set_collidable(agent.module_accessor, false);
                }
                else if PostureModule::pos_x(agent.module_accessor) <= -180.0 {
                    PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: 180.0 , y: PostureModule::pos_y(agent.module_accessor), z: PostureModule::pos_z(agent.module_accessor)});
                    AttackModule::clear_all(agent.module_accessor);
                    GroundModule::set_collidable(agent.module_accessor, false);
                }
                else if PostureModule::pos_y(agent.module_accessor) <= -60.0 {
                    PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: PostureModule::pos_x(agent.module_accessor) , y: 130.0, z: PostureModule::pos_z(agent.module_accessor)});
                    AttackModule::clear_all(agent.module_accessor);
                    GroundModule::set_collidable(agent.module_accessor, false);
                }
                else if PostureModule::pos_y(agent.module_accessor) >= 130.0 {
                    PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: PostureModule::pos_x(agent.module_accessor) , y: -60.0, z: PostureModule::pos_z(agent.module_accessor)});
                    AttackModule::clear_all(agent.module_accessor);
                    GroundModule::set_collidable(agent.module_accessor, false);
                }
            }
            else if smash::app::stage::get_stage_id() == 0x15B {
                //Small Battlefield
                if PostureModule::pos_x(agent.module_accessor) >= 170.0 {
                    PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: -170.0 , y: PostureModule::pos_y(agent.module_accessor), z: PostureModule::pos_z(agent.module_accessor)});
                    AttackModule::clear_all(agent.module_accessor);
                    GroundModule::set_collidable(agent.module_accessor, false);
                }
                else if PostureModule::pos_x(agent.module_accessor) <= -170.0 {
                    PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: 170.0 , y: PostureModule::pos_y(agent.module_accessor), z: PostureModule::pos_z(agent.module_accessor)});
                    AttackModule::clear_all(agent.module_accessor);
                    GroundModule::set_collidable(agent.module_accessor, false);
                }
                else if PostureModule::pos_y(agent.module_accessor) <= -80.0 {
                    PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: PostureModule::pos_x(agent.module_accessor) , y: 130.0, z: PostureModule::pos_z(agent.module_accessor)});
                    AttackModule::clear_all(agent.module_accessor);
                    GroundModule::set_collidable(agent.module_accessor, false);
                }
                else if PostureModule::pos_y(agent.module_accessor) >= 130.0 {
                    PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: PostureModule::pos_x(agent.module_accessor) , y: -80.0, z: PostureModule::pos_z(agent.module_accessor)});
                    AttackModule::clear_all(agent.module_accessor);
                    GroundModule::set_collidable(agent.module_accessor, false);
                }
            }
            else if smash::app::stage::get_stage_id() == 0x5F {
                //Smashville
                if PostureModule::pos_x(agent.module_accessor) >= 160.0 {
                    PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: -160.0 , y: PostureModule::pos_y(agent.module_accessor), z: PostureModule::pos_z(agent.module_accessor)});
                    AttackModule::clear_all(agent.module_accessor);
                    GroundModule::set_collidable(agent.module_accessor, false);
                }
                else if PostureModule::pos_x(agent.module_accessor) <= -160.0 {
                    PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: 160.0 , y: PostureModule::pos_y(agent.module_accessor), z: PostureModule::pos_z(agent.module_accessor)});
                    AttackModule::clear_all(agent.module_accessor);
                    GroundModule::set_collidable(agent.module_accessor, false);
                }
                else if PostureModule::pos_y(agent.module_accessor) <= -50.0 {
                    PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: PostureModule::pos_x(agent.module_accessor) , y: 140.0, z: PostureModule::pos_z(agent.module_accessor)});
                    AttackModule::clear_all(agent.module_accessor);
                    GroundModule::set_collidable(agent.module_accessor, false);
                }
                else if PostureModule::pos_y(agent.module_accessor) >= 140.0 {
                    PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: PostureModule::pos_x(agent.module_accessor) , y: -50.0, z: PostureModule::pos_z(agent.module_accessor)});
                    AttackModule::clear_all(agent.module_accessor);
                    GroundModule::set_collidable(agent.module_accessor, false);
                }
            }
            else if smash::app::stage::get_stage_id() == 0x6B {
                //PS2
                if PostureModule::pos_x(agent.module_accessor) >= 180.0 {
                    PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: -180.0 , y: PostureModule::pos_y(agent.module_accessor), z: PostureModule::pos_z(agent.module_accessor)});
                    AttackModule::clear_all(agent.module_accessor);
                    GroundModule::set_collidable(agent.module_accessor, false);
                }
                else if PostureModule::pos_x(agent.module_accessor) <= -180.0 {
                    PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: 180.0 , y: PostureModule::pos_y(agent.module_accessor), z: PostureModule::pos_z(agent.module_accessor)});
                    AttackModule::clear_all(agent.module_accessor);
                    GroundModule::set_collidable(agent.module_accessor, false);
                }
                else if PostureModule::pos_y(agent.module_accessor) <= -50.0 {
                    PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: PostureModule::pos_x(agent.module_accessor) , y: 130.0, z: PostureModule::pos_z(agent.module_accessor)});
                    AttackModule::clear_all(agent.module_accessor);
                    GroundModule::set_collidable(agent.module_accessor, false);
                }
                else if PostureModule::pos_y(agent.module_accessor) >= 130.0 {
                    PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: PostureModule::pos_x(agent.module_accessor) , y: -50.0, z: PostureModule::pos_z(agent.module_accessor)});
                    AttackModule::clear_all(agent.module_accessor);
                    GroundModule::set_collidable(agent.module_accessor, false);
                }
            }
            else if smash::app::stage::get_stage_id() == 0xF2 {
                //Kalos
                if PostureModule::pos_x(agent.module_accessor) >= 165.0 {
                    PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: -165.0 , y: PostureModule::pos_y(agent.module_accessor), z: PostureModule::pos_z(agent.module_accessor)});
                    AttackModule::clear_all(agent.module_accessor);
                    GroundModule::set_collidable(agent.module_accessor, false);
                }
                else if PostureModule::pos_x(agent.module_accessor) <= -165.0 {
                    PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: 165.0 , y: PostureModule::pos_y(agent.module_accessor), z: PostureModule::pos_z(agent.module_accessor)});
                    AttackModule::clear_all(agent.module_accessor);
                    GroundModule::set_collidable(agent.module_accessor, false);
                }
                else if PostureModule::pos_y(agent.module_accessor) <= -60.0 {
                    PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: PostureModule::pos_x(agent.module_accessor) , y: 130.0, z: PostureModule::pos_z(agent.module_accessor)});
                    AttackModule::clear_all(agent.module_accessor);
                    GroundModule::set_collidable(agent.module_accessor, false);
                }
                else if PostureModule::pos_y(agent.module_accessor) >= 130.0 {
                    PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: PostureModule::pos_x(agent.module_accessor) , y: -60.0, z: PostureModule::pos_z(agent.module_accessor)});
                    AttackModule::clear_all(agent.module_accessor);
                    GroundModule::set_collidable(agent.module_accessor, false);
                }
            }
            else if smash::app::stage::get_stage_id() == 0x101 {
                //Town and City
                if PostureModule::pos_x(agent.module_accessor) >= 160.0 {
                    PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: -160.0 , y: PostureModule::pos_y(agent.module_accessor), z: PostureModule::pos_z(agent.module_accessor)});
                    AttackModule::clear_all(agent.module_accessor);
                    GroundModule::set_collidable(agent.module_accessor, false);
                }
                else if PostureModule::pos_x(agent.module_accessor) <= -160.0 {
                    PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: 160.0 , y: PostureModule::pos_y(agent.module_accessor), z: PostureModule::pos_z(agent.module_accessor)});
                    AttackModule::clear_all(agent.module_accessor);
                    GroundModule::set_collidable(agent.module_accessor, false);
                }
                else if PostureModule::pos_y(agent.module_accessor) <= -80.0 {
                    PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: PostureModule::pos_x(agent.module_accessor) , y: 105.0, z: PostureModule::pos_z(agent.module_accessor)});
                    AttackModule::clear_all(agent.module_accessor);
                    GroundModule::set_collidable(agent.module_accessor, false);
                }
                else if PostureModule::pos_y(agent.module_accessor) >= 105.0 {
                    PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: PostureModule::pos_x(agent.module_accessor) , y: -80.0, z: PostureModule::pos_z(agent.module_accessor)});
                    AttackModule::clear_all(agent.module_accessor);
                    GroundModule::set_collidable(agent.module_accessor, false);
                }
            }
            else if smash::app::stage::get_stage_id() == 0x169 {
                //Hollow Bastion
                if PostureModule::pos_x(agent.module_accessor) >= 180.0 {
                    PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: -180.0 , y: PostureModule::pos_y(agent.module_accessor), z: PostureModule::pos_z(agent.module_accessor)});
                    AttackModule::clear_all(agent.module_accessor);
                    GroundModule::set_collidable(agent.module_accessor, false);
                }
                else if PostureModule::pos_x(agent.module_accessor) <= -180.0 {
                    PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: 180.0 , y: PostureModule::pos_y(agent.module_accessor), z: PostureModule::pos_z(agent.module_accessor)});
                    AttackModule::clear_all(agent.module_accessor);
                    GroundModule::set_collidable(agent.module_accessor, false);
                }
                else if PostureModule::pos_y(agent.module_accessor) <= -60.0 {
                    PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: PostureModule::pos_x(agent.module_accessor) , y: 130.0, z: PostureModule::pos_z(agent.module_accessor)});
                    AttackModule::clear_all(agent.module_accessor);
                    GroundModule::set_collidable(agent.module_accessor, false);
                }
                else if PostureModule::pos_y(agent.module_accessor) >= 130.0 {
                    PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: PostureModule::pos_x(agent.module_accessor) , y: -60.0, z: PostureModule::pos_z(agent.module_accessor)});
                    AttackModule::clear_all(agent.module_accessor);
                    GroundModule::set_collidable(agent.module_accessor, false);
                }
            }
            else {
                macros::ATTACK(agent, 0, 0, Hash40::new("pizzapacman"), 1.0, 361, 66, 0, 40, 6.0, 0.0, 2.8, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
            }
        }
        wait(agent.lua_state_agent, 6.0);
        if macros::is_excute(agent) {
            GroundModule::set_collidable(agent.module_accessor, true);
        }
    }
}

pub fn install() {
    Agent::new("pacman")
        .game_acmd("game_specialsdash", pacman_specialsdash, Low)
        
        .install();
}