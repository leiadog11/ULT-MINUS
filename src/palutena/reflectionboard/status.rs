use super::*;

//////////////////////// REFLECTION BOARD

//----------------------SHOOT------------------------

// PRE
unsafe extern "C" fn reflectionboard_shoot_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_AIR), *WEAPON_KINETIC_TYPE_NORMAL, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    return 0.into();
}

// INIT
unsafe extern "C" fn reflectionboard_shoot_init(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    //Snap to throw position
    let mut offset_add = Vector3f{x:10.0,y:10.0,z:0.0};
    let mut newPos = Vector3f{x:0.0,y:0.0,z:0.0};

    let lr = PostureModule::lr(owner_boma);
    if lr == 1.0 {
        newPos = Vector3f{x: PostureModule::pos_x(owner_boma) + offset_add.x, y: PostureModule::pos_y(owner_boma) + offset_add.y, z: PostureModule::pos_z(owner_boma)};
    } 
    else {
        newPos = Vector3f{x: PostureModule::pos_x(owner_boma) - offset_add.x, y: PostureModule::pos_y(owner_boma) + offset_add.y, z: PostureModule::pos_z(owner_boma)};
    }

    PostureModule::set_pos(weapon.module_accessor, &newPos);

    return 0.into();
}

// MAIN
unsafe extern "C" fn reflectionboard_shoot_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("shoot"), 0.0, 1.0, false, 0.0, false, false);
    let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let charge_mul = WorkModule::get_float(owner_boma, FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLOAT_CHARGE_MUL);
    let lr = PostureModule::lr(weapon.module_accessor);
    let mut life = 120;

    ReflectorModule::set_status(weapon.module_accessor, *WEAPON_PALUTENA_REFLECTIONBOARD_REFLECTOR_KIND_REFLECTOR, smash::app::ShieldStatus(*SHIELD_STATUS_NORMAL), 0);

    if charge_mul > 1.0 {
        life = 120 * charge_mul as i32;
    }

    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);

    let energy_type = KineticModule::get_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL) as *mut smash::app::KineticEnergy;
    let mut speed_x: f32 = lua_bind::KineticEnergy::get_speed_x(energy_type);

    speed_x = if lr == 1.0 { 0.5 } else { -0.5 };

    weapon.clear_lua_stack();
    weapon.push_lua_stack(&mut L2CValue::new_int(*WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL as u64));
    weapon.push_lua_stack(&mut L2CValue::new_num(speed_x));
    sv_kinetic_energy::set_speed(weapon.lua_state_agent);

    weapon.fastshift(L2CValue::Ptr(reflectionboard_shoot_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn reflectionboard_shoot_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);

    if life < 0 {
        weapon.change_status(WEAPON_PALUTENA_REFLECTIONBOARD_STATUS_KIND_BREAK.into(), false.into());
        WorkModule::set_float(owner_boma, 1.0, FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLOAT_CHARGE_MUL);
        return 1.into();
    }

    return 0.into();
}

// END
unsafe extern "C" fn reflectionboard_shoot_end(weapon: &mut L2CWeaponCommon) -> L2CValue {
    return 0.into();
}

pub fn install() {
    Agent::new("palutena_reflectionboard")
        .status(Pre, *WEAPON_PALUTENA_REFLECTIONBOARD_STATUS_KIND_SHOOT, reflectionboard_shoot_pre)
        .status(Init, *WEAPON_PALUTENA_REFLECTIONBOARD_STATUS_KIND_SHOOT, reflectionboard_shoot_init)
        .status(Main, *WEAPON_PALUTENA_REFLECTIONBOARD_STATUS_KIND_SHOOT, reflectionboard_shoot_main)
        .status(End, *WEAPON_PALUTENA_REFLECTIONBOARD_STATUS_KIND_SHOOT, reflectionboard_shoot_end)
        .install();
}