use smash::{
    *,
    hash40,
    phx::{Hash40, Vector3f},
    lib::{lua_const::*, L2CValue},
    app::{lua_bind::{*, StatusModule::*}, sv_animcmd::{frame, wait}, BattleObjectModuleAccessor},
    lua2cpp::{L2CFighterCommon, L2CAgentBase}
};
use smashline::acmd;


pub struct Projectile{
    fighter: L2CAgentBase,
    damage: f32,
    size:f32,
    x: f32,
    y: f32,
    speed: f32,
    kbg: f32,
    bkb: f32,
    fkb: f32,
    launch: bool,
    start_joint: String
}


impl Projectile{
    pub fn new(fighter: L2CAgentBase, damage:f32, size:f32, x: f32, y: f32, speed:f32, kbg: f32, bkb: f32, fkb: f32, launch: bool, start_joint:&str) -> Self{
       Projectile{
           fighter,
           damage,
           size,
           x,
           y,
           speed,
           kbg,
           bkb,
           fkb,
           launch,
           start_joint: start_joint.to_string()
       }
    }
    pub fn launch(&mut self){
        self.launch = true;
    }
    pub fn restart(&mut self){
        self.launch = false;
        self.x = 0.0;
        self.y = 0.0;
    }
   pub fn update(&mut self){
        if !self.launch{
            return;
        }
        let lua_state = self.fighter.lua_state_agent;
        acmd!(lua_state, {
            ATTACK(ID=0, Part=0, Bone=hash40(self.start_joint.as_str()), Damage=self.damage, Angle=361, KBG=self.kbg, FKB=self.fkb, BKB=self.bkb, Size=self.size, X=self.x, Y=self.y, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=1, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_ENERGY)
        });
       self.x += self.speed;
       println!("{}", self.x);
    }
}