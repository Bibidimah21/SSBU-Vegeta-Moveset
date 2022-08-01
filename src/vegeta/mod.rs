use std::borrow::BorrowMut;
use std::sync::atomic::{AtomicPtr, Ordering};
use smash::{
    *,
    hash40,
    phx::{Hash40, Vector3f},
    lib::{lua_const::*, L2CValue},
    app::{lua_bind::{*, StatusModule::*}, sv_animcmd::{frame, wait}, BattleObjectModuleAccessor},
    lua2cpp::{L2CFighterCommon, L2CAgentBase, L2CFighterBase}
};
use smash_utils::bomaext::BomaExt;
use smashline::*;
use smash_script::{macros::*, notify_event_msc_cmd, lua_args};
use std::vec::Vec;
use smash::lib::lua_const::*;
use crate::utils::*;
use skyline::hooks::{getRegionAddress, Region};
use skyline::libc::*;
use smash::app::{ArticleOperationTarget, enSEType, Fighter};
use smash_utils::cmdflag::Buttons;
use crate::vegeta_status::*;
use smash_utils::utils::{CancelKind, FIGHTER_MANAGER};
pub static mut CHARGE_TIME:[f32;8] = [0.0;8];
pub static mut TEST: i32 = 0;
pub static mut TEST2: i32 = 0;
pub static mut TEST3: i32 = 0;


pub const FIGHTER_VEGETA_GENERATE_ARTICLE_BBATK: i32 = 0x4;

enum MeshType{
    SS,
    SSB,
    SSBE,
    Base,
    All
}
const ALL_VEGETA_MESHES:[&str;39] = [
    "ken_bingo_mouth", "ken_bingo_mouth2", "ken_bingo_y", "ken_bingo", "ken_bingo_b", "ken_bingo_p", "ken_facen_mouth", "ken_final_mouth", "ken_finalblink",
    "ken_finalblink_b", "ken_finalblink_y", "ken_finalblink_p", "ken_hurt_mouth", "ken_hurtblink", "ken_hurtblink_y", "ken_hurtblink_b", "ken_hurtblink_p", "ken_laugh_mouth",
    "ken_laughblink", "ken_laughblink_y", "ken_laughblink_b","ken_laughblink_p", "ken_openblink", "ken_openblink_y", "ken_openblink_b", "ken_openblink_p", "ken_saiyan_mouth",
    "ken_smile2_mouth", "ken_smile2blink", "ken_smile2blink_y", "ken_smile2blink_b", "ken_smile2blink_p", "ken_smile_mouth", "ken_hair1", "ken_hair2",
    "ken_hair3", "ken_hair4", "ken_hair5", "ken_earring_p",
];


pub unsafe fn disable_all_face_meshes(module_accessor: *mut BattleObjectModuleAccessor){
    for mesh in ALL_VEGETA_MESHES {
        if mesh == "ken_earring_p"{
            continue
        }
        ModelModule::set_mesh_visibility(module_accessor, Hash40::new(mesh), false);
    }
}



pub unsafe fn base_neutral_face(module_accessor: *mut app::BattleObjectModuleAccessor){
    disable_all_face_meshes(module_accessor);
    ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ken_facen_mouth"), true);
    ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ken_openblink"), true);
    ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ken_hair1"), true);
}

pub unsafe fn ssj_neutral_face(module_accessor: *mut app::BattleObjectModuleAccessor){
    disable_all_face_meshes(module_accessor);
    ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ken_facen_mouth"), true);
    ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ken_openblink_y"), true);
    ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ken_hair2"), true);
}

pub unsafe fn ssjb_neutral_face(module_accessor: *mut app::BattleObjectModuleAccessor){
    disable_all_face_meshes(module_accessor);
    ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ken_facen_mouth"), true);
    ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ken_openblink_b"), true);
    ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ken_hair3"), true);
}

pub unsafe fn ssjbe_neutral_face(module_accessor: *mut app::BattleObjectModuleAccessor){
    disable_all_face_meshes(module_accessor);
    ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ken_facen_mouth"), true);
    ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ken_openblink_b"), true);
    ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ken_hair4"), true);
}

pub unsafe fn ue_neutral_face(module_accessor: *mut app::BattleObjectModuleAccessor){
    disable_all_face_meshes(module_accessor);
    ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ken_facen_mouth"), true);
    ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ken_openblink_p"), true);
    ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ken_hair5"), true);
}

pub unsafe fn ue_smile_face(module_accessor: *mut app::BattleObjectModuleAccessor){
    disable_all_face_meshes(module_accessor);
    ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ken_smile_mouth"), true);
    ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ken_openblink_p"), true);
    ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ken_hair5"), true);
}

pub unsafe fn base_smile_face(module_accessor: *mut app::BattleObjectModuleAccessor){
    disable_all_face_meshes(module_accessor);
    ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ken_smile2_mouth"), true);
    ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ken_openblink"), true);
    ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ken_hair1"), true);
}

pub unsafe fn base_power_attack_face_n(module_accessor: *mut app::BattleObjectModuleAccessor){
    disable_all_face_meshes(module_accessor);
    ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ken_final_mouth"), true);
    ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ken_openblink"), true);
    ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ken_hair1"), true);
}

pub unsafe fn base_power_charge_face_n(module_accessor: *mut app::BattleObjectModuleAccessor){
    disable_all_face_meshes(module_accessor);
    ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ken_hurt_mouth"), true);
    ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ken_openblink"), true);
    ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ken_hair1"), true);
}


const ZERO_VECTOR:Vector3f = Vector3f{x: 0.0, y: 0.0, z: 0.0};



pub unsafe fn ki_charge(module_accessor: &mut BattleObjectModuleAccessor){
    if [hash40("appeal_s_l"), hash40("appeal_s_r")].contains(&MotionModule::motion_kind(module_accessor)){
        change_motion(module_accessor, "ki_charge")
    }
    if MotionModule::motion_kind(module_accessor) == hash40("ki_charge") && MotionModule::frame(module_accessor) == 30.0{
        MotionModule::set_rate(module_accessor, 0.0);
        let eff: u32 = EffectModule::req_follow(module_accessor, Hash40::new("sys_magicball_aura"), smash::phx::Hash40::new("rot"), &Vector3f {x: -2.0*PostureModule::lr(module_accessor), y: -3.0, z: 1.0*PostureModule::lr(module_accessor)}, &ZERO_VECTOR, 2.5, true, 0, 0, 0, 0, 0, true, true) as u32;
        EffectModule::set_scale(module_accessor, eff, &Vector3f{
            x: 3.0,
            y: 3.0,
            z: 5.0
        });
        EffectModule::set_add_vel(module_accessor, eff, &mut Vector3f{
            x: 0.0,
            y: 1.0,
            z: 0.0
        });
        EffectModule::set_rate(module_accessor, eff, 0.7);
    }
}

static mut BUTTON_QCF_TIMER:[f32;8] = [0.0;8];
static mut BUTTON_QCB_TIMER:[f32;8] = [0.0;8];
static mut START_TIMER_QCF:[bool;8] = [false;8];
static mut START_TIMER_QCB:[bool;8] = [false;8];
pub unsafe fn qcf_handle(module_accessor: &mut BattleObjectModuleAccessor){
    let entry_id = get_entry_id(module_accessor);
    let status_kind = StatusModule::status_kind(module_accessor);
        
    if is_qcf(module_accessor) {
        START_TIMER_QCF[entry_id] = true;
    }
    if START_TIMER_QCF[entry_id]{
        BUTTON_QCF_TIMER[entry_id] +=1.0
    }
    if START_TIMER_QCF[entry_id] && BUTTON_QCF_TIMER[entry_id] < 20.0{
        if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_ATTACK) &&
                ![FIGHTER_VEGETA_STATUS_KIND_SUPERDASHKICK].contains(&status_kind){
            StatusModule::change_status_request_from_script(module_accessor, FIGHTER_VEGETA_STATUS_KIND_SUPERDASHKICK, true);
        }
    }
    else{
        BUTTON_QCF_TIMER[entry_id] = 0.0;
        START_TIMER_QCF[entry_id] = false;
    }
    if [hash40("superdashkick")].contains(&MotionModule::motion_kind(module_accessor)){
        if MotionModule::is_end(module_accessor){
            enable_gravity(module_accessor)
        }
        else{
            disable_gravity(module_accessor);
        }
    }
}

pub unsafe fn qcb_handle(module_accessor: &mut BattleObjectModuleAccessor){
    let entry_id = get_entry_id(module_accessor);
    let status_kind = StatusModule::status_kind(module_accessor);

    if is_qcb(module_accessor) {
        START_TIMER_QCB[entry_id] = true;
    }
    if START_TIMER_QCB[entry_id]{
        BUTTON_QCB_TIMER[entry_id] +=1.0
    }
    if START_TIMER_QCB[entry_id] && BUTTON_QCB_TIMER[entry_id] < 20.0{
        if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
            && ![FIGHTER_VEGETA_STATUS_KIND_GALICK_GUN_HOLD, FIGHTER_VEGETA_STATUS_KIND_GALICK_GUN_START, FIGHTER_VEGETA_STATUS_KIND_GALICK_GUN_FIRE].contains(&status_kind){
            StatusModule::change_status_request_from_script(module_accessor, FIGHTER_VEGETA_STATUS_KIND_GALICK_GUN_START, true);
        }
    }
    else{
        BUTTON_QCB_TIMER[entry_id] = 0.0;
        START_TIMER_QCB[entry_id] = false;
    }
}


pub static mut EFFPOS: smash::phx::Vector3f = smash::phx::Vector3f {x:0.0,y:0.0,z:0.0};
pub static mut EFFROT: smash::phx::Vector3f = smash::phx::Vector3f {x:0.0,y:0.0,z:0.0};


#[weapon_frame(agent = WEAPON_KIND_LUCARIO_AURABALL)]
pub fn hadoken(weapon: &mut L2CFighterBase) {
    unsafe{
        let lua_state = weapon.lua_state_agent;
        let weapon_module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
        let owner_module_accessor = &mut *weapon_module_accessor.get_owner_boma();
        let owner_motion_kind = MotionModule::motion_kind(owner_module_accessor);
        let motion_frame = weapon_module_accessor.motion_frame();
        let multiplier = owner_module_accessor.get_float(FIGHTER_VEGETA_INSTANCE_WORK_ID_FLOAT_POWER_MUL);
        AttackModule::set_power_mul(weapon_module_accessor, multiplier);
        if weapon_module_accessor.motion_frame() >= 30.0{
            AttackModule::clear_all(weapon_module_accessor);
            EffectModule::kill_kind(weapon_module_accessor, Hash40::new("sys_killereye_bullet"), false, true);
            EffectModule::kill_kind(weapon_module_accessor, Hash40::new("sys_sscope_bullet"), false, true);
            EffectModule::kill_kind(weapon_module_accessor, Hash40::new("sys_sscope_bullet_max"), false, true);
            return 0.into();
        }
        EffectModule::kill_kind(weapon_module_accessor, Hash40::new("lucario_hadoudan_tail"), false, true);
        EffectModule::kill_kind(weapon_module_accessor, Hash40::new("lucario_hadoudan_hold"), false, true);
        EffectModule::kill_kind(weapon_module_accessor, Hash40::new("lucario_hadoudan_max_sign"), false, true);
        EffectModule::kill_kind(weapon_module_accessor, Hash40::new("lucario_hadoudan_max_hold"), false, true);
        EffectModule::kill_kind(weapon_module_accessor, Hash40::new("lucario_hadoudan_bomb"), false, true);
        EffectModule::kill_kind(weapon_module_accessor, Hash40::new("lucario_hadoudan_max_l"), false, true);
        EffectModule::kill_kind(weapon_module_accessor, Hash40::new("lucario_hadoudan_max_r"), false, true);
        EffectModule::kill_kind(weapon_module_accessor, Hash40::new("lucario_hadoudan"), false, true);

        if owner_module_accessor.status_kind() == *FIGHTER_STATUS_KIND_SPECIAL_S{
            if weapon_module_accessor.motion_frame() == 1.0{
                let bbatk = EffectModule::req_follow(weapon_module_accessor, Hash40::new("sys_sscope_bullet_max"), smash::phx::Hash40::new("top"), &ZERO_VECTOR, &ZERO_VECTOR, 2.25, true, 0, 0, 0, 0, 0, true, true) as u32;
                EffectModule::set_rgb(weapon_module_accessor, bbatk, 0.0, 0.8, 13.0);
            }
            acmd!(lua_state, {
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=9.0, Angle=40, KBG=65, FKB=0, BKB=55, Size=6.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_SPEED, SetWeight=false, ShieldDamage=-7, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=true, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_aura"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_ENERGY)
            });
        }
        if owner_module_accessor.status_kind() == *FIGHTER_STATUS_KIND_SPECIAL_N{
            if weapon_module_accessor.motion_frame() == 1.0{
                let kiblast = EffectModule::req_follow(weapon_module_accessor, Hash40::new("sys_sscope_bullet"), smash::phx::Hash40::new("top"), &ZERO_VECTOR, &ZERO_VECTOR, 1.26, true, 0, 0, 0, 0, 0, true, true) as u32;
            }
            acmd!(lua_state, {
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=3.0, Angle=40, KBG=65, FKB=0, BKB=55, Size=2.83, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_SPEED, SetWeight=false, ShieldDamage=-7, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=true, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M,SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_ENERGY)
            });
        }
        if owner_module_accessor.status_kind() == *FIGHTER_STATUS_KIND_ATTACK_LW4{
            let effect_size = owner_module_accessor.get_float(FIGHTER_VEGETA_INSTANCE_WORK_ID_FLOAT_LW4_EFFECT_SIZE);
            let hitbox_size = owner_module_accessor.get_float(FIGHTER_VEGETA_INSTANCE_WORK_ID_FLOAT_LW4_HITBOX_SIZE);
            let damage = owner_module_accessor.get_float(FIGHTER_VEGETA_INSTANCE_WORK_ID_FLOAT_LW4_DAMAGE);
            if [15.0, 29.0].contains(&owner_module_accessor.motion_frame()){
                let bbatk = EffectModule::req_follow(weapon_module_accessor, Hash40::new("sys_killereye_bullet"), smash::phx::Hash40::new("top"), &ZERO_VECTOR, &Vector3f{x: 180.0, y: 0.0, z: 0.0}, effect_size, true, 0, 0, 0, 0, 0, true, true) as u32;
                EffectModule::set_rgb(weapon_module_accessor, bbatk, 0.0, 0.8, 1.0);
            }
            acmd!(lua_state, {
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=damage, Angle=40, KBG=60, FKB=0, BKB=50, Size=hitbox_size, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_SPEED, SetWeight=false, ShieldDamage=-7, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=true, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L,SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_ENERGY)
            });
        }

    }
}

fn update_effect_from_file() -> String{
    std::fs::read_to_string("sd:/effect.txt").unwrap()
}



unsafe fn print_all_fighters_motions(){
    for i in 0..FighterManager::entry_count(FIGHTER_MANAGER){
        let module_accessor = get_module_accessor_by_entry_id(i);
        println!("[print_all_fighter_motions] Fighter Num: {} Motion {:#x}", i, MotionModule::motion_kind(module_accessor));
    }
}

static mut IS_USED_EFFECT:bool = false;
#[fighter_frame(agent = FIGHTER_KIND_LUCARIO)]
pub fn vegeta_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let lua_state = fighter.lua_state_agent;
        let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
        let status_kind = boma.status_kind();
        let entry_id = get_entry_id(boma);
        let current_form = boma.get_int(FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_CURRENT_FORM);
        let current_form_timer = boma.get_int(FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_CURRENT_FORM_TIMER);
        let multiplier = boma.get_float(FIGHTER_VEGETA_INSTANCE_WORK_ID_FLOAT_POWER_MUL);
        let current_form_aura = boma.get_int(FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_CURRENT_FORM_AURA);
        AttackModule::set_power_mul(boma, multiplier);
        test_variables(boma);
        if boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_S4_START]) {
            if boma.is_infliction_status(*COLLISION_KIND_MASK_HIT) {
                if !boma.is_flag(FIGHTER_VEGETA_INSTANCE_WORK_ID_FLAG_USED_S4_EFFECT) {
                    EffectModule::req_on_joint(boma, Hash40::new("sys_togezoshell_bomb"), smash::phx::Hash40::new("haver"), &ZERO_VECTOR, &ZERO_VECTOR, 1.0, &ZERO_VECTOR, &ZERO_VECTOR, false, 0, 0, 0) as u32;
                    boma.on_flag(FIGHTER_VEGETA_INSTANCE_WORK_ID_FLAG_USED_S4_EFFECT);
                }
            }
        }
        else {
            boma.off_flag(FIGHTER_VEGETA_INSTANCE_WORK_ID_FLAG_USED_S4_EFFECT);
        }
        if boma.is_motion(Hash40::new("win_2")){
            IS_USED_EFFECT = false;
            if !boma.is_motion_end(){
                base_neutral_face(boma);
            }
        }
        else if boma.is_motion(Hash40::new("win_2_wait")){
            if !IS_USED_EFFECT{
                let handle = EffectModule::req_follow(boma, Hash40::new("lucario_aura"), smash::phx::Hash40::new("top"), &ZERO_VECTOR, &ZERO_VECTOR, 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
                EffectModule::set_rate(boma, handle, 3.0);
                EffectModule::set_rgb(boma, handle, 10.0, 0.7, 10.0);
                IS_USED_EFFECT = true;
            }
            ue_smile_face(boma);
        }
        else if boma.is_motion_one_of(&[Hash40::new("win_3"), Hash40::new("win_3_wait")]){
            base_smile_face(boma);
        }
        else{
            base_neutral_face(boma);
        }
        if boma.is_status(*FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD) {
            let mut charge = 0.0;
            if boma.get_int(*FIGHTER_STATUS_ATTACK_WORK_INT_SMASH_HOLD_KEEP_FRAME) == 120{
                boma.inc_int(FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_LW4_CHARGE_FRAME);
                charge = boma.get_int(FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_LW4_CHARGE_FRAME) as f32 / 120.0;
            }
            else{
                charge = 1.0;
            }
            let mut hitbox_size  = (charge * 3.5) + 7.0;
            let mut effect_size = (charge * 0.75) + 1.5;
            let mut damage = (charge * 7.0) + 10.0;
            boma.set_float(hitbox_size, FIGHTER_VEGETA_INSTANCE_WORK_ID_FLOAT_LW4_HITBOX_SIZE);
            boma.set_float(effect_size, FIGHTER_VEGETA_INSTANCE_WORK_ID_FLOAT_LW4_EFFECT_SIZE);
            boma.set_float(damage, FIGHTER_VEGETA_INSTANCE_WORK_ID_FLOAT_LW4_DAMAGE);
            let hold_eff_size = (charge) + 2.0;
            if !boma.is_flag(FIGHTER_VEGETA_INSTANCE_WORK_ID_FLAG_USED_LW4_EFFECT) {
                let charge_1 = EffectModule::req_follow(boma, Hash40::new("sys_sscope_bullet_max"), smash::phx::Hash40::new("haver"), &Vector3f { x: 0.0, y: 0.0, z: 5.0 }, &ZERO_VECTOR, hold_eff_size, true, 0, 0, 0, 0, 0, true, true) as u32;
                let charge_2 = EffectModule::req_follow(boma, Hash40::new("sys_sscope_bullet_max"), smash::phx::Hash40::new("havel"), &Vector3f { x: 0.0, y: 0.0, z: 5.0 }, &ZERO_VECTOR, hold_eff_size, true, 0, 0, 0, 0, 0, true, true) as u32;
                EffectModule::set_rgb(boma, charge_1, 0.0, 0.8, 13.0);
                EffectModule::set_rgb(boma, charge_2, 0.0, 0.8, 13.0);
                boma.on_flag(FIGHTER_VEGETA_INSTANCE_WORK_ID_FLAG_USED_LW4_EFFECT);
                boma.set_int(charge_1 as i32, FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_LW4_RIGHT_EFFECT_HANDLE);
                boma.set_int(charge_2 as i32, FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_LW4_LEFT_EFFECT_HANDLE);
            }
        }
        if boma.is_status(*FIGHTER_STATUS_KIND_ATTACK_LW4_START){
            boma.off_flag(FIGHTER_VEGETA_INSTANCE_WORK_ID_FLAG_USED_LW4_EFFECT);
        }
        if boma.is_status(*FIGHTER_STATUS_KIND_ATTACK_LW4) {
            if !boma.is_flag(FIGHTER_VEGETA_INSTANCE_WORK_ID_FLAG_USED_LW4_EFFECT) && boma.get_int(FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_LW4_CHARGE_FRAME) == 0 {
                let charge_1 = EffectModule::req_follow(boma, Hash40::new("sys_sscope_bullet_max"), smash::phx::Hash40::new("haver"), &Vector3f { x: 0.0, y: 0.0, z: 5.0 }, &ZERO_VECTOR, 2.0, true, 0, 0, 0, 0, 0, true, true) as u32;
                let charge_2 = EffectModule::req_follow(boma, Hash40::new("sys_sscope_bullet_max"), smash::phx::Hash40::new("havel"), &Vector3f { x: 0.0, y: 0.0, z: 5.0 }, &ZERO_VECTOR, 2.0, true, 0, 0, 0, 0, 0, true, true) as u32;
                EffectModule::set_rgb(boma, charge_1, 0.0, 0.8, 13.0);
                EffectModule::set_rgb(boma, charge_2, 0.0, 0.8, 13.0);
                boma.on_flag(FIGHTER_VEGETA_INSTANCE_WORK_ID_FLAG_USED_LW4_EFFECT);
                boma.set_int(charge_1 as i32, FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_LW4_RIGHT_EFFECT_HANDLE);
                boma.set_int(charge_2 as i32, FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_LW4_LEFT_EFFECT_HANDLE);
            }
            if boma.motion_frame() == 14.0 {
                EffectModule::kill(boma, boma.get_int(FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_LW4_RIGHT_EFFECT_HANDLE) as u32, false, true);
            }
            if boma.motion_frame() == 28.0 {
                EffectModule::kill(boma, boma.get_int(FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_LW4_LEFT_EFFECT_HANDLE) as u32, false, true);
            }
        }
        if !boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_LW4, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, *FIGHTER_STATUS_KIND_SPECIAL_S]) {
            boma.set_int(0, FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_LW4_CHARGE_FRAME);
            boma.set_float(4.67, FIGHTER_VEGETA_INSTANCE_WORK_ID_FLOAT_LW4_HITBOX_SIZE);
            boma.set_float(1.0, FIGHTER_VEGETA_INSTANCE_WORK_ID_FLOAT_LW4_EFFECT_SIZE);
            boma.set_float(10.0, FIGHTER_VEGETA_INSTANCE_WORK_ID_FLOAT_LW4_DAMAGE);
            boma.off_flag(FIGHTER_VEGETA_INSTANCE_WORK_ID_FLAG_USED_LW4_EFFECT);
            EffectModule::kill_kind(boma, Hash40::new("sys_sscope_bullet_max"), false, true);
        }
        ModelModule::set_mesh_visibility(boma, Hash40::new("ken_earring_p"), true);
        EffectModule::kill_kind(boma, Hash40::new("lucario_hadou"), false, true);
        EffectModule::kill_kind(boma, Hash40::new("lucario_hadou_l"), false, true);
        EffectModule::kill_kind(boma, Hash40::new("lucario_hadou_l_l"), false, true);
        EffectModule::kill_kind(boma, Hash40::new("lucario_hadou_l_r"), false, true);
        EffectModule::kill_kind(boma, Hash40::new("lucario_hadou_m"), false, true);
        GroundModule::set_rhombus_offset(boma, &smash::phx::Vector2f {
            x: 0.0,
            y: -1.0
        });
        if boma.is_grounded(){
            boma.set_int(0, FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_AIR_TIMER);
            boma.off_flag(FIGHTER_VEGETA_INSTANCE_WORK_ID_FLAG_USED_AIR_GALICK_GUN);
        }
        if MotionModule::motion_kind(boma) == hash40("attack_air_lw"){
            //enable_jump(boma);
        }
        //test_variables(boma);
        if current_form != 0 && !boma.is_status(*FIGHTER_STATUS_KIND_SPECIAL_LW){
            boma.inc_int(FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_CURRENT_FORM_TIMER);
        }
        if !FighterManager::is_result_mode(FIGHTER_MANAGER){
            if current_form == 1 {
                ssj_neutral_face(boma);
         //       if !EffectModule::is_exist_effect(boma, current_form_aura as u32){
             //       let ssj_aura = EffectModule::req_follow(boma, Hash40::new("sys_final_aura_charge"), smash::phx::Hash40::new("top"), &ZERO_VECTOR, &ZERO_VECTOR, 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
           //         boma.set_int(ssj_aura as i32, FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_CURRENT_FORM_AURA);
           //         EffectModule::set_rgb(boma, ssj_aura, 0.7, 0.7, 0.0);
              //  }
                boma.set_float(1.1, FIGHTER_VEGETA_INSTANCE_WORK_ID_FLOAT_POWER_MUL);
                if current_form_timer == 1800{
                    boma.set_int(0,FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_CURRENT_FORM);
                    boma.on_flag(FIGHTER_VEGETA_INSTANCE_WORK_ID_FLAG_FORM_TIMER_END);
                    boma.set_int(0, FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_CURRENT_FORM_TIMER);
                    let aura = EffectModule::req_follow(boma, Hash40::new("lucario_aura"), smash::phx::Hash40::new("top"), &ZERO_VECTOR, &ZERO_VECTOR, 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
                    EffectModule::set_rgb(boma, aura, 0.7, 0.7, 0.0);
                    EffectModule::set_alpha(boma, aura, 2.0);
                    EffectModule::set_rate(boma, aura, 5.0);
                }
            }
            else if current_form == 2 {
                ssjb_neutral_face(boma);
                boma.set_float(1.3, FIGHTER_VEGETA_INSTANCE_WORK_ID_FLOAT_POWER_MUL);
                if current_form_timer == 1500{
                    boma.set_int(0,FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_CURRENT_FORM);
                    boma.on_flag(FIGHTER_VEGETA_INSTANCE_WORK_ID_FLAG_FORM_TIMER_END);
                    boma.set_int(0, FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_CURRENT_FORM_TIMER);
                    let aura = EffectModule::req_follow(boma, Hash40::new("lucario_aura"), smash::phx::Hash40::new("top"), &ZERO_VECTOR, &ZERO_VECTOR, 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
                    EffectModule::set_rgb(boma, aura, 1.5, 1.5, 1.5);
                    EffectModule::set_rate(boma, aura, 5.0);
                }
            }
            else if current_form == 3 {
                ue_neutral_face(boma);
                boma.set_float(1.5, FIGHTER_VEGETA_INSTANCE_WORK_ID_FLOAT_POWER_MUL);
                if current_form_timer == 900{
                    boma.set_int(0,FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_CURRENT_FORM);
                    boma.on_flag(FIGHTER_VEGETA_INSTANCE_WORK_ID_FLAG_FORM_TIMER_END);
                    boma.set_int(0, FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_CURRENT_FORM_TIMER);
                    let aura = EffectModule::req_follow(boma, Hash40::new("lucario_aura"), smash::phx::Hash40::new("top"), &ZERO_VECTOR, &ZERO_VECTOR, 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
                    EffectModule::set_rgb(boma, aura, 10.0, 0.7, 10.0);
                    EffectModule::set_rate(boma, aura, 5.0);
                }
            }
            else{
                base_neutral_face(boma);
                boma.set_float(1.0, FIGHTER_VEGETA_INSTANCE_WORK_ID_FLOAT_POWER_MUL);
            }
        }

        if boma.is_status(*FIGHTER_STATUS_KIND_DEAD){
            boma.set_int(0,FIGHTER_VEGETA_INSTANCE_WORK_ID_INT_CURRENT_FORM);
        }
        if boma.is_button_on(Buttons::Special) && !boma.is_status(*FIGHTER_STATUS_KIND_FINAL){
          //  boma.change_status(*FIGHTER_STATUS_KIND_FINAL, false);
        }
        //qcf_handle(boma);
        qcb_handle(boma);
        /*
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL){
            if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_APPEAL_S_L){
                b+=0.1;
                println!("b: {}", b);
            }
            if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_APPEAL_S_R){
                b-=0.1;
                println!("b: {}", b);
            }
        }
        else{
            if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_APPEAL_S_L){
                r+=0.1;
                println!("r: {}", r);
            }
            if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_APPEAL_S_R){
                r-=0.1;
                println!("r: {}", r);
            }
        }
        if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_APPEAL_HI){
            g+=0.1;
            println!("g: {}", g);
        }
        if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_APPEAL_LW){
            g-=0.1;
            println!("g: {}", g);
        }

         */
        if ![FIGHTER_VEGETA_STATUS_KIND_GALICK_GUN_HOLD, FIGHTER_VEGETA_STATUS_KIND_GALICK_GUN_START, FIGHTER_VEGETA_STATUS_KIND_GALICK_GUN_FIRE].contains(&status_kind) {
            CHARGE_TIME[entry_id] = 0.0;
        }
    }
}

pub fn install() {
    smashline::install_agent_frames!(vegeta_frame, hadoken);
}