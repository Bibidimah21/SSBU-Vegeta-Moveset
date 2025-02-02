use crate::utils::*;
use smash::app::lua_bind::*;
use smash::app::sv_battle_object::module_accessor;
use smash::app::{ArticleOperationTarget, BattleObjectModuleAccessor, enSEType, FighterEntryID, HitStatus, SituationKind};
use smash::lib::lua_const::*;
use smash::lib::L2CValue;
use smash::phx::Vector3f;
use smash::lua2cpp::*;
use smashline::*;
use smash::hash40;
use smash::phx::Hash40;
use smash_utils::bomaext::ModelColorType;
use smash_utils::bomaext::BomaExt;
use smash_utils::cmdflag::{Buttons, Cat2, PadFlag};
use smash_utils::utils::{CancelKind, FIGHTER_MANAGER};

use crate::vegeta::CHARGE_TIME;

pub static mut GALICKGUN_ROT: [i32;8] = [0;8];



#[status_script(agent = "lucario_auraball", status = WEAPON_LUCARIO_AURABALL_STATUS_KIND_START, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn auraball_start(fighter: &mut L2CWeaponCommon) -> L2CValue {
    L2CValue::I32(0)
}

#[status_script(agent = "lucario_auraball", status = WEAPON_LUCARIO_AURABALL_STATUS_KIND_CHARGE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn auraball_charge(fighter: &mut L2CWeaponCommon) -> L2CValue {
    L2CValue::I32(0)
}

#[status_script(agent = "lucario_auraball", status = WEAPON_LUCARIO_AURABALL_STATUS_KIND_SHOOT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn auraball_shoot(fighter: &mut L2CWeaponCommon) -> L2CValue {
    L2CValue::I32(0)
}

#[status_script(agent = "lucario", status = FIGHTER_STATUS_KIND_SPECIAL_N, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn special_n_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    L2CValue::I32(0)
}

#[status_script(agent = "lucario", status = FIGHTER_STATUS_KIND_SPECIAL_N, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn special_n_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut boma = &mut *fighter.module_accessor;
    boma.inc_int(FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_KIBLAST_TOTAL);
    boma.play_se(Hash40::new("vc_lucario_004"));
    if boma.is_grounded(){
        boma.change_motion(Hash40::new("kiblast_left"), false)
    }
    else{
        boma.set_position_lock();
        boma.change_motion(Hash40::new("kiblastair_left"), false)
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(special_n_main as *const () as _))
}

unsafe extern "C" fn special_n_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut boma:&mut BattleObjectModuleAccessor = &mut *fighter.module_accessor;
    let max_kiblasts = boma.get_int(FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_MAX_KIBLASTS);
    if boma.is_grounded(){
        boma.set_position_lock();
    }
    if !boma.is_grounded(){
        if boma.get_int(FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_AIR_TIMER) < 90{
            boma.set_position_lock();
            boma.inc_int(FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_AIR_TIMER);
        }
        else{
            boma.unset_position_lock()
        }
    }
    if boma.is_button_on(Buttons::Attack){
        boma.on_flag(FIGHTER_VEGETA_INSTANCE_WORK_ID_FLAG_KIBLAST_RAPIDFIRE);
    }
    if boma.is_motion_one_of(&[Hash40::new("kiblastair_right"), Hash40::new("kiblastair_left")]) && boma.is_grounded(){
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
    }
    if boma.motion_frame() > 10.0{
        if boma.is_flag(FIGHTER_VEGETA_INSTANCE_WORK_ID_FLAG_KIBLAST_RAPIDFIRE) && boma.get_int(FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_KIBLAST_TOTAL) < max_kiblasts{
            if boma.is_motion(Hash40::new("kiblast_left")) || boma.is_motion(Hash40::new("kiblastair_left")){
                boma.inc_int(FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_KIBLAST_TOTAL);
                if boma.is_grounded(){
                    boma.change_motion(Hash40::new("kiblast_right"), false)
                }
                else{
                    boma.change_motion(Hash40::new("kiblastair_right"), false)
                }
            }
            else{
                boma.inc_int(FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_KIBLAST_TOTAL);
                if boma.is_grounded(){
                    boma.change_motion(Hash40::new("kiblast_left"), false)
                }
                else{
                    boma.change_motion(Hash40::new("kiblastair_left"), false)
                }
            }
        }
        else if boma.is_button_on(Buttons::Special) && boma.get_int(FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_KIBLAST_TOTAL) < max_kiblasts{
            if boma.is_motion(Hash40::new("kiblast_left")) || boma.is_motion(Hash40::new("kiblastair_left")){
                boma.inc_int(FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_KIBLAST_TOTAL);
                if boma.is_grounded(){
                    boma.change_motion(Hash40::new("kiblast_right"), false)
                }
                else{
                    boma.change_motion(Hash40::new("kiblastair_right"), false)
                }
            }
            else{
                boma.inc_int(FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_KIBLAST_TOTAL);
                if boma.is_grounded(){
                    boma.change_motion(Hash40::new("kiblast_left"), false)
                }
                else{
                    boma.change_motion(Hash40::new("kiblastair_left"), false)
                }
            }
        }
    }
    if boma.is_motion_end(){
        if boma.is_grounded() {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    L2CValue::I32(0)
}

#[status_script(agent = "lucario", status = FIGHTER_STATUS_KIND_SPECIAL_N, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn special_n_status_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut boma = &mut *fighter.module_accessor;
    boma.set_int(0, FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_KIBLAST_TOTAL);
    boma.dec_int(FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_MAX_KIBLASTS);
    boma.off_flag(FIGHTER_VEGETA_INSTANCE_WORK_ID_FLAG_KIBLAST_RAPIDFIRE);
    boma.unset_position_lock();
    boma.stop_all_sound();
    L2CValue::I32(0)
}

#[status_script(agent = "lucario", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn special_s_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut boma = &mut *fighter.module_accessor;
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if boma.is_grounded(){
        boma.change_motion(Hash40::new("bigbangatk"), false)
    }
    else{
        boma.change_motion(Hash40::new("bigbangatk_air"), false)
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(bigbangatk_main as *const () as _))
}


unsafe extern "C" fn bigbangatk_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut boma = &mut *fighter.module_accessor;
    let lua_state = fighter.lua_state_agent;
    let entry_id = boma.entry_id();
    if boma.is_grounded(){
        boma.set_position_lock();
    }
    if !boma.is_grounded(){
        if boma.get_int(FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_AIR_TIMER) < 90{
            boma.set_position_lock();
            boma.inc_int(FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_AIR_TIMER);
        }
        else{
            boma.unset_position_lock()
        }
    }


    boma.suspend_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION);
    if boma.is_motion_end(){
        if boma.is_grounded() {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        } else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    L2CValue::I32(0)
}

#[status_script(agent = "lucario", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn bigbangatk_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut boma = &mut *fighter.module_accessor;
    boma.unset_position_lock();
    L2CValue::I32(0)
}

#[status_script(agent = "lucario", status = 0x1eb, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn superdashkick_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    L2CValue::I32(0)
}

#[status_script(agent = "lucario", status = 0x1eb, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn superdashkick(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut boma = &mut *fighter.module_accessor;

    boma.change_motion(Hash40::new("superdashkick"), false);
    fighter.sub_shift_status_main(L2CValue::Ptr(superdashkick_main as *const () as _))
}

unsafe extern "C" fn superdashkick_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut boma = &mut *fighter.module_accessor;
    boma.set_gravity(false);
    boma.suspend_energy(*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    if boma.is_motion_end() {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    L2CValue::I32(0)
}

#[status_script(agent = "lucario", status = 0x1eb, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn superdashkick_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut boma = &mut *fighter.module_accessor;
    boma.set_gravity(true);
    L2CValue::I32(0)
}

#[status_script(agent = "lucario", status = 0x1ec, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn galickgun_start_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    //println!("gg start pre");

    L2CValue::I32(0)
}

#[status_script(agent = "lucario", status = 0x1ec, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn galickgun_start(fighter: &mut L2CFighterCommon) -> L2CValue {
    //println!("gg start");
    let mut boma = &mut *fighter.module_accessor;

    if boma.is_grounded() {
        boma.change_motion(Hash40::new("galickgun_start"), false);
    } else {
        boma.change_motion(Hash40::new("galickgun_start_air"), false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(galickgun_start_main as *const () as _))
}

unsafe extern "C" fn galickgun_start_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    //println!("gg start main");
    let mut boma = &mut *fighter.module_accessor;
    let lua_state = fighter.lua_state_agent;
    let entry_id = boma.entry_id();

    let hold_eff: u32 = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_hit_elec"), smash::phx::Hash40::new("haver"), &ZERO_VECTOR, &ZERO_VECTOR, 0.4, true, 0, 0, 0, 0, 0, true, true) as u32;
    EffectModule::set_rgb(fighter.module_accessor, hold_eff, 0.5, 0.3, 0.6);
    boma.set_color_rgb(0.5, 0.3, 0.6, ModelColorType(*MODEL_COLOR_TYPE_COLOR_BLEND));
    boma.play_se(Hash40::new("vc_lucario_appeal01"));
    if boma.is_motion_end() {
        fighter.change_status(
            FIGHTER_VEGETA_STATUS_KIND_GALICK_GUN_HOLD.into(),
            false.into(),
        );
    }
    L2CValue::I32(0)
}

#[status_script(agent = "lucario", status = 0x1ec, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn galickgun_start_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    //println!("gg start end");
    let mut boma = &mut *fighter.module_accessor;
    boma.unset_position_lock();
    L2CValue::I32(0)
}

#[status_script(agent = "lucario", status = 0x1ed, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn galickgun_hold_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    //println!("gg hold pre");
    L2CValue::I32(0)
}

#[status_script(agent = "lucario", status = 0x1ed, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn galickgun_hold(fighter: &mut L2CFighterCommon) -> L2CValue {
    //println!("gg hold");
    let mut boma = &mut *fighter.module_accessor;

    if boma.is_grounded() {
        boma.change_motion(Hash40::new("galickgun_hold"), false);
    } else {
        boma.change_motion(Hash40::new("galickgun_hold_air"), false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(galickgun_hold_main as *const () as _))
}


unsafe extern "C" fn galickgun_hold_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    //println!("gg hold main");
    let mut boma = &mut *fighter.module_accessor;
    let lua_state = fighter.lua_state_agent;
    let entry_id = boma.entry_id();
    //  disable_gravity(fighter.module_accessor);
    CHARGE_TIME[entry_id] += 1.0;
    if CHARGE_TIME[entry_id] >= 60.0{
        fighter.change_status(
            FIGHTER_VEGETA_STATUS_KIND_GALICK_GUN_FIRE.into(),
            false.into(),
        );
    }

    let hold_eff: u32 = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_hit_elec"), smash::phx::Hash40::new("haver"), &ZERO_VECTOR, &ZERO_VECTOR, 0.4, true, 0, 0, 0, 0, 0, true, true) as u32;
    EffectModule::set_rgb(fighter.module_accessor, hold_eff, 0.5, 0.3, 0.6);
    boma.set_color_rgb(0.5, 0.3, 0.6, ModelColorType(*MODEL_COLOR_TYPE_COLOR_BLEND));

    if !SoundModule::is_playing(fighter.module_accessor, Hash40::new("se_galickgun_start")){
       // boma.play_se(Hash40::new("se_galickgun_hold"));
    }
    if boma.is_button_on(Buttons::AppealSL) {
        boma.set_position_lock();
    } else {
        boma.stop_all_sound();
        fighter.change_status(
            FIGHTER_VEGETA_STATUS_KIND_GALICK_GUN_FIRE.into(),
            false.into(),
        );
    }
    L2CValue::I32(0)
}

#[status_script(agent = "lucario", status = 0x1ed, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn galickgun_hold_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    //println!("gg hold end");
    //  enable_gravity(fighter.module_accessor);
    let mut boma = &mut *fighter.module_accessor;
    boma.unset_position_lock();
    boma.stop_all_sound();
    L2CValue::I32(0)
}

#[status_script(agent = "lucario", status = 0x1ee, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn galickgunfire_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    L2CValue::I32(0)
}

#[status_script(agent = "lucario", status = 0x1ee, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn galickgunfire(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut boma = &mut *fighter.module_accessor;
    boma.play_se(Hash40::new("vc_lucario_appeal02"));
    if boma.is_grounded() {
        boma.change_motion(Hash40::new("galickgun_fire"), false);
    } else {
        boma.change_motion(Hash40::new("galickgun_fire_air"), false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(galickgunfire_main as *const () as _))
}

unsafe extern "C" fn galickgunfire_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut boma = &mut *fighter.module_accessor;

    let lua_state = fighter.lua_state_agent;
    let entry_id = boma.entry_id();
    // disable_gravity(fighter.module_accessor);
    if boma.is_grounded(){
        if boma.lr() > 0.0 {
            GALICKGUN_ROT[entry_id] = 90;
        }
        else {
            GALICKGUN_ROT[entry_id] = 270;
        }
    }
    else{
        if boma.lr() > 0.0 {
            GALICKGUN_ROT[entry_id] = 44;
        } else {
            GALICKGUN_ROT[entry_id] = 316;
        }
    }

    if boma.motion_frame() >= 35.0{
       EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_hit_elec"), true, false);
    }
    else{
        /*
        let hold_eff: u32 = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_hit_elec"), smash::phx::Hash40::new("rot"), &Vector3f {x: 0.0, y: 0.0, z: -4.0}, &ZERO_VECTOR, 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
        EffectModule::set_alpha(fighter.module_accessor, hold_eff, 0.3);
        EffectModule::set_rgb(fighter.module_accessor, hold_eff, 0.5, 0.3, 0.6);
         */
    }
    //boma.set_color_rgb( 0.5, 0.3, 0.6, ModelColorType(*MODEL_COLOR_TYPE_COLOR_BLEND));

    boma.set_position_lock();
    if boma.is_motion_end() {
        if boma.is_grounded() {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        } else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    L2CValue::I32(0)
}

#[status_script(agent = "lucario", status = 0x1ee, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn galickgunfire_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    //  enable_gravity(fighter.module_accessor);
    let mut boma = &mut *fighter.module_accessor;
    boma.unset_position_lock();
    boma.stop_all_sound();
    EffectModule::kill_kind(boma, Hash40::new("lucario_final_beam"), false, true);
    L2CValue::I32(0)
}


#[status_script(agent = "lucario", status = FIGHTER_STATUS_KIND_ATTACK_HI4, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn attackhi4_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut boma = &mut *fighter.module_accessor;
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    boma.change_motion(Hash40::new("attack_hi4"), false);
    fighter.sub_shift_status_main(L2CValue::Ptr(attackhi4_main as *const () as _))
}
unsafe extern "C" fn attackhi4_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut boma:&mut BattleObjectModuleAccessor = &mut *fighter.module_accessor;
    let lua_state = fighter.lua_state_agent;
    let attacked_players = get_attacked_players(boma);
    if attacked_players.len() > 0{
        let attacked_player = &mut *get_module_accessor_by_entry_id(attacked_players[0] as i32);
        if (19.0..25.0).contains(&boma.motion_frame()){
            if boma.is_button_on(Buttons::Attack){
                let teleport_eff = boma.get_int(FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_TELEPORT_EFF_HANDLE) as u32;
                if !boma.is_flag(FIGHTER_VEGETA_INSTANCE_WORK_ID_FLAG_USED_TELEPORT_EFFECT){
                    let teleport = EffectModule::req_on_joint(boma,Hash40::new("sys_attack_speedline"),Hash40::new("rot"),&ZERO_VECTOR,&ZERO_VECTOR,2.0,&ZERO_VECTOR, &ZERO_VECTOR,false,0,0,0) as u32;
                    boma.set_int(teleport as i32, FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_TELEPORT_EFF_HANDLE);
                  //  EffectModule::set_rate(boma, teleport, 2.0);
                    boma.on_flag(FIGHTER_VEGETA_INSTANCE_WORK_ID_FLAG_USED_TELEPORT_EFFECT);
                    ModelModule::set_scale(boma, 0.0);
                    MotionModule::set_rate(boma, 0.0);
                    boma.play_se(Hash40::new("se_lucario_smash_h01"));
                }
                else if !EffectModule::is_exist_common(boma, Hash40::new("sys_attack_speedline")){
                    ModelModule::set_scale(boma, 1.0);
                    MotionModule::set_rate(boma, 1.0);
                    boma.set_position(&Vector3f{
                        x: attacked_player.pos_x(),
                        y: attacked_player.pos_y(),
                        z: attacked_player.pos_z()
                    });
                    boma.on_flag(FIGHTER_VEGETA_INSTANCE_WORK_ID_FLAG_AMAZING_IMPACT);
                    acmd!(lua_state, {
                   SLOW_OPPONENT(20, 60)
                });
                    fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_AIR.into(), false.into())
                }
            }
        }
    }
    if boma.is_motion_end() {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
    }
    L2CValue::I32(0)
}

#[status_script(agent = "lucario", status = FIGHTER_STATUS_KIND_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn attackair_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut boma = &mut *fighter.module_accessor;
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    boma.off_flag(FIGHTER_VEGETA_INSTANCE_WORK_ID_FLAG_USED_TELEPORT_EFFECT);
    boma.off_flag(FIGHTER_VEGETA_INSTANCE_WORK_ID_FLAG_USED_TELEPORT_EFFECT);
    boma.set_int(0, FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_TELEPORT_EFF_HANDLE);
    if boma.is_flag(FIGHTER_VEGETA_INSTANCE_WORK_ID_FLAG_AMAZING_IMPACT){
        boma.change_motion(Hash40::new("attack_air_f"), false);
        AttackModule::set_reaction_mul(boma, 1.3);
        fighter.sub_shift_status_main(L2CValue::Ptr(attackair_main as *const () as _))
    }
    else{
        original!(fighter)
    }
}
unsafe extern "C" fn attackair_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut boma:&mut BattleObjectModuleAccessor = &mut *fighter.module_accessor;
    let lua_state = fighter.lua_state_agent;
    boma.set_position_lock();
    if boma.motion_frame() == 10.0{
        acmd!(lua_state, {
            SLOW_OPPONENT(0, 0)
        });
    }
    if boma.is_motion_end(){
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    L2CValue::I32(0)
}

#[status_script(agent = "lucario", status = FIGHTER_STATUS_KIND_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn attackair_status_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut boma = &mut *fighter.module_accessor;
    let lua_state = fighter.lua_state_agent;
    boma.unset_position_lock();
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    boma.off_flag(FIGHTER_VEGETA_INSTANCE_WORK_ID_FLAG_AMAZING_IMPACT);
    AttackModule::set_reaction_mul(boma, 1.0);
    original!(fighter)
}

#[status_script(agent = "lucario", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn ki_charge(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut boma = &mut *fighter.module_accessor;
    EffectModule::kill_kind(boma, Hash40::new("lucario_aura"), false, true);
    boma.change_motion(Hash40::new("ki_charge_start"), false);
    fighter.sub_shift_status_main(L2CValue::Ptr(kicharge_main as *const () as _))
}

unsafe extern "C" fn kicharge_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut boma:&mut BattleObjectModuleAccessor = &mut *fighter.module_accessor;
    let lua_state = fighter.lua_state_agent;

    let mut time = 0;
    match boma.get_int(FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_CURRENT_FORM) {
        0 => {
            time = 60
        },
        1 => {
            time = 90
        },
        2 => {
            time = 120
        }
        _ => {}
    }

    if boma.is_grounded(){
        boma.set_position_lock();
        acmd!(lua_state, {
            sv_module_access::damage(MSC=MA_MSC_DAMAGE_DAMAGE_NO_REACTION, Type=DAMAGE_NO_REACTION_MODE_ALWAYS, DamageThreshold=0)
        });
    }
    else{
        if boma.get_int(FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_AIR_TIMER) < 90{
            boma.set_position_lock();
            boma.inc_int(FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_AIR_TIMER);
        }
        else{
            boma.unset_position_lock()
        }
    }
    if boma.is_motion(Hash40::new("ki_charge_start")) && boma.is_motion_end(){
        if boma.is_button_on(Buttons::Special) && boma.get_int(FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_KI_CHARGE) != time && boma.get_int(FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_CURRENT_FORM) < 3{
            boma.change_motion(Hash40::new("ki_charge_hold"), false);
        }
        else if boma.motion_frame() == 28.0 && boma.get_int(FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_KI_CHARGE) >= time && boma.get_int(FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_CURRENT_FORM) < 3{
            boma.set_int(0, FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_KI_CHARGE);
            boma.set_int(0, FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_CURRENT_FORM_TIMER);
            boma.stop_all_sound();
            boma.play_se(Hash40::new("vc_lucario_003"));
            boma.play_se(Hash40::new("se_lucario_special_l02"));
            let handle = EffectModule::req_follow(boma, Hash40::new("lucario_aura"), smash::phx::Hash40::new("top"), &ZERO_VECTOR, &ZERO_VECTOR, 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
            EffectModule::set_rate(boma, handle, 3.0);
            boma.inc_int(FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_CURRENT_FORM);
            let current_form = boma.get_int(FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_CURRENT_FORM);
            if current_form == 1{
                EffectModule::set_rgb(boma, handle, 0.7, 0.7, 0.0);
                EffectModule::set_alpha(boma, handle, 2.0);
            }
            else if current_form == 2{
                EffectModule::set_rgb(boma, handle, 1.5, 1.5, 1.5);
            }
            else if current_form == 3{
                EffectModule::set_rgb(boma, handle, 10.0, 0.7, 10.0);
            }
            else{
                EffectModule::set_rgb(boma, handle, 0.3, 0.3, 0.3);
                EffectModule::set_alpha(boma, handle, 2.0);
            }
        }
        else{
            boma.change_motion(Hash40::new("ki_charge_end"), false);
        }
    }
    if boma.is_motion(Hash40::new("ki_charge_hold")){
        let current_form = boma.get_int(FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_CURRENT_FORM);
        let handle = boma.get_int(FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_KI_CHARGE_EFFECT_HANDLE) as u32;
        let aura_time = boma.get_int(FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_TIME_AURA_RESET);
        boma.inc_int(FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_KI_CHARGE);

        if boma.get_int(FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_KI_CHARGE) >= time{
            EffectModule::kill_kind(boma, Hash40::new("lucario_aura"), false, true);
            boma.change_motion(Hash40::new("ki_charge_start"), false);
        }
        if aura_time < 15{
            if aura_time == 0{
                let aura = EffectModule::req_follow(boma, Hash40::new("lucario_aura"), smash::phx::Hash40::new("top"), &ZERO_VECTOR, &ZERO_VECTOR, 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
                boma.set_int(aura as i32, FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_KI_CHARGE_EFFECT_HANDLE as i32);
                EffectModule::set_rate(boma, aura, 0.67);
            }
            boma.inc_int(FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_TIME_AURA_RESET);
        }
        else{
            EffectModule::kill_kind(boma, Hash40::new("lucario_aura"), false, true);
            boma.set_int(0, FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_TIME_AURA_RESET);
        }
        if !boma.is_flag(FIGHTER_VEGETA_INSTANCE_WORK_ID_FLAG_PLAYED_KICHARGE_SOUNDS){
            boma.on_flag(FIGHTER_VEGETA_INSTANCE_WORK_ID_FLAG_PLAYED_KICHARGE_SOUNDS);
            boma.play_se(Hash40::new("vc_lucario_002"));
            boma.play_se(Hash40::new("se_lucario_special_l01"));
        }
        //
        if current_form == 1{
            EffectModule::set_rgb(boma, handle, 0.7, 0.7, 0.0);
            EffectModule::set_alpha(boma, handle, 2.0);
        }
        else if current_form == 2{
            EffectModule::set_rgb(boma, handle, 1.5, 1.5, 1.5);
        }
        else if current_form == 3{
            EffectModule::set_rgb(boma, handle, 10.0, 0.7, 10.0);
        }
        else{
            EffectModule::set_rgb(boma, handle, 0.3, 0.3, 0.3);
            EffectModule::set_alpha(boma, handle, 2.0);
        }
        if !boma.is_button_on(Buttons::Special){
            EffectModule::kill_kind(boma, Hash40::new("lucario_aura"), false, true);
            boma.change_motion(Hash40::new("ki_charge_end"), false);
            boma.stop_all_sound();
        }
    }
    if boma.is_motion_end() {
        if boma.is_grounded() {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        } else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    L2CValue::I32(0)
}

#[status_script(agent = "lucario", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn ki_charge_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut boma = &mut *fighter.module_accessor;
    boma.unset_position_lock();
    KineticModule::clear_speed_all(boma);
    boma.off_flag(FIGHTER_VEGETA_INSTANCE_WORK_ID_FLAG_PLAYED_KICHARGE_SOUNDS);
    L2CValue::I32(0)
}

#[status_script(agent = "lucario", status = FIGHTER_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn special_hi_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut boma = &mut *fighter.module_accessor;
    L2CValue::I32(0)
}
#[status_script(agent = "lucario", status = FIGHTER_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn special_hi_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut boma = &mut *fighter.module_accessor;
    boma.change_motion(Hash40::new("superdashkick"), false);
    fighter.sub_shift_status_main(L2CValue::Ptr(specialhi_main as *const () as _))
}


unsafe extern "C" fn specialhi_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut boma:&mut BattleObjectModuleAccessor = &mut *fighter.module_accessor;
    boma.enable_energy(*FIGHTER_KINETIC_TYPE_MOTION_AIR);
    boma.enable_energy(*FIGHTER_KINETIC_TYPE_MOTION);
    boma.set_gravity(false);
    StatusModule::set_situation_kind(boma, SituationKind(*SITUATION_KIND_AIR), true);
    if boma.is_motion_end(){
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    L2CValue::I32(0)
}

#[status_script(agent = "lucario", status = FIGHTER_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn special_hi_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut boma:&mut BattleObjectModuleAccessor = &mut *fighter.module_accessor;
    L2CValue::I32(0)
}

#[status_script(agent = "lucario", status = FIGHTER_STATUS_KIND_FINAL, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn final_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut boma = &mut *fighter.module_accessor;
    L2CValue::I32(0)
}
#[status_script(agent = "lucario", status = FIGHTER_STATUS_KIND_FINAL, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn final_main_script(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut boma = &mut *fighter.module_accessor;
    boma.change_motion(Hash40::new("final_start_l"), false);
    fighter.sub_shift_status_main(L2CValue::Ptr(final_main as *const () as _))
}


unsafe extern "C" fn final_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut boma:&mut BattleObjectModuleAccessor = &mut *fighter.module_accessor;
    if boma.is_motion_end(){
        fighter.change_status(FIGHTER_LUCARIO_STATUS_KIND_FINAL_END.into(), false.into())
    }
    boma.set_position_lock();
    StatusModule::set_situation_kind(boma, SituationKind(*SITUATION_KIND_AIR), true);
    L2CValue::I32(0)
}

#[status_script(agent = "lucario", status = FIGHTER_STATUS_KIND_FINAL, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn final_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut boma:&mut BattleObjectModuleAccessor = &mut *fighter.module_accessor;
    boma.unset_position_lock();
    L2CValue::I32(0)
}

#[status_script(agent = "lucario", status = FIGHTER_LUCARIO_STATUS_KIND_FINAL_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn final_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    L2CValue::I32(0)
}

pub fn install() {
    smashline::install_status_scripts!(
        superdashkick_pre,
        superdashkick,
        superdashkick_end,
        galickgun_start_pre,
        galickgun_start,
        galickgun_start_end,
        galickgun_hold_pre,
        galickgun_hold,
        galickgun_hold_end,
        galickgunfire_pre,
        galickgunfire,
        galickgunfire_end,
        special_s_status,
        bigbangatk_end,
        special_n_pre_status,
        special_n_status,
        special_n_status_end,
        auraball_start,
        auraball_charge,
        auraball_shoot,
        attackhi4_status,
        attackair_status,
        attackair_status_end,
        ki_charge,
        ki_charge_end,
        //special_hi_pre,
        //special_hi_main,
        //special_hi_end,
        final_pre,
        final_main_script,
        final_end,
        final_end_main
    );
}
