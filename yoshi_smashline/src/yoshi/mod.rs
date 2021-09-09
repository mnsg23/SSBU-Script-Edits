use smash::hash40;
use smash::phx::*;
use smash::lib::lua_const::*;
use smash::lua2cpp::L2CAgentBase;
use smash::app::{AttackHeight, sv_animcmd, sv_module_access, lua_bind::*, sv_animcmd::*};
use smash_script::*;
use smashline::*;
use crate::FIGHTER_CUTIN_MANAGER_ADDR;

#[acmd_script(agent = "yoshi", script = "game_attack11", category = ACMD_GAME)]
unsafe fn yoshi_game_attack11(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	acmd!(lua_state, {
		frame(Frame=1)
		FT_MOTION_RATE(FSM=0.5)
		frame(Frame=3)
		FT_MOTION_RATE(FSM=1)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=3.0, Angle=361, KBG=15, FKB=0, BKB=30, Size=1.8, X=0.0, Y=7.0, Z=5.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=3.0, Angle=361, KBG=15, FKB=0, BKB=30, Size=1.8, X=0.0, Y=7.0, Z=8.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_KICK)
			ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=3.0, Angle=180, KBG=10, FKB=0, BKB=20, Size=2.2, X=0.0, Y=7.0, Z=12.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_FIGHTER, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_KICK)
			ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=3.0, Angle=361, KBG=10, FKB=0, BKB=20, Size=2.2, X=0.0, Y=7.0, Z=12.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_KICK)
		}
		wait(Frames=1)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=5)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
		}
		frame(Frame=15)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO)
		}
	});
}

#[acmd_script(agent = "yoshi", script = "game_attack12", category = ACMD_GAME)]
unsafe fn yoshi_game_attack12(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	acmd!(lua_state, {
		frame(Frame=3)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=5.0, Angle=361, KBG=100, FKB=0, BKB=50, Size=3.0, X=0.0, Y=7.0, Z=5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=5.0, Angle=361, KBG=100, FKB=0, BKB=50, Size=3.0, X=0.0, Y=7.0, Z=8.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_KICK)
			ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=5.0, Angle=361, KBG=100, FKB=0, BKB=50, Size=4.0, X=0.0, Y=7.5, Z=13.3, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_KICK)
		}
		wait(Frames=2)
		if(is_excute){
			AttackModule::clear_all()
		}
	});
}

#[acmd_script(agent = "yoshi", script = "game_attackairb", category = ACMD_GAME)]
unsafe fn yoshi_game_attackairb(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	acmd!(lua_state, {
		frame(Frame=1)
		FT_MOTION_RATE(FSM=0.5)
		frame(Frame=6)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=11)
		FT_MOTION_RATE(FSM=1)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=3.5, Angle=367, KBG=100, FKB=30, BKB=0, Size=5.5, X=0.0, Y=5.0, Z=-10.0, X2=0.0, Y2=3.0, Z2=-10.0, Hitlag=0.5, SDI=0.1, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_SLAP, Type=ATTACK_REGION_TAIL)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=3.5, Angle=85, KBG=100, FKB=30, BKB=0, Size=5.5, X=0.0, Y=5.0, Z=-10.0, X2=0.0, Y2=3.0, Z2=-10.0, Hitlag=0.5, SDI=0.1, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_SLAP, Type=ATTACK_REGION_TAIL)
		}
		wait(Frames=1)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=14)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=3.5, Angle=367, KBG=100, FKB=30, BKB=0, Size=5.5, X=0.0, Y=5.0, Z=-10.0, X2=0.0, Y2=3.0, Z2=-10.0, Hitlag=0.5, SDI=0.1, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_SLAP, Type=ATTACK_REGION_TAIL)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=3.5, Angle=85, KBG=100, FKB=30, BKB=0, Size=5.5, X=0.0, Y=5.0, Z=-10.0, X2=0.0, Y2=3.0, Z2=-10.0, Hitlag=0.5, SDI=0.1, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_SLAP, Type=ATTACK_REGION_TAIL)
		}
		wait(Frames=1)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=18)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=6.0, Angle=45, KBG=176, FKB=0, BKB=30, Size=8.0, X=0.0, Y=5.0, Z=-11.0, X2=0.0, Y2=5.0, Z2=-9.0, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_SLAP, Type=ATTACK_REGION_TAIL)
		}
		wait(Frames=2)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=44)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
	});
}

#[acmd_script(agent = "yoshi", script = "game_attackairf", category = ACMD_GAME)]
unsafe fn yoshi_game_attackairf(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	acmd!(lua_state, {
		frame(Frame=1)
		FT_MOTION_RATE(FSM=0.5)
		frame(Frame=4)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=5)
		FT_MOTION_RATE(FSM=1)
		frame(Frame=15)
		if(is_excute){
			HIT_NODE(hash40("head"), HIT_STATUS_XLU)
			HIT_NODE(hash40("snout"), HIT_STATUS_XLU)
		}
		frame(Frame=16)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("head"), Damage=15.0, Angle=361, KBG=90, FKB=0, BKB=20, Size=4.2, X=1.5, Y=-0.5, Z=-0.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_HEAD)
			ATTACK(ID=1, Part=0, Bone=hash40("head"), Damage=15.0, Angle=275, KBG=100, FKB=0, BKB=20, Size=4.0, X=7.0, Y=1.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_HEAD)
		}
		wait(Frames=5)
		if(is_excute){
			HIT_NODE(hash40("head"), HIT_STATUS_NORMAL)
			HIT_NODE(hash40("snout"), HIT_STATUS_NORMAL)
			AttackModule::clear_all()
		}
		frame(Frame=40)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=67)
		if(is_excute){
			sv_battle_object::notify_event_msc_cmd(0x2127e37c07, GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES)
		}
	});
}

#[acmd_script(agent = "yoshi", script = "game_attackairhi", category = ACMD_GAME)]
unsafe fn yoshi_game_attackairhi(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	acmd!(lua_state, {
		frame(Frame=5)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
			ATTACK(ID=0, Part=0, Bone=hash40("tail2"), Damage=12.0, Angle=90, KBG=80, FKB=0, BKB=60, Size=7.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_SLAP, Type=ATTACK_REGION_TAIL)
			ATTACK(ID=1, Part=0, Bone=hash40("tail2"), Damage=12.0, Angle=90, KBG=80, FKB=0, BKB=60, Size=7.5, X=6.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_SLAP, Type=ATTACK_REGION_TAIL)
		}
		wait(Frames=2)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=10)
		FT_MOTION_RATE(FSM=0.75)
		frame(Frame=33)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
	});
}

#[acmd_script(agent = "yoshi", script = "game_attackairlw", category = ACMD_GAME)]
unsafe fn yoshi_game_attackairlw(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	acmd!(lua_state, {
		frame(Frame=14)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		for(12 Iterations){
			if(is_excute){
				ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.5, Angle=270, KBG=100, FKB=10, BKB=0, Size=5.0, X=0.0, Y=1.0, Z=2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=0.2, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_KICK)
				ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=2.5, Angle=265, KBG=100, FKB=10, BKB=0, Size=3.5, X=0.0, Y=2.0, Z=6.5, X2=0.0, Y2=2.0, Z2=-2.5, Hitlag=0.5, SDI=0.2, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_KICK)
			}
			wait(Frames=2)
			if(is_excute){
				AttackModule::clear_all()
			}
		}
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.0, Angle=270, KBG=300, FKB=0, BKB=60, Size=6.5, X=0.0, Y=0.5, Z=2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=1.0, Angle=270, KBG=300, FKB=0, BKB=60, Size=4.5, X=0.0, Y=1.5, Z=6.5, X2=0.0, Y2=1.5, Z2=-2.5, Hitlag=2.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_KICK)
		}
		wait(Frames=2)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=50)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
	});
}

#[acmd_script(agent = "yoshi", script = "game_attackairn", category = ACMD_GAME)]
unsafe fn yoshi_game_attackairn(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	acmd!(lua_state, {
		frame(Frame=3)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
			ATTACK(ID=0, Part=0, Bone=hash40("footr"), Damage=10.0, Angle=361, KBG=95, FKB=0, BKB=40, Size=5.5, X=0.0, Y=-2.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("kneer"), Damage=10.0, Angle=361, KBG=95, FKB=0, BKB=40, Size=5.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_KICK)
			ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=10.0, Angle=361, KBG=95, FKB=0, BKB=40, Size=5.0, X=0.0, Y=3.0, Z=-1.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_KICK)
		}
		frame(Frame=7)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("footr"), Damage=8.0, Angle=361, KBG=100, FKB=0, BKB=30, Size=5.0, X=0.0, Y=-2.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("kneer"), Damage=8.0, Angle=361, KBG=100, FKB=0, BKB=30, Size=4.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_KICK)
			ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=8.0, Angle=361, KBG=100, FKB=0, BKB=30, Size=4.5, X=0.0, Y=3.0, Z=-1.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_KICK)
		}
		frame(Frame=13)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("footr"), Damage=6.0, Angle=361, KBG=100, FKB=0, BKB=30, Size=4.5, X=0.0, Y=-2.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("kneer"), Damage=6.0, Angle=361, KBG=100, FKB=0, BKB=30, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_KICK)
			ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=6.0, Angle=361, KBG=100, FKB=0, BKB=30, Size=4.0, X=0.0, Y=3.0, Z=-1.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_KICK)
		}
		frame(Frame=26)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=38)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
	});
}

#[acmd_script(agent = "yoshi", script = "game_attackdash", category = ACMD_GAME)]
unsafe fn yoshi_game_attackdash(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	acmd!(lua_state, {
		frame(Frame=1)
		FT_MOTION_RATE(FSM=0.5)
		frame(Frame=9)
		FT_MOTION_RATE(FSM=1)
		frame(Frame=10)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("legl"), Damage=11.0, Angle=70, KBG=45, FKB=0, BKB=110, Size=5.0, X=6.1, Y=0.0, Z=-0.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("legl"), Damage=11.0, Angle=70, KBG=45, FKB=0, BKB=110, Size=3.5, X=2.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_KICK)
		}
		wait(Frames=3)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("legl"), Damage=8.0, Angle=70, KBG=55, FKB=0, BKB=90, Size=4.0, X=6.1, Y=0.0, Z=-0.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("legl"), Damage=8.0, Angle=70, KBG=55, FKB=0, BKB=90, Size=2.5, X=2.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_KICK)
		}
		wait(Frames=8)
		if(is_excute){
			AttackModule::clear_all()
		}
		FT_MOTION_RATE(FSM=0.75)
	});
}

#[acmd_script(agent = "yoshi", script = "game_attackhi3", category = ACMD_GAME)]
unsafe fn yoshi_game_attackhi3(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	acmd!(lua_state, {
		frame(Frame=1)
		FT_MOTION_RATE(FSM=0.5)
		frame(Frame=5)
		FT_MOTION_RATE(FSM=1)
		frame(Frame=7)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("tail2"), Damage=7.0, Angle=85, KBG=77, FKB=0, BKB=80, Size=4.0, X=4.5, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_SLAP, Type=ATTACK_REGION_TAIL)
			ATTACK(ID=1, Part=0, Bone=hash40("tail2"), Damage=7.0, Angle=85, KBG=77, FKB=0, BKB=80, Size=4.5, X=1.2, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_SLAP, Type=ATTACK_REGION_TAIL)
			ATTACK(ID=2, Part=0, Bone=hash40("tail1"), Damage=7.0, Angle=85, KBG=77, FKB=0, BKB=80, Size=2.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_SLAP, Type=ATTACK_REGION_TAIL)
		}
		wait(Frames=9)
		if(is_excute){
			AttackModule::clear_all()
		}
	});
}

#[acmd_script(agent = "yoshi", script = "game_attackhi4", category = ACMD_GAME)]
unsafe fn yoshi_game_attackhi4(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	acmd!(lua_state, {
		frame(Frame=1)
		FT_MOTION_RATE(FSM=0.7)
		frame(Frame=8)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
		}
		frame(Frame=11)
		FT_MOTION_RATE(FSM=1)
		if(is_excute){
			HIT_NODE(hash40("toel"), HIT_STATUS_XLU)
			HIT_NODE(hash40("footl"), HIT_STATUS_XLU)
			HIT_NODE(hash40("kneel"), HIT_STATUS_XLU)
			HIT_NODE(hash40("legl"), HIT_STATUS_XLU)
			ATTACK(ID=0, Part=0, Bone=hash40("footl"), Damage=14.0, Angle=85, KBG=82, FKB=0, BKB=60, Size=5.0, X=0.0, Y=-2.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("kneel"), Damage=14.0, Angle=85, KBG=82, FKB=0, BKB=60, Size=4.0, X=2.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_KICK)
			ATTACK(ID=2, Part=0, Bone=hash40("legl"), Damage=14.0, Angle=85, KBG=82, FKB=0, BKB=60, Size=4.0, X=1.0, Y=1.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_KICK)
			ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=14.0, Angle=85, KBG=82, FKB=0, BKB=60, Size=5.0, X=0.0, Y=5.0, Z=5.5, X2=0.0, Y2=9.0, Z2=10.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_KICK)
		}
		wait(Frames=1)
		if(is_excute){
			AttackModule::clear(ID=3, false)
		}
		wait(Frames=5)
		if(is_excute){
			HIT_NODE(hash40("toel"), HIT_STATUS_NORMAL)
			HIT_NODE(hash40("footl"), HIT_STATUS_NORMAL)
			HIT_NODE(hash40("kneel"), HIT_STATUS_NORMAL)
			HIT_NODE(hash40("legl"), HIT_STATUS_NORMAL)
			AttackModule::clear_all()
		}
	});
}

#[acmd_script(agent = "yoshi", script = "game_attacklw3", category = ACMD_GAME)]
unsafe fn yoshi_game_attacklw3(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	acmd!(lua_state, {
		frame(Frame=1)
		FT_MOTION_RATE(FSM=0.25)
		frame(Frame=5)
		FT_MOTION_RATE(FSM=1)
		frame(Frame=7)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("tail1"), Damage=6.0, Angle=361, KBG=110, FKB=0, BKB=20, Size=3.5, X=1.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.5, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_SLAP, Type=ATTACK_REGION_TAIL)
			ATTACK(ID=1, Part=0, Bone=hash40("tail2"), Damage=6.0, Angle=361, KBG=110, FKB=0, BKB=20, Size=3.5, X=3.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.5, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_SLAP, Type=ATTACK_REGION_TAIL)
			ATTACK(ID=2, Part=0, Bone=hash40("tail3"), Damage=6.0, Angle=361, KBG=110, FKB=0, BKB=20, Size=3.0, X=3.5, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.5, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_SLAP, Type=ATTACK_REGION_TAIL)
			AttackModule::set_attack_height_all(AttackHeight(*ATTACK_HEIGHT_LOW), false)
		}
		frame(Frame=10)
		if(is_excute){
			AttackModule::clear_all()
		}
		FT_MOTION_RATE(FSM=0.9)
		frame(Frame=20)
		FT_MOTION_RATE(FSM=0.7)
	});
}

#[acmd_script(agent = "yoshi", script = "game_attacklw4", category = ACMD_GAME)]
unsafe fn yoshi_game_attacklw4(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	acmd!(lua_state, {
		frame(Frame=1)
		FT_MOTION_RATE(FSM=0.5)
		frame(Frame=3)
		FT_MOTION_RATE(FSM=1)
		frame(Frame=4)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
		}
		frame(Frame=7)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=13.0, Angle=25, KBG=85, FKB=0, BKB=40, Size=3.5, X=0.0, Y=3.0, Z=7.0, X2=0.0, Y2=3.0, Z2=10.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_SLAP, Type=ATTACK_REGION_TAIL)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=13.0, Angle=25, KBG=85, FKB=0, BKB=40, Size=2.0, X=0.0, Y=2.0, Z=4.0, X2=0.0, Y2=2.0, Z2=14.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_SLAP, Type=ATTACK_REGION_TAIL)
			AttackModule::set_attack_height_all(AttackHeight(*ATTACK_HEIGHT_LOW), false)
		}
		wait(Frames=2)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=22)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=13.0, Angle=25, KBG=85, FKB=0, BKB=40, Size=3.5, X=0.0, Y=3.0, Z=-7.0, X2=0.0, Y2=3.0, Z2=-10.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_SLAP, Type=ATTACK_REGION_TAIL)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=13.0, Angle=25, KBG=85, FKB=0, BKB=40, Size=2.0, X=0.0, Y=2.0, Z=-3.0, X2=0.0, Y2=2.0, Z2=-13.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_SLAP, Type=ATTACK_REGION_TAIL)
			AttackModule::set_attack_height_all(AttackHeight(*ATTACK_HEIGHT_LOW), false)
		}
		wait(Frames=2)
		if(is_excute){
			AttackModule::clear_all()
		}
	});
}

#[acmd_script(agent = "yoshi", script = "game_attacks3", category = ACMD_GAME)]
unsafe fn yoshi_game_attacks3(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	acmd!(lua_state, {
		frame(Frame=5)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("tail1"), Damage=8.0, Angle=45, KBG=98, FKB=0, BKB=50, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_SLAP, Type=ATTACK_REGION_TAIL)
			ATTACK(ID=1, Part=0, Bone=hash40("tail1"), Damage=8.0, Angle=45, KBG=98, FKB=0, BKB=50, Size=3.5, X=4.3, Y=-1.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_SLAP, Type=ATTACK_REGION_TAIL)
			ATTACK(ID=2, Part=0, Bone=hash40("tail2"), Damage=8.0, Angle=45, KBG=98, FKB=0, BKB=50, Size=3.0, X=4.5, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_SLAP, Type=ATTACK_REGION_TAIL)
		}
		wait(Frames=3)
		if(is_excute){
			AttackModule::clear_all()
		}
	});
}

#[acmd_script(agent = "yoshi", script = "game_attacks3hi", category = ACMD_GAME)]
unsafe fn yoshi_game_attacks3hi(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	acmd!(lua_state, {
		frame(Frame=5)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("tail1"), Damage=8.0, Angle=45, KBG=98, FKB=0, BKB=50, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_SLAP, Type=ATTACK_REGION_TAIL)
			ATTACK(ID=1, Part=0, Bone=hash40("tail1"), Damage=8.0, Angle=45, KBG=98, FKB=0, BKB=50, Size=3.5, X=4.3, Y=-1.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_SLAP, Type=ATTACK_REGION_TAIL)
			ATTACK(ID=2, Part=0, Bone=hash40("tail2"), Damage=8.0, Angle=45, KBG=98, FKB=0, BKB=50, Size=3.0, X=3.1, Y=0.5, Z=0.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_SLAP, Type=ATTACK_REGION_TAIL)
			AttackModule::set_attack_height_all(AttackHeight(*ATTACK_HEIGHT_HIGH), false)
		}
		wait(Frames=3)
		if(is_excute){
			AttackModule::clear_all()
		}
	});
}

#[acmd_script(agent = "yoshi", script = "game_attacks3lw", category = ACMD_GAME)]
unsafe fn yoshi_game_attacks3lw(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	acmd!(lua_state, {
		frame(Frame=5)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("tail1"), Damage=8.0, Angle=45, KBG=98, FKB=0, BKB=50, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_SLAP, Type=ATTACK_REGION_TAIL)
			ATTACK(ID=1, Part=0, Bone=hash40("tail1"), Damage=8.0, Angle=45, KBG=98, FKB=0, BKB=50, Size=3.5, X=4.3, Y=-1.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_SLAP, Type=ATTACK_REGION_TAIL)
			ATTACK(ID=2, Part=0, Bone=hash40("tail2"), Damage=8.0, Angle=45, KBG=98, FKB=0, BKB=50, Size=3.0, X=3.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_SLAP, Type=ATTACK_REGION_TAIL)
			AttackModule::set_attack_height_all(AttackHeight(*ATTACK_HEIGHT_LOW), false)
		}
		wait(Frames=3)
		if(is_excute){
			AttackModule::clear_all()
		}
	});
}

#[acmd_script(agent = "yoshi", script = "game_attacks4", category = ACMD_GAME)]
unsafe fn yoshi_game_attacks4(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	acmd!(lua_state, {
		frame(Frame=8)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
		}
		frame(Frame=12)
		if(is_excute){
			HIT_NODE(hash40("head"), HIT_STATUS_XLU)
			HIT_NODE(hash40("snout"), HIT_STATUS_XLU)
		}
		frame(Frame=14)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=16.0, Angle=361, KBG=100, FKB=0, BKB=30, Size=4.5, X=0.0, Y=6.5, Z=9.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_HEAD)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=16.0, Angle=361, KBG=100, FKB=0, BKB=30, Size=4.8, X=0.0, Y=6.5, Z=14.4, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_HEAD)
		}
		wait(Frames=3)
		if(is_excute){
			HIT_NODE(hash40("head"), HIT_STATUS_NORMAL)
			HIT_NODE(hash40("snout"), HIT_STATUS_NORMAL)
			AttackModule::clear_all()
		}
	});
}

#[acmd_script(agent = "yoshi", script = "game_attacks4hi", category = ACMD_GAME)]
unsafe fn yoshi_game_attacks4hi(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	acmd!(lua_state, {
		frame(Frame=8)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
		}
		frame(Frame=12)
		if(is_excute){
			HIT_NODE(hash40("head"), HIT_STATUS_XLU)
			HIT_NODE(hash40("snout"), HIT_STATUS_XLU)
		}
		frame(Frame=14)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=16.0, Angle=361, KBG=100, FKB=0, BKB=30, Size=4.5, X=0.0, Y=11.0, Z=8.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_HEAD)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=16.0, Angle=361, KBG=100, FKB=0, BKB=30, Size=4.8, X=0.0, Y=11.5, Z=12.4, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_HEAD)
		}
		wait(Frames=3)
		if(is_excute){
			HIT_NODE(hash40("head"), HIT_STATUS_NORMAL)
			HIT_NODE(hash40("snout"), HIT_STATUS_NORMAL)
			AttackModule::clear_all()
		}
	});
}

#[acmd_script(agent = "yoshi", script = "game_attacks4lw", category = ACMD_GAME)]
unsafe fn yoshi_game_attacks4lw(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	acmd!(lua_state, {
		frame(Frame=8)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
		}
		frame(Frame=12)
		if(is_excute){
			HIT_NODE(hash40("head"), HIT_STATUS_XLU)
			HIT_NODE(hash40("snout"), HIT_STATUS_XLU)
		}
		frame(Frame=14)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=16.0, Angle=361, KBG=100, FKB=0, BKB=30, Size=4.5, X=0.0, Y=4.2, Z=8.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_HEAD)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=16.0, Angle=361, KBG=100, FKB=0, BKB=30, Size=4.8, X=0.0, Y=4.2, Z=13.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_HEAD)
		}
		wait(Frames=3)
		if(is_excute){
			HIT_NODE(hash40("head"), HIT_STATUS_NORMAL)
			HIT_NODE(hash40("snout"), HIT_STATUS_NORMAL)
			AttackModule::clear_all()
		}
	});
}

#[acmd_script(agent = "yoshi", script = "game_catch", category = ACMD_GAME)]
unsafe fn yoshi_game_catch(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter,	0.4);
	frame(fighter.lua_state_agent, 11.0);
	macros::FT_MOTION_RATE(fighter,	1.0);
	frame(fighter.lua_state_agent, 13.0);
	if macros::is_excute(fighter) {
		GrabModule::set_rebound(fighter.module_accessor, true);
	}
	frame(fighter.lua_state_agent, 14.0);
	if macros::is_excute(fighter) {
		macros::CATCH(fighter, 0, Hash40::new("mouth2"), 3.5, -2.0, -0.5, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED_YOSHI, *COLLISION_SITUATION_MASK_GA);
		macros::CATCH(fighter, 1, Hash40::new("mouth2"), 2.8, 1.2, -0.5, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED_YOSHI, *COLLISION_SITUATION_MASK_GA);
		macros::CATCH(fighter, 2, Hash40::new("top"), 5.5, 0.0, 6.0, 7.5, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED_YOSHI, *COLLISION_SITUATION_MASK_GA);
	}
	macros::game_CaptureCutCommon(fighter);
	wait(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		fighter.clear_lua_stack();
		lua_args!(fighter, *MA_MSC_CMD_GRAB_CLEAR, 2);
		sv_module_access::grab(fighter.lua_state_agent);
		fighter.pop_lua_stack(1);
	}
	frame(fighter.lua_state_agent, 22.0);
	if macros::is_excute(fighter) {
		fighter.clear_lua_stack();
		lua_args!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		sv_module_access::grab(fighter.lua_state_agent);
		fighter.pop_lua_stack(1);
		GrabModule::set_rebound(fighter.module_accessor, false);
	}
	frame(fighter.lua_state_agent, 48.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
	}
}

#[acmd_script(agent = "yoshi", script = "game_catchattack", category = ACMD_GAME)]
unsafe fn yoshi_game_catchattack(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("head"), 1.3, 361, 100, 30, 0, 5.7, 1.6, 0.0, 3.2, None, None, None, 2.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_YOSHI_BITE_HIT, *ATTACK_REGION_BITE);
		AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
	}
	wait(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

#[acmd_script(agent = "yoshi", script = "game_catchdash", category = ACMD_GAME)]
unsafe fn yoshi_game_catchdash(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter,	0.4);
	frame(fighter.lua_state_agent, 11.0);
	macros::FT_MOTION_RATE(fighter,	1.0);
	frame(fighter.lua_state_agent, 15.0);
	if macros::is_excute(fighter) {
		GrabModule::set_rebound(fighter.module_accessor, true);
	}
	frame(fighter.lua_state_agent, 16.0);
	if macros::is_excute(fighter) {
		macros::CATCH(fighter, 0, Hash40::new("mouth2"), 3.3, -2.0, -0.5, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED_YOSHI, *COLLISION_SITUATION_MASK_GA);
		macros::CATCH(fighter, 1, Hash40::new("mouth2"), 2.2, 1.2, -0.5, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED_YOSHI, *COLLISION_SITUATION_MASK_GA);
		macros::CATCH(fighter, 2, Hash40::new("top"), 5.5, 0.0, 6.0, 9.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED_YOSHI, *COLLISION_SITUATION_MASK_GA);
	}
	macros::game_CaptureCutCommon(fighter);
	wait(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		fighter.clear_lua_stack();
		lua_args!(fighter, *MA_MSC_CMD_GRAB_CLEAR, 2);
		sv_module_access::grab(fighter.lua_state_agent);
		fighter.pop_lua_stack(1);
	}
	frame(fighter.lua_state_agent, 24.0);
	if macros::is_excute(fighter) {
		fighter.clear_lua_stack();
		lua_args!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		sv_module_access::grab(fighter.lua_state_agent);
		fighter.pop_lua_stack(1);
		GrabModule::set_rebound(fighter.module_accessor, false);
	}
	frame(fighter.lua_state_agent, 37.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
	}
}

#[acmd_script(agent = "yoshi", script = "game_catchturn", category = ACMD_GAME)]
unsafe fn yoshi_game_catchturn(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter,	0.3);
	frame(fighter.lua_state_agent, 11.0);
	macros::FT_MOTION_RATE(fighter,	1.0);
	frame(fighter.lua_state_agent, 16.0);
	if macros::is_excute(fighter) {
		GrabModule::set_rebound(fighter.module_accessor, true);
	}
	frame(fighter.lua_state_agent, 17.0);
	if macros::is_excute(fighter) {
		macros::CATCH(fighter, 0, Hash40::new("mouth2"), 3.5, -2.0, -0.5, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED_YOSHI, *COLLISION_SITUATION_MASK_GA);
		macros::CATCH(fighter, 1, Hash40::new("mouth2"), 2.8, 1.2, -0.5, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED_YOSHI, *COLLISION_SITUATION_MASK_GA);
		macros::CATCH(fighter, 2, Hash40::new("top"), 5.5, 0.0, 6.0, -8.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED_YOSHI, *COLLISION_SITUATION_MASK_GA);
	}
	macros::game_CaptureCutCommon(fighter);
	wait(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		fighter.clear_lua_stack();
		lua_args!(fighter, *MA_MSC_CMD_GRAB_CLEAR, 2);
		sv_module_access::grab(fighter.lua_state_agent);
		fighter.pop_lua_stack(1);
	}
	frame(fighter.lua_state_agent, 25.0);
	if macros::is_excute(fighter) {
		fighter.clear_lua_stack();
		lua_args!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		sv_module_access::grab(fighter.lua_state_agent);
		fighter.pop_lua_stack(1);
		GrabModule::set_rebound(fighter.module_accessor, false);
	}
	frame(fighter.lua_state_agent, 28.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
	}
}

#[acmd_script(agent = "yoshi", script = "game_cliffattack", category = ACMD_GAME)]
unsafe fn yoshi_game_cliffattack(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	acmd!(lua_state, {
		frame(Frame=20)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=9.0, Angle=45, KBG=20, FKB=0, BKB=90, Size=5.0, X=0.0, Y=5.0, Z=12.5, X2=0.0, Y2=5.0, Z2=1.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_SLAP, Type=ATTACK_REGION_TAIL)
		}
		wait(Frames=3)
		if(is_excute){
			AttackModule::clear_all()
		}
	});
}

#[acmd_script(agent = "yoshi", script = "game_downattackd", category = ACMD_GAME)]
unsafe fn yoshi_game_downattackd(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	acmd!(lua_state, {
		frame(Frame=14)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=7.0, Angle=48, KBG=48, FKB=0, BKB=80, Size=5.0, X=0.0, Y=5.0, Z=11.5, X2=0.0, Y2=5.0, Z2=3.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=8, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_HEAD)
		}
		wait(Frames=2)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=24)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=7.0, Angle=48, KBG=48, FKB=0, BKB=80, Size=5.0, X=0.0, Y=5.0, Z=-18.0, X2=0.0, Y2=5.0, Z2=-5.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=8, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_HEAD)
		}
		wait(Frames=2)
		if(is_excute){
			AttackModule::clear_all()
		}
	});
}

#[acmd_script(agent = "yoshi", script = "game_downattacku", category = ACMD_GAME)]
unsafe fn yoshi_game_downattacku(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	acmd!(lua_state, {
		frame(Frame=14)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=7.0, Angle=48, KBG=48, FKB=0, BKB=80, Size=5.0, X=0.0, Y=5.0, Z=14.0, X2=0.0, Y2=5.0, Z2=3.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=8, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_KICK)
		}
		wait(Frames=2)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=20)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=7.0, Angle=48, KBG=48, FKB=0, BKB=80, Size=5.0, X=0.0, Y=5.0, Z=-15.0, X2=0.0, Y2=5.0, Z2=-4.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=8, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_KICK)
		}
		wait(Frames=2)
		if(is_excute){
			AttackModule::clear_all()
		}
	});
}

#[acmd_script(agent = "yoshi", script = "game_landingairlw", category = ACMD_GAME)]
unsafe fn yoshi_game_landingairlw(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	acmd!(lua_state, {
		frame(Frame=1)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.0, Angle=80, KBG=200, FKB=0, BKB=60, Size=5.0, X=0.0, Y=2.5, Z=-3.0, X2=0.0, Y2=2.5, Z2=7.0, Hitlag=2.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_KICK)
		}
		wait(Frames=1)
		if(is_excute){
			AttackModule::clear_all()
		}
	});
}

#[acmd_script(agent = "yoshi", script = "game_slipattack", category = ACMD_GAME)]
unsafe fn yoshi_game_slipattack(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	acmd!(lua_state, {
		frame(Frame=19)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=5.0, Angle=361, KBG=50, FKB=0, BKB=60, Size=4.5, X=0.0, Y=4.5, Z=12.0, X2=0.0, Y2=4.5, Z2=3.5, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=8, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_SLAP, Type=ATTACK_REGION_TAIL)
		}
		wait(Frames=2)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=26)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=5.0, Angle=361, KBG=50, FKB=0, BKB=60, Size=4.5, X=0.0, Y=4.5, Z=-15.0, X2=0.0, Y2=4.5, Z2=-4.5, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=8, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_SLAP, Type=ATTACK_REGION_TAIL)
		}
		wait(Frames=2)
		if(is_excute){
			AttackModule::clear_all()
		}
	});
}

#[acmd_script(agent = "yoshi", script = "game_specialairlw", category = ACMD_GAME)]
unsafe fn yoshi_game_specialairlw(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	acmd!(lua_state, {
		frame(Frame=15)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_YOSHI_STATUS_SPECIAL_LW_FLAG_LANDING_ENABLE)
		}
		frame(Frame=19)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=16.0, Angle=80, KBG=76, FKB=0, BKB=60, Size=6.8, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=10, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_HIP)
			AttackModule::set_attack_height_all(AttackHeight(*ATTACK_HEIGHT_HIGH), false)
		}
		frame(Frame=20)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=16.0, Angle=80, KBG=76, FKB=0, BKB=60, Size=6.8, X=0.0, Y=5.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=10, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_HIP)
			AttackModule::set_attack_height_all(AttackHeight(*ATTACK_HEIGHT_HIGH), false)
		}
	});
}

#[acmd_script(agent = "yoshi", script = "game_specialairn", category = ACMD_GAME)]
unsafe fn yoshi_game_specialairn(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter,	0.3);
	frame(fighter.lua_state_agent, 11.0);
	macros::FT_MOTION_RATE(fighter,	0.5);
	frame(fighter.lua_state_agent, 15.0);
	macros::FT_MOTION_RATE(fighter,	1.0);
	frame(fighter.lua_state_agent, 16.0);
	if macros::is_excute(fighter) {
		GrabModule::set_rebound(fighter.module_accessor, true);
	}
	frame(fighter.lua_state_agent, 17.0);
	if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
		if macros::is_excute(fighter) {
			macros::CATCH(fighter, 0, Hash40::new("throw"), 4.0, -0.9, -0.5, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_YOSHI, *COLLISION_SITUATION_MASK_GA);
			macros::CATCH(fighter, 1, Hash40::new("throw"), 3.5, -2.0, -0.5, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_YOSHI, *COLLISION_SITUATION_MASK_GA);
			macros::CATCH(fighter, 2, Hash40::new("top"), 5.9, 0.0, 7.0, 9.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_YOSHI, *COLLISION_SITUATION_MASK_GA);
		}
	} else {
		if macros::is_excute(fighter) {
			macros::CATCH(fighter, 0, Hash40::new("mouth2"), 4.0, -0.9, -0.5, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_YOSHI, *COLLISION_SITUATION_MASK_GA);
			macros::CATCH(fighter, 1, Hash40::new("mouth2"), 3.5, -2.0, -0.5, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_YOSHI, *COLLISION_SITUATION_MASK_GA);
			macros::CATCH(fighter, 2, Hash40::new("top"), 5.9, 0.0, 7.0, 9.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_YOSHI, *COLLISION_SITUATION_MASK_GA);
		}
	}
	if macros::is_excute(fighter) {
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 0, 50, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
	}
	frame(fighter.lua_state_agent, 18.0);
	if macros::is_excute(fighter) {
		fighter.clear_lua_stack();
		lua_args!(fighter, *MA_MSC_CMD_GRAB_CLEAR, 2);
		sv_module_access::grab(fighter.lua_state_agent);
		fighter.pop_lua_stack(1);
	}
	frame(fighter.lua_state_agent, 21.0);
	if macros::is_excute(fighter) {
		fighter.clear_lua_stack();
		lua_args!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		sv_module_access::grab(fighter.lua_state_agent);
		fighter.pop_lua_stack(1);
		GrabModule::set_rebound(fighter.module_accessor, false);
	}
}

#[acmd_script(agent = "yoshi", script = "game_specialairn2", category = ACMD_GAME)]
unsafe fn yoshi_game_specialairn2(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 8.0, 361, 100, 0, 0, 0.0, 1.0, *ATTACK_LR_CHECK_F, 1.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 4.0, 361, 100, 0, 0, 0.0, 1.0, *ATTACK_LR_CHECK_F, 1.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
	}
	frame(fighter.lua_state_agent, 7.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_YOSHI_STATUS_SPECIAL_N_FLAG_SWALLOW);
	}
	frame(fighter.lua_state_agent, 21.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_YOSHI_STATUS_SPECIAL_N_FLAG_SPIT);
	}
}

#[acmd_script(agent = "yoshi", script = "game_specialairsend", category = ACMD_GAME)]
unsafe fn yoshi_game_specialairsend(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		fighter.clear_lua_stack();
		lua_args!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
		sv_module_access::damage(fighter.lua_state_agent);
		fighter.pop_lua_stack(1);
		AttackModule::clear_all(fighter.module_accessor);
		JostleModule::set_status(fighter.module_accessor, true);
	}
	frame(fighter.lua_state_agent, 13.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_YOSHI_STATUS_SPECIAL_S_FLAG_HIDE_EGG);
	}
}

#[acmd_script(agent = "yoshi", script = "game_specialairsloop", category = ACMD_GAME)]
unsafe fn yoshi_game_specialairsloop(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		fighter.clear_lua_stack();
		lua_args!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 10);
		sv_module_access::damage(fighter.lua_state_agent);
		fighter.pop_lua_stack(1);
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 80, 65, 0, 80, 3.5, 0.0, 5.6, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 32, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
		JostleModule::set_status(fighter.module_accessor, false);
	}
}

#[acmd_script(agent = "yoshi", script = "game_speciallw", category = ACMD_GAME)]
unsafe fn yoshi_game_speciallw(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	acmd!(lua_state, {
		frame(Frame=7)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=4.0, Angle=85, KBG=100, FKB=80, BKB=0, Size=7.0, X=0.0, Y=7.0, Z=10.0, X2=0.0, Y2=7.0, Z2=0.0, Hitlag=0.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_HIP)
		}
		frame(Frame=8)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=9)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_YOSHI_STATUS_SPECIAL_LW_FLAG_LANDING_ENABLE)
		}
		frame(Frame=27)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=16.0, Angle=80, KBG=76, FKB=0, BKB=60, Size=6.8, X=0.0, Y=5.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=10, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_HIP)
			AttackModule::set_attack_height_all(AttackHeight(*ATTACK_HEIGHT_HIGH), false)
		}
	});
}

#[acmd_script(agent = "yoshi", script = "game_specialn", category = ACMD_GAME)]
unsafe fn yoshi_game_specialn(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter,	0.3);
	frame(fighter.lua_state_agent, 11.0);
	macros::FT_MOTION_RATE(fighter,	0.5);
	frame(fighter.lua_state_agent, 15.0);
	macros::FT_MOTION_RATE(fighter,	1.0);
	frame(fighter.lua_state_agent, 16.0);
	if macros::is_excute(fighter) {
		GrabModule::set_rebound(fighter.module_accessor, true);
	}
	frame(fighter.lua_state_agent, 17.0);
	if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
		if macros::is_excute(fighter) {
			macros::CATCH(fighter, 0, Hash40::new("throw"), 4.0, -0.9, -0.5, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_YOSHI, *COLLISION_SITUATION_MASK_GA);
			macros::CATCH(fighter, 1, Hash40::new("throw"), 3.5, -2.0, -0.5, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_YOSHI, *COLLISION_SITUATION_MASK_GA);
			macros::CATCH(fighter, 2, Hash40::new("top"), 5.9, 0.0, 7.0, 9.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_YOSHI, *COLLISION_SITUATION_MASK_GA);
		}
	} else {
		if macros::is_excute(fighter) {
			macros::CATCH(fighter, 0, Hash40::new("mouth2"), 4.0, -0.9, -0.5, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_YOSHI, *COLLISION_SITUATION_MASK_GA);
			macros::CATCH(fighter, 1, Hash40::new("mouth2"), 3.5, -2.0, -0.5, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_YOSHI, *COLLISION_SITUATION_MASK_GA);
			macros::CATCH(fighter, 2, Hash40::new("top"), 5.9, 0.0, 7.0, 9.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_YOSHI, *COLLISION_SITUATION_MASK_GA);
		}
	}
	if macros::is_excute(fighter) {
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 0, 50, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
	}
	frame(fighter.lua_state_agent, 18.0);
	if macros::is_excute(fighter) {
		fighter.clear_lua_stack();
		lua_args!(fighter, *MA_MSC_CMD_GRAB_CLEAR, 2);
		sv_module_access::grab(fighter.lua_state_agent);
		fighter.pop_lua_stack(1);
	}
	frame(fighter.lua_state_agent, 21.0);
	if macros::is_excute(fighter) {
		fighter.clear_lua_stack();
		lua_args!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		sv_module_access::grab(fighter.lua_state_agent);
		fighter.pop_lua_stack(1);
		GrabModule::set_rebound(fighter.module_accessor, false);
	}
}

#[acmd_script(agent = "yoshi", script = "game_specialn2", category = ACMD_GAME)]
unsafe fn yoshi_game_specialn2(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 8.0, 361, 100, 0, 0, 0.0, 1.0, *ATTACK_LR_CHECK_F, 1.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 4.0, 361, 100, 0, 0, 0.0, 1.0, *ATTACK_LR_CHECK_F, 1.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
	}
	frame(fighter.lua_state_agent, 7.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_YOSHI_STATUS_SPECIAL_N_FLAG_SWALLOW);
	}
	frame(fighter.lua_state_agent, 21.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_YOSHI_STATUS_SPECIAL_N_FLAG_SPIT);
	}
}

#[acmd_script(agent = "yoshi", script = "game_specialsend", category = ACMD_GAME)]
unsafe fn yoshi_game_specialsend(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		fighter.clear_lua_stack();
		lua_args!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
		sv_module_access::damage(fighter.lua_state_agent);
		fighter.pop_lua_stack(1);
		AttackModule::clear_all(fighter.module_accessor);
		JostleModule::set_status(fighter.module_accessor, true);
	}
	frame(fighter.lua_state_agent, 13.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_YOSHI_STATUS_SPECIAL_S_FLAG_HIDE_EGG);
	}
}

#[acmd_script(agent = "yoshi", script = "game_specialsloop", category = ACMD_GAME)]
unsafe fn yoshi_game_specialsloop(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		fighter.clear_lua_stack();
		lua_args!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 10);
		sv_module_access::damage(fighter.lua_state_agent);
		fighter.pop_lua_stack(1);
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 80, 65, 0, 80, 3.5, 0.0, 5.6, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 32, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
		JostleModule::set_status(fighter.module_accessor, false);
	}
}

#[acmd_script(agent = "yoshi", script = "game_throwb", category = ACMD_GAME)]
unsafe fn yoshi_game_throwb(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 9.0, 45, 60, 0, 80, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_THROW);
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
	}
	frame(fighter.lua_state_agent, 19.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_REVERSE_LR_FINISH_CAMERA_THROW_ORBIT);
		macros::CHECK_FINISH_CAMERA(fighter, 14, 12);
		let fighter_cutin_manager = *(FIGHTER_CUTIN_MANAGER_ADDR as *mut *mut smash::app::FighterCutInManager);
		FighterCutInManager::set_throw_finish_zoom_rate(fighter_cutin_manager, 1.1);
		FighterCutInManager::set_throw_finish_offset(fighter_cutin_manager, Vector3f{x: -6.0, y: 4.5, z: 0.0});
	}
	frame(fighter.lua_state_agent, 20.0);
	if macros::is_excute(fighter) {
		macros::REVERSE_LR(fighter);
		macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
	}
}

#[acmd_script(agent = "yoshi", script = "game_throwf", category = ACMD_GAME)]
unsafe fn yoshi_game_throwf(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 9.0, 45, 60, 0, 80, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_THROW);
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
	}
	frame(fighter.lua_state_agent, 14.0);
	if macros::is_excute(fighter) {
		macros::CHECK_FINISH_CAMERA(fighter, 13, 14);
		let fighter_cutin_manager = *(FIGHTER_CUTIN_MANAGER_ADDR as *mut *mut smash::app::FighterCutInManager);
		FighterCutInManager::set_throw_finish_zoom_rate(fighter_cutin_manager, 1.1);
		FighterCutInManager::set_throw_finish_offset(fighter_cutin_manager, Vector3f{x: 6.0, y: 4.5, z: 0.0});
	}
	frame(fighter.lua_state_agent, 15.0);
	if macros::is_excute(fighter) {
		macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
	}
}

#[acmd_script(agent = "yoshi", script = "game_throwhi", category = ACMD_GAME)]
unsafe fn yoshi_game_throwhi(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 8.0, 90, 65, 0, 80, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_THROW);
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
	}
	frame(fighter.lua_state_agent, 12.0);
	if macros::is_excute(fighter) {
		macros::CHECK_FINISH_CAMERA(fighter, 2, 25);
		let fighter_cutin_manager = *(FIGHTER_CUTIN_MANAGER_ADDR as *mut *mut smash::app::FighterCutInManager);
		FighterCutInManager::set_throw_finish_zoom_rate(fighter_cutin_manager, 1.3);
		FighterCutInManager::set_throw_finish_offset(fighter_cutin_manager, Vector3f{x: 0.0, y: 6.0, z: 0.0});
	}
	frame(fighter.lua_state_agent, 13.0);
	if macros::is_excute(fighter) {
		macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
	}
}

#[acmd_script(agent = "yoshi", script = "game_throwlw", category = ACMD_GAME)]
unsafe fn yoshi_game_throwlw(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 6.0, 85, 40, 0, 80, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_THROW);
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
	}
	frame(fighter.lua_state_agent, 14.0);
	if macros::is_excute(fighter) {
		macros::CHECK_FINISH_CAMERA(fighter, -1, 6);
		let fighter_cutin_manager = *(FIGHTER_CUTIN_MANAGER_ADDR as *mut *mut smash::app::FighterCutInManager);
		FighterCutInManager::set_throw_finish_zoom_rate(fighter_cutin_manager, 1.4);
		FighterCutInManager::set_throw_finish_offset(fighter_cutin_manager, Vector3f{x: 0.0, y: 4.0, z: 0.0});
		fighter.clear_lua_stack();
		lua_args!(fighter, 8, 1);
		sv_animcmd::FT_CATCH_STOP(fighter.lua_state_agent);
		fighter.pop_lua_stack(1);
	}
	frame(fighter.lua_state_agent, 15.0);
	if macros::is_excute(fighter) {
		macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
	}
	frame(fighter.lua_state_agent, 34.0);
	macros::FT_MOTION_RATE(fighter,	0.6);
}

#[acmd_script(agent = "yoshi_star", script = "game_move", category = ACMD_GAME)]
unsafe fn yoshi_star_game_move(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	acmd!(lua_state, {
		frame(Frame=3)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=4.0, Angle=40, KBG=45, FKB=0, BKB=60, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=6, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=true, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_NONE)
		}
		frame(Frame=13)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=18)
		if(is_excute){
			WorkModule::on_flag(Flag=WEAPON_YOSHI_STAR_STATUS_WORK_FLAG_REMOVE)
		}
	});
}

#[acmd_script(agent = "yoshi_tamago", script = "game_burst", category = ACMD_GAME)]
unsafe fn yoshi_tamago_game_burst(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 70, 75, 0, 70, 6.5, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, -3, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_YOSHI_EGG_HIT, *ATTACK_REGION_OBJECT);
		ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_explosion"), 0, false, 0);
	}
	wait(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

#[acmd_script(agent = "yoshi_tamago", script = "game_throwed", category = ACMD_GAME)]
unsafe fn yoshi_tamago_game_throwed(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	acmd!(lua_state, {
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=6.0, Angle=70, KBG=75, FKB=0, BKB=70, Size=2.5, X=0.0, Y=-1.0, Z=0.0, X2=0.0, Y2=0.0, Z2=0.0, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-3, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_YOSHI_EGG_HIT, Type=ATTACK_REGION_OBJECT)
		}
	});
}

pub fn install() {
	smashline::install_acmd_scripts!(
		yoshi_game_attack11,
		yoshi_game_attack12,
		yoshi_game_attackairb,
		yoshi_game_attackairf,
		yoshi_game_attackairhi,
		yoshi_game_attackairlw,
		yoshi_game_attackairn,
		yoshi_game_attackdash,
		yoshi_game_attackhi3,
		yoshi_game_attackhi4,
		yoshi_game_attacklw3,
		yoshi_game_attacklw4,
		yoshi_game_attacks3,
		yoshi_game_attacks3hi,
		yoshi_game_attacks3lw,
		yoshi_game_attacks4,
		yoshi_game_attacks4hi,
		yoshi_game_attacks4lw,
		yoshi_game_catch,
		yoshi_game_catchattack,
		yoshi_game_catchdash,
		yoshi_game_catchturn,
		yoshi_game_cliffattack,
		yoshi_game_downattackd,
		yoshi_game_downattacku,
		yoshi_game_landingairlw,
		yoshi_game_slipattack,
		yoshi_game_specialairlw,
		yoshi_game_specialairn,
		yoshi_game_specialairn2,
		yoshi_game_specialairsend,
		yoshi_game_specialairsloop,
		yoshi_game_speciallw,
		yoshi_game_specialn,
		yoshi_game_specialn2,
		yoshi_game_specialsend,
		yoshi_game_specialsloop,
		yoshi_game_throwb,
		yoshi_game_throwf,
		yoshi_game_throwhi,
		yoshi_game_throwlw,
		yoshi_star_game_move,
		yoshi_tamago_game_burst,
		yoshi_tamago_game_throwed,
	);
}
