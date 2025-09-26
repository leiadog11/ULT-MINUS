use super::*;

/////////// PARACHUTE FORWARD SMASH

// INIT
pub unsafe extern "C" fn gamewatch_parachute_attacks4_init(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    //Snap to throw position
    let mut owner_pos = Vector3f{x:0.0,y:0.0,z:0.0};
    let mut article_pos = Vector3f{x:0.0,y:0.0,z:0.0};
    let mut offset_add = Vector3f{x:0.0,y:10.0,z:0.0};

    let lr = PostureModule::lr(owner_boma);
    let owner_offset = ModelModule::joint_global_offset_from_top(owner_boma, Hash40{hash: hash40("throw")}, &mut owner_pos);  
    let cap_offset = ModelModule::joint_global_offset_from_top(weapon.module_accessor, Hash40{hash: hash40("have")}, &mut article_pos);       
    let newPos = Vector3f{x: PostureModule::pos_x(owner_boma) + owner_pos.x - article_pos.x + (offset_add.x*lr), y: PostureModule::pos_y(owner_boma) + owner_pos.y - (article_pos.y)+ offset_add.y, z: PostureModule::pos_z(owner_boma) + owner_pos.z - article_pos.z};
    PostureModule::set_pos(weapon.module_accessor, &newPos);

    0.into()
}

// PRE
unsafe extern "C" fn gamewatch_parachute_attacks4_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor, 
        smash::app::SituationKind(*SITUATION_KIND_NONE), 
        *WEAPON_KINETIC_TYPE_NONE, 
        GROUND_CORRECT_KIND_AIR.into(), 
        smash::app::GroundCliffCheckKind(0), 
        false, 
        *WEAPON_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *WEAPON_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *WEAPON_STATUS_WORK_KEEP_FLAG_ALL_FLOAT, 
        0
    );
    return 0.into();
}

// MAIN
unsafe extern "C" fn gamewatch_parachute_attacks4_main(weapon: &mut L2CWeaponCommon) -> L2CValue { 
    let life = 80;
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("attack_s4"), 0.0, 1.0, false, 0.0, false, false);

    let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let lr = PostureModule::lr(weapon.module_accessor);
    let energy_type = KineticModule::get_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL) as *mut smash::app::KineticEnergy;
    let mut speed_x: f32 = lua_bind::KineticEnergy::get_speed_x(energy_type);
    let mut speed_y: f32 = lua_bind::KineticEnergy::get_speed_y(energy_type);

    speed_x = if lr == 1.0 { 1.0 } else { -1.0 };
    speed_y = 0.0; 

    // Declare status_frame
    let status_frame = weapon.global_table[0xe].get_f32();
    
    // Set speed
    weapon.clear_lua_stack();
    weapon.push_lua_stack(&mut L2CValue::new_int(*WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL as u64));
    weapon.push_lua_stack(&mut L2CValue::new_num(speed_x));
    weapon.push_lua_stack(&mut L2CValue::new_num(speed_y));
    sv_kinetic_energy::set_speed(weapon.lua_state_agent);

    weapon.fastshift(L2CValue::Ptr(gamewatch_parachute_attacks4_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn gamewatch_parachute_attacks4_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue { 
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);

    // REFLECTION CHECK
    if (AttackModule::is_infliction(weapon.module_accessor,*COLLISION_KIND_MASK_REFLECTOR)) {
        KineticModule::reflect_speed(weapon.module_accessor,  &Vector3f{x: 1.2, y: 0.0, z: 0.0}, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
        KineticModule::mul_accel(weapon.module_accessor,  &Vector3f{x: 0.0, y: 0.0, z: 0.0}, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
        return 0.into();
    }

    if life < 0 {
        parachute_remove(weapon);
        return 0.into();
    }

    return 0.into();
}

// REMOVE
pub unsafe extern "C" fn parachute_remove(weapon: &mut smashline::L2CWeaponCommon) {
    let owner = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);

    let pos = PostureModule::pos(weapon.module_accessor);
    let eff = EffectModule::req(
        weapon.module_accessor,
        Hash40::new("sys_misfire"),
        pos,
        &Vector3f{x: 0.0,y:0.0,z:0.0},
        1.0,
        0,
        -1,
        false,
        0
    ) as u32;
    EffectModule::set_rgb(weapon.module_accessor, eff, 0.5, 0.5, 0.5);

    smash_script::notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
}

// END
unsafe extern "C" fn gamewatch_parachute_attacks4_end(weapon: &mut L2CWeaponCommon) -> L2CValue { 
    return 0.into();
}

pub fn install() {
    Agent::new("gamewatch_parachute")
        .status(Init, WEAPON_GAMEWATCH_PARACHUTE_STATUS_KIND_ATTACKS4, gamewatch_parachute_attacks4_init)
        .status(Pre, WEAPON_GAMEWATCH_PARACHUTE_STATUS_KIND_ATTACKS4, gamewatch_parachute_attacks4_pre)
        .status(Main, WEAPON_GAMEWATCH_PARACHUTE_STATUS_KIND_ATTACKS4, gamewatch_parachute_attacks4_main)
        .status(End, WEAPON_GAMEWATCH_PARACHUTE_STATUS_KIND_ATTACKS4, gamewatch_parachute_attacks4_end)
        .install();
}