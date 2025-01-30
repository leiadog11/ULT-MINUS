use super::*;

pub mod acmd;
pub mod frame;
pub mod status;
pub mod bigpacman;
pub mod firehydrant;
pub mod trampoline;

static mut UP_SMASH: [bool; 8] = [false; 8];
static mut DOWN_SMASH: [bool; 8] = [false; 8];
static mut ITEM_CHOICE: [i32; 8] = [0; 8];
static mut KEY_COOLDOWN: [i32; 8] = [0; 8];
static mut APPLE_COOLDOWN: [i32; 8] = [0; 8];
static mut MELON_COOLDOWN: [i32; 8] = [0; 8];
static mut GALAXIAN_COOLDOWN: [i32; 8] = [0; 8];
static mut BELL_COOLDOWN: [i32; 8] = [0; 8];

// SIDE B BLASTZONE CHECK
unsafe extern "C" fn blastzone_check(agent: &mut L2CAgentBase) { 
    let boma = agent.module_accessor;
    if smash::app::stage::get_stage_id() == 0x0 {
        //Battlefield
        if PostureModule::pos_x(boma) >= 170.0 {
            PostureModule::set_pos(boma, &Vector3f{ x: -170.0 , y: PostureModule::pos_y(boma), z: PostureModule::pos_z(boma)});
            AttackModule::clear_all(boma);
            GroundModule::set_collidable(boma, false);
        }
        else if PostureModule::pos_x(boma) <= -170.0 {
            PostureModule::set_pos(boma, &Vector3f{ x: 170.0 , y: PostureModule::pos_y(boma), z: PostureModule::pos_z(boma)});
            AttackModule::clear_all(boma);
            GroundModule::set_collidable(boma, false);
        }
        else if PostureModule::pos_y(boma) <= -80.0 {
            PostureModule::set_pos(boma, &Vector3f{ x: PostureModule::pos_x(boma) , y: 130.0, z: PostureModule::pos_z(boma)});
            AttackModule::clear_all(boma);
            GroundModule::set_collidable(boma, false);
        }
        else if PostureModule::pos_y(boma) >= 130.0 {
            PostureModule::set_pos(boma, &Vector3f{ x: PostureModule::pos_x(boma) , y: -80.0, z: PostureModule::pos_z(boma)});
            AttackModule::clear_all(boma);
            GroundModule::set_collidable(boma, false);
        }
    }
    else if smash::app::stage::get_stage_id() == 0x3 {
        //FD
        if PostureModule::pos_x(boma) >= 180.0 {
            PostureModule::set_pos(boma, &Vector3f{ x: -180.0 , y: PostureModule::pos_y(boma), z: PostureModule::pos_z(boma)});
            AttackModule::clear_all(boma);
            GroundModule::set_collidable(boma, false);
        }
        else if PostureModule::pos_x(boma) <= -180.0 {
            PostureModule::set_pos(boma, &Vector3f{ x: 180.0 , y: PostureModule::pos_y(boma), z: PostureModule::pos_z(boma)});
            AttackModule::clear_all(boma);
            GroundModule::set_collidable(boma, false);
        }
        else if PostureModule::pos_y(boma) <= -60.0 {
            PostureModule::set_pos(boma, &Vector3f{ x: PostureModule::pos_x(boma) , y: 130.0, z: PostureModule::pos_z(boma)});
            AttackModule::clear_all(boma);
            GroundModule::set_collidable(boma, false);
        }
        else if PostureModule::pos_y(boma) >= 130.0 {
            PostureModule::set_pos(boma, &Vector3f{ x: PostureModule::pos_x(boma) , y: -60.0, z: PostureModule::pos_z(boma)});
            AttackModule::clear_all(boma);
            GroundModule::set_collidable(boma, false);
        }
    }
    else if smash::app::stage::get_stage_id() == 0x15B {
        //Small Battlefield
        if PostureModule::pos_x(boma) >= 170.0 {
            PostureModule::set_pos(boma, &Vector3f{ x: -170.0 , y: PostureModule::pos_y(boma), z: PostureModule::pos_z(boma)});
            AttackModule::clear_all(boma);
            GroundModule::set_collidable(boma, false);
        }
        else if PostureModule::pos_x(boma) <= -170.0 {
            PostureModule::set_pos(boma, &Vector3f{ x: 170.0 , y: PostureModule::pos_y(boma), z: PostureModule::pos_z(boma)});
            AttackModule::clear_all(boma);
            GroundModule::set_collidable(boma, false);
        }
        else if PostureModule::pos_y(boma) <= -80.0 {
            PostureModule::set_pos(boma, &Vector3f{ x: PostureModule::pos_x(boma) , y: 130.0, z: PostureModule::pos_z(boma)});
            AttackModule::clear_all(boma);
            GroundModule::set_collidable(boma, false);
        }
        else if PostureModule::pos_y(boma) >= 130.0 {
            PostureModule::set_pos(boma, &Vector3f{ x: PostureModule::pos_x(boma) , y: -80.0, z: PostureModule::pos_z(boma)});
            AttackModule::clear_all(boma);
            GroundModule::set_collidable(boma, false);
        }
    }
    else if smash::app::stage::get_stage_id() == 0x5F {
        //Smashville
        if PostureModule::pos_x(boma) >= 160.0 {
            PostureModule::set_pos(boma, &Vector3f{ x: -160.0 , y: PostureModule::pos_y(boma), z: PostureModule::pos_z(boma)});
            AttackModule::clear_all(boma);
            GroundModule::set_collidable(boma, false);
        }
        else if PostureModule::pos_x(boma) <= -160.0 {
            PostureModule::set_pos(boma, &Vector3f{ x: 160.0 , y: PostureModule::pos_y(boma), z: PostureModule::pos_z(boma)});
            AttackModule::clear_all(boma);
            GroundModule::set_collidable(boma, false);
        }
        else if PostureModule::pos_y(boma) <= -50.0 {
            PostureModule::set_pos(boma, &Vector3f{ x: PostureModule::pos_x(boma) , y: 140.0, z: PostureModule::pos_z(boma)});
            AttackModule::clear_all(boma);
            GroundModule::set_collidable(boma, false);
        }
        else if PostureModule::pos_y(boma) >= 140.0 {
            PostureModule::set_pos(boma, &Vector3f{ x: PostureModule::pos_x(boma) , y: -50.0, z: PostureModule::pos_z(boma)});
            AttackModule::clear_all(boma);
            GroundModule::set_collidable(boma, false);
        }
    }
    else if smash::app::stage::get_stage_id() == 0x6B {
        //PS2
        if PostureModule::pos_x(boma) >= 180.0 {
            PostureModule::set_pos(boma, &Vector3f{ x: -180.0 , y: PostureModule::pos_y(boma), z: PostureModule::pos_z(boma)});
            AttackModule::clear_all(boma);
            GroundModule::set_collidable(boma, false);
        }
        else if PostureModule::pos_x(boma) <= -180.0 {
            PostureModule::set_pos(boma, &Vector3f{ x: 180.0 , y: PostureModule::pos_y(boma), z: PostureModule::pos_z(boma)});
            AttackModule::clear_all(boma);
            GroundModule::set_collidable(boma, false);
        }
        else if PostureModule::pos_y(boma) <= -50.0 {
            PostureModule::set_pos(boma, &Vector3f{ x: PostureModule::pos_x(boma) , y: 130.0, z: PostureModule::pos_z(boma)});
            AttackModule::clear_all(boma);
            GroundModule::set_collidable(boma, false);
        }
        else if PostureModule::pos_y(boma) >= 130.0 {
            PostureModule::set_pos(boma, &Vector3f{ x: PostureModule::pos_x(boma) , y: -50.0, z: PostureModule::pos_z(boma)});
            AttackModule::clear_all(boma);
            GroundModule::set_collidable(boma, false);
        }
    }
    else if smash::app::stage::get_stage_id() == 0xF2 {
        //Kalos
        if PostureModule::pos_x(boma) >= 165.0 {
            PostureModule::set_pos(boma, &Vector3f{ x: -165.0 , y: PostureModule::pos_y(boma), z: PostureModule::pos_z(boma)});
            AttackModule::clear_all(boma);
            GroundModule::set_collidable(boma, false);
        }
        else if PostureModule::pos_x(boma) <= -165.0 {
            PostureModule::set_pos(boma, &Vector3f{ x: 165.0 , y: PostureModule::pos_y(boma), z: PostureModule::pos_z(boma)});
            AttackModule::clear_all(boma);
            GroundModule::set_collidable(boma, false);
        }
        else if PostureModule::pos_y(boma) <= -60.0 {
            PostureModule::set_pos(boma, &Vector3f{ x: PostureModule::pos_x(boma) , y: 130.0, z: PostureModule::pos_z(boma)});
            AttackModule::clear_all(boma);
            GroundModule::set_collidable(boma, false);
        }
        else if PostureModule::pos_y(boma) >= 130.0 {
            PostureModule::set_pos(boma, &Vector3f{ x: PostureModule::pos_x(boma) , y: -60.0, z: PostureModule::pos_z(boma)});
            AttackModule::clear_all(boma);
            GroundModule::set_collidable(boma, false);
        }
    }
    else if smash::app::stage::get_stage_id() == 0x101 {
        //Town and City
        if PostureModule::pos_x(boma) >= 160.0 {
            PostureModule::set_pos(boma, &Vector3f{ x: -160.0 , y: PostureModule::pos_y(boma), z: PostureModule::pos_z(boma)});
            AttackModule::clear_all(boma);
            GroundModule::set_collidable(boma, false);
        }
        else if PostureModule::pos_x(boma) <= -160.0 {
            PostureModule::set_pos(boma, &Vector3f{ x: 160.0 , y: PostureModule::pos_y(boma), z: PostureModule::pos_z(boma)});
            AttackModule::clear_all(boma);
            GroundModule::set_collidable(boma, false);
        }
        else if PostureModule::pos_y(boma) <= -80.0 {
            PostureModule::set_pos(boma, &Vector3f{ x: PostureModule::pos_x(boma) , y: 105.0, z: PostureModule::pos_z(boma)});
            AttackModule::clear_all(boma);
            GroundModule::set_collidable(boma, false);
        }
        else if PostureModule::pos_y(boma) >= 105.0 {
            PostureModule::set_pos(boma, &Vector3f{ x: PostureModule::pos_x(boma) , y: -80.0, z: PostureModule::pos_z(boma)});
            AttackModule::clear_all(boma);
            GroundModule::set_collidable(boma, false);
        }
    }
    else if smash::app::stage::get_stage_id() == 0x169 {
        //Hollow Bastion
        if PostureModule::pos_x(boma) >= 180.0 {
            PostureModule::set_pos(boma, &Vector3f{ x: -180.0 , y: PostureModule::pos_y(boma), z: PostureModule::pos_z(boma)});
            AttackModule::clear_all(boma);
            GroundModule::set_collidable(boma, false);
        }
        else if PostureModule::pos_x(boma) <= -180.0 {
            PostureModule::set_pos(boma, &Vector3f{ x: 180.0 , y: PostureModule::pos_y(boma), z: PostureModule::pos_z(boma)});
            AttackModule::clear_all(boma);
            GroundModule::set_collidable(boma, false);
        }
        else if PostureModule::pos_y(boma) <= -60.0 {
            PostureModule::set_pos(boma, &Vector3f{ x: PostureModule::pos_x(boma) , y: 130.0, z: PostureModule::pos_z(boma)});
            AttackModule::clear_all(boma);
            GroundModule::set_collidable(boma, false);
        }
        else if PostureModule::pos_y(boma) >= 130.0 {
            PostureModule::set_pos(boma, &Vector3f{ x: PostureModule::pos_x(boma) , y: -60.0, z: PostureModule::pos_z(boma)});
            AttackModule::clear_all(boma);
            GroundModule::set_collidable(boma, false);
        }
    }
}

pub fn install() {
    acmd::install();
    frame::install();
    status::install();
    bigpacman::install();
    firehydrant::install();
    trampoline::install();
}