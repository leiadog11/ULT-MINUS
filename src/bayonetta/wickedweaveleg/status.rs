use super::*;

//-------------------- ATTACK S4 --------------------

// PRE
unsafe extern "C" fn wickedweaveleg_attacks4_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor, 
        SituationKind(*SITUATION_KIND_AIR), 
        *WEAPON_KINETIC_TYPE_NORMAL, 
        GROUND_CORRECT_KIND_NONE.into(), 
        GroundCliffCheckKind(0), 
        false, 
        0, 
        0, 
        0, 
        0
    );

    return 0.into();
}

// MAIN
unsafe extern "C" fn wickedweaveleg_attacks4_main(weapon: &mut L2CWeaponCommon) -> L2CValue { 
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("attack_s4_s"), 0.0, 1.0, false, 0.0, false, false);

    let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let life = 145;
    let lr = PostureModule::lr(weapon.module_accessor);

    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);

    if LinkModule::is_link(weapon.module_accessor, *WEAPON_LINK_NO_CONSTRAINT) {
        LinkModule::unlink(weapon.module_accessor, *WEAPON_LINK_NO_CONSTRAINT);
    }

    let pos_x = PostureModule::pos_x(owner_boma);
    let pos_y = PostureModule::pos_y(owner_boma);
    let pos_z = PostureModule::pos_z(owner_boma);

    let mut newPos = Vector3f{x: pos_x + 5.0 * lr, y: pos_y + 7.0, z: pos_z};
    PostureModule::set_pos(weapon.module_accessor, &newPos);

    weapon.fastshift(L2CValue::Ptr(wickedweaveleg_attacks4_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn wickedweaveleg_attacks4_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let lr = PostureModule::lr(weapon.module_accessor);

    let speed_x = 1.0 * lr;
    let speed_y = 0.0;
    weapon.clear_lua_stack();
    weapon.push_lua_stack(&mut L2CValue::new_int(*WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL as u64));
    weapon.push_lua_stack(&mut L2CValue::new_num(speed_x));
    weapon.push_lua_stack(&mut L2CValue::new_num(speed_y));
    sv_kinetic_energy::set_speed(weapon.lua_state_agent);
   
    // REFLECTION CHECK - REMOVE FOR NOW
    if (AttackModule::is_infliction(weapon.module_accessor,*COLLISION_KIND_MASK_REFLECTOR)) {
        wickedweaveleg_remove(weapon);
        return 0.into();
    }

    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);

    if life > 35 && life < 125 {
        MotionModule::set_rate(weapon.module_accessor, 0.0);
    } else {
        MotionModule::set_rate(weapon.module_accessor, 1.0);
    }

    if life <= 0 {
        wickedweaveleg_remove(weapon);
        return 0.into();
    }

    return 0.into();
}

//-------------------- ATTACK LW4 --------------------

// PRE
unsafe extern "C" fn wickedweaveleg_attacklw4_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor, 
        SituationKind(*SITUATION_KIND_AIR), 
        *WEAPON_KINETIC_TYPE_NORMAL, 
        GROUND_CORRECT_KIND_NONE.into(), 
        GroundCliffCheckKind(0), 
        false, 
        0, 
        0, 
        0, 
        0
    );

    return 0.into();
}

// MAIN
unsafe extern "C" fn wickedweaveleg_attacklw4_main(weapon: &mut L2CWeaponCommon) -> L2CValue { 
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("attack_lw4"), 0.0, 1.0, false, 0.0, false, false);

    let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let life = 145;
    let lr = PostureModule::lr(weapon.module_accessor);

    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);

    if LinkModule::is_link(weapon.module_accessor, *WEAPON_LINK_NO_CONSTRAINT) {
        LinkModule::unlink(weapon.module_accessor, *WEAPON_LINK_NO_CONSTRAINT);
    }

    let pos_x = PostureModule::pos_x(owner_boma);
    let pos_y = PostureModule::pos_y(owner_boma);
    let pos_z = PostureModule::pos_z(owner_boma);

    let mut newPos = Vector3f{x: pos_x + 5.0 * lr, y: pos_y + 5.0, z: pos_z};
    PostureModule::set_pos(weapon.module_accessor, &newPos);

    weapon.fastshift(L2CValue::Ptr(wickedweaveleg_attacklw4_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn wickedweaveleg_attacklw4_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);

    if life < 130 {
        let speed_x = 0.0; 
        let speed_y = -1.0;
        weapon.clear_lua_stack();
        weapon.push_lua_stack(&mut L2CValue::new_int(*WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL as u64));
        weapon.push_lua_stack(&mut L2CValue::new_num(speed_x));
        weapon.push_lua_stack(&mut L2CValue::new_num(speed_y));
        sv_kinetic_energy::set_speed(weapon.lua_state_agent);
    }
   
    // REFLECTION CHECK - REMOVE FOR NOW
    if (AttackModule::is_infliction(weapon.module_accessor, *COLLISION_KIND_MASK_REFLECTOR)) {
        wickedweaveleg_remove(weapon);
        return 0.into();
    }

    if life > 40 && life < 125 {
        MotionModule::set_rate(weapon.module_accessor, 0.0);
    } else {
        MotionModule::set_rate(weapon.module_accessor, 1.0);
    }

    if life <= 0 {
        wickedweaveleg_remove(weapon);
        return 0.into();
    }

    return 0.into();
}

//-------------------- ATTACK HI4 --------------------

// PRE
unsafe extern "C" fn wickedweaveleg_attackhi4_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor, 
        SituationKind(*SITUATION_KIND_AIR), 
        *WEAPON_KINETIC_TYPE_NORMAL, 
        GROUND_CORRECT_KIND_NONE.into(), 
        GroundCliffCheckKind(0), 
        false, 
        0, 
        0, 
        0, 
        0
    );

    return 0.into();
}

// MAIN
unsafe extern "C" fn wickedweaveleg_attackhi4_main(weapon: &mut L2CWeaponCommon) -> L2CValue { 
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("attack_hi4"), 0.0, 1.0, false, 0.0, false, false);

    let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let lr = PostureModule::lr(weapon.module_accessor);

    let life = 70;
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);

    if LinkModule::is_link(weapon.module_accessor, *WEAPON_LINK_NO_CONSTRAINT) {
        LinkModule::unlink(weapon.module_accessor, *WEAPON_LINK_NO_CONSTRAINT);
    }

    let pos_x = PostureModule::pos_x(owner_boma);
    let pos_y = PostureModule::pos_y(owner_boma);
    let pos_z = PostureModule::pos_z(owner_boma);

    let mut newPos = Vector3f{x: pos_x + 5.0 * lr, y: pos_y + 0.0, z: pos_z};
    PostureModule::set_pos(weapon.module_accessor, &newPos);

    weapon.fastshift(L2CValue::Ptr(wickedweaveleg_attackhi4_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn wickedweaveleg_attackhi4_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let speed_x = 0.0; 
    let speed_y = 0.0;
    weapon.clear_lua_stack();
    weapon.push_lua_stack(&mut L2CValue::new_int(*WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL as u64));
    weapon.push_lua_stack(&mut L2CValue::new_num(speed_x));
    weapon.push_lua_stack(&mut L2CValue::new_num(speed_y));
    sv_kinetic_energy::set_speed(weapon.lua_state_agent);

    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);

    if life <= 0 {
        wickedweaveleg_remove(weapon);
        return 0.into();
    }

    return 0.into();
}

// REMOVE
pub unsafe extern "C" fn wickedweaveleg_remove(weapon: &mut L2CWeaponCommon) {
    smash_script::notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
}

pub fn install() {
    Agent::new("bayonetta_wickedweaveleg")
        .status(Pre, WEAPON_BAYONETTA_WICKEDWEAVELEG_STATUS_KIND_ATTACK_S4, wickedweaveleg_attacks4_pre)
        .status(Main, WEAPON_BAYONETTA_WICKEDWEAVELEG_STATUS_KIND_ATTACK_S4, wickedweaveleg_attacks4_main)

        .status(Pre, *WEAPON_BAYONETTA_WICKEDWEAVELEG_STATUS_KIND_ATTACK_LW4, wickedweaveleg_attacklw4_pre)
        .status(Main, *WEAPON_BAYONETTA_WICKEDWEAVELEG_STATUS_KIND_ATTACK_LW4, wickedweaveleg_attacklw4_main)

        .status(Pre, WEAPON_BAYONETTA_WICKEDWEAVELEG_STATUS_KIND_ATTACK_HI4, wickedweaveleg_attackhi4_pre)
        .status(Main, WEAPON_BAYONETTA_WICKEDWEAVELEG_STATUS_KIND_ATTACK_HI4, wickedweaveleg_attackhi4_main)
    .install();
}