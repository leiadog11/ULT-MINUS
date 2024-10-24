use super::*;

/*
//////////// CLOUD

unsafe extern "C" fn cloud_regular_pre(weapon: &mut L2CWeaponCommon) -> L2CValue { 
    return 0.into();
}

unsafe extern "C" fn cloud_regular_main(weapon: &mut L2CWeaponCommon) -> L2CValue { 
    ArticleModule::set_visibility_whole(weapon.module_accessor, *FIGHTER_PICHU_GENERATE_ARTICLE_CLOUD, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    ArticleModule::set_visibility_whole(weapon.module_accessor, *FIGHTER_PIKACHU_GENERATE_ARTICLE_CLOUD, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    WorkModule::off_flag(weapon.module_accessor, *WEAPON_PIKACHU_CLOUD_INSTANCE_WORK_ID_FLAG_ACTIVATE_KAMINARI);
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("regular"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(cloud_regular_main_loop as *const () as _))
}

unsafe extern "C" fn cloud_regular_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue { 
    return 0.into();
}

unsafe extern "C" fn cloud_regular_end(weapon: &mut L2CWeaponCommon) -> L2CValue { 
    ArticleModule::remove_exist(weapon.module_accessor, *FIGHTER_PICHU_GENERATE_ARTICLE_CLOUD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    ArticleModule::remove_exist(weapon.module_accessor, *FIGHTER_PIKACHU_GENERATE_ARTICLE_CLOUD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    return 0.into();
}
*/

pub fn install() {
    /*
    Agent::new("pichu_cloud")
        .status(Pre, *WEAPON_PIKACHU_CLOUD_STATUS_KIND_REGULAR, cloud_regular_pre)
        .status(Main, *WEAPON_PIKACHU_CLOUD_STATUS_KIND_REGULAR, cloud_regular_main)
        .status(End, *WEAPON_PIKACHU_CLOUD_STATUS_KIND_REGULAR, cloud_regular_end)
        .install();
    */
}