use crate::utils::*;
use crate::vegeta::CHARGE_TIME;
use smash::app::lua_bind::*;
use smash::app::*;
use smash::hash40;
use smash::lib::lua_const::*;
use smash::lua2cpp::L2CAgentBase;
use smash::phx::Hash40;
use smash::phx::Vector2f;
use smashline::*;

#[acmd_script(
agent = "lucario",
scripts = ["sound_galickgun_start", "sound_galickgun_start_air"],
category = ACMD_SOUND,
low_priority )]
unsafe fn sound_vegeta_galickgun_start(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = fighter.module_accessor;
    let entry_id = get_entry_id(module_accessor);
    acmd!(lua_state, {
        frame(Frame=9)
        if(is_execute){
            PLAY_SE(hash40("se_galickgun_start"))
        }
    });
}


#[acmd_script(
agent = "lucario",
scripts = ["sound_galickgun_fire", "sound_galickgun_fire_air"],
category = ACMD_SOUND,
low_priority )]
unsafe fn sound_vegeta_galickgun_fire(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = fighter.module_accessor;
    let entry_id = get_entry_id(module_accessor);
    acmd!(lua_state, {
        frame(Frame=12)
        if(is_execute){
            PLAY_SE(hash40("se_galickgun_fire"))
        }
    });
}



#[acmd_script(agent = "lucario", script = "sound_bigbangatk", category = ACMD_SOUND, low_priority )]
unsafe fn sound_vegeta_bigbangatk(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = fighter.module_accessor;
    let entry_id = get_entry_id(module_accessor);
    acmd!(lua_state, {
        if(is_execute){
            PLAY_SE(hash40("vc_vegeta_bigbangatk_1"))
        }
        frame(Frame=25)
        if(is_execute){
            PLAY_SE(hash40("se_kiblast_heavy"))
        }
    });
}

#[acmd_script(agent = "lucario", script = "sound_bigbangatk_air", category = ACMD_SOUND, low_priority )]
unsafe fn sound_vegeta_bigbangatk_air(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = fighter.module_accessor;
    let entry_id = get_entry_id(module_accessor);
    acmd!(lua_state, {
        if(is_execute){
            PLAY_SE(hash40("vc_vegeta_bigbangatk_1"))
        }
        frame(Frame=25)
        if(is_execute){
            PLAY_SE(hash40("se_kiblast_heavy"))
        }
    });
}


/*
#[acmd_script(
agent = "lucario",
scripts = ["sound_galickgun_hold"],
category = ACMD_SOUND,
low_priority )]
unsafe fn sound_vegeta_galickgun_hold(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {

    });
}
 */

/*
#[acmd_script(agent = "ken_hadoken", scripts = ["sound_moves", "sound_movew", "sound_movem"], category = ACMD_SOUND, low_priority )]
unsafe fn sound_vegeta_projectile(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {

    });
}
 */

pub fn install() {
    smashline::install_acmd_scripts! {
        sound_vegeta_galickgun_fire,
        sound_vegeta_galickgun_start,
        sound_vegeta_bigbangatk,
        sound_vegeta_bigbangatk_air
    };
}
