#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![feature(asm)]
#![feature(const_mut_refs)]
#![allow(warnings)]
#![allow(dead_code)]


mod utils;
use utils::*;
mod vegeta;
mod vegeta_effect;
mod vegeta_game;
mod vegeta_status;
mod vegeta_sound;

//mod vegeta_sound;
use std::ffi::{CStr, CString};
use std::mem::transmute;
use std::ptr::hash;
use skyline::hooks::{getRegionAddress, Region};
use skyline::logging::print_stack_trace;
use smash::app::{ArticleOperationTarget, BattleObjectModuleAccessor, ItemKind, utility};
use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::phx::Hash40;
use utils::*;
use skyline::nn;
use smash_utils::bomaext::BomaExt;

pub fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

/*
MOVESET PLAN
Input grab: "Shall I tell you where you miscalculated"
jab: dbfz ss vegeta lights
up smash: dbfz vegeta 2h
down smash: gogeta blue level 1 ki blasts, turnauround mid way
side smash: galick impact (?) frieza rush
side special: big bang attack
qcf + attack: dbfz super dash kick
qcb + attack: dbfz ssgss vegeta scale 1 to 10
qcb + special: galick gun
foward throw: dbfz z broly 214lmh
back throw: throw and ki blast backwards
down throw: pin opponent to ground, ki blast downward
up throw: throw directly up then quick beam attack
nair: dbfz ss/ssgss vegeta j.m
fair: dbfz base vegeta j.h
uair: dbfz vegeta j2h
dair: ss vegeta j2l
bair: ?
up tilt: upwards blast
down tilt: dbfz bardock 2m
side tilt: Uppercut -> Roundhouse Kick
 */

static mut CONSTANT_OFFSET: usize = 0x3727390; //13.0.1
static mut INT_OFFSET: usize = 0x4e5380; // 12.0.0
static mut FLOAT_OFFSET: usize = 0x4e53C0; // 12.0.0

pub static INT_SEARCH_CODE: &[u8] = &[
    0x00, 0x1c, 0x40, 0xf9, 0x08, 0x00, 0x40, 0xf9, 0x03, 0x11, 0x40, 0xf9,
];
pub static FLOAT_SEARCH_CODE: &[u8] = &[
    0x00, 0x1c, 0x40, 0xf9, 0x08, 0x00, 0x40, 0xf9, 0x03, 0x19, 0x40, 0xf9,
];


pub fn rust_str_to_cstr(string: &str) -> *const u8 {
    CString::new(string).unwrap().into_raw() as *const u8
}

#[skyline::hook(offset = CONSTANT_OFFSET)]
unsafe fn declare_const_hook(unk: u64, constant: *const u8, mut value: u32) {
    let str = CStr::from_ptr(constant as _).to_str().unwrap();
    if str.contains("FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_TERM") {
        value = 0x100000D5;
    }
    if str.contains("FIGHTER_KOOPA_STATUS_KIND_MAX") {
        value = 0x1ef
    }
    if str.contains("FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_TERM"){
        value = 0x100000D1;
    }
    if str.contains("FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_TERM"){
        value = 0x57;
    }
    original!()(unk, constant, value)
}


#[skyline::hook(offset = INT_OFFSET)]
pub unsafe fn get_param_int_replace(work_module: u64, param_type: u64, param_hash: u64) -> i32 {
    let mut boma = *(*((work_module as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
    let ret = original!()(work_module, param_type, param_hash);
    ret
}

#[skyline::hook(offset = FLOAT_OFFSET)]
pub unsafe fn get_param_float_replace(work_module: u64, param_type: u64, param_hash: u64) -> f32 {
    let mut boma_ptr = (*((work_module as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
    let mut boma = &mut *boma_ptr;
    let ret = original!()(work_module, param_type, param_hash);
    if boma.kind() == *WEAPON_KIND_LUCARIO_AURABALL{
        if [hash40("min_speed"), hash40("max_speed")].contains(&param_hash){
            return 3.5;
        }
        if [hash40("charge_min_scale"), hash40("charge_max_scale"),
            hash40("charge_min_scale_shoot"), hash40("charge_max_scale_shoot")].contains(&param_hash){
            return 1.0;
        }
    }
    if boma.kind() == *FIGHTER_KIND_LUCARIO{
       if param_hash == hash40("run_speed_max"){
           return 2.167;
       }
        if param_hash == hash40("run_accel_mul"){
           return 0.099;
       }
    }
    ret
}

#[skyline::hook(offset=0x490720)]
unsafe fn set_mesh_visibility_2(model_module: u64, mesh: Hash40, vis: bool){
    let mut boma = &mut *(*((model_module as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);

    if boma.kind() == *FIGHTER_KIND_LUCARIO && !get_all_vegeta_meshes().contains(&mesh){
        return;
    }
    original!()(model_module, mesh, vis);
}

fn installAll() {
    unsafe {
        let text_ptr = getRegionAddress(Region::Text) as *const u8;
        let text_size = (getRegionAddress(Region::Rodata) as usize) - (text_ptr as usize);
        let text = std::slice::from_raw_parts(text_ptr, text_size);
        if let Some(offset) = find_subsequence(text, FLOAT_SEARCH_CODE) {
            FLOAT_OFFSET = offset;
        }
        if let Some(offset) = find_subsequence(text, INT_SEARCH_CODE) {
            INT_OFFSET = offset;
        }
    }

    vegeta::install();
    vegeta_effect::install();
    vegeta_status::install();
    vegeta_game::install();
    vegeta_sound::install();
    skyline::install_hooks!(declare_const_hook, get_param_float_replace, set_mesh_visibility_2);
}
#[skyline::main(name = "vegeta_moveset")]
pub fn main() {
    installAll();
}
