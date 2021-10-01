use smash::hash40;
use smash::phx::*;
use smash::lib::lua_const::*;
use smash::lua2cpp::L2CAgentBase;
use smash::app::{AttackHeight, lua_bind::*, sv_animcmd::*};
use smash_script::*;
use smashline::*;
use crate::FIGHTER_CUTIN_MANAGER_ADDR;

#[acmd_script(agent = "zelda", script = "game_attack11", category = ACMD_GAME)]
unsafe fn zelda_game_attack11(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	acmd!(lua_state, {
		frame(Frame=1)
		FT_MOTION_RATE(FSM=0.3333333)
		frame(Frame=4)
		FT_MOTION_RATE(FSM=1)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.5, Angle=361, KBG=15, FKB=0, BKB=25, Size=2.8, X=0.0, Y=12.0, Z=4.0, X2=0.0, Y2=5.5, Z2=4.0, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=2.5, Angle=361, KBG=15, FKB=0, BKB=25, Size=2.8, X=0.0, Y=12.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
			ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=2.5, Angle=180, KBG=10, FKB=0, BKB=20, Size=3.2, X=0.0, Y=11.8, Z=10.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_FIGHTER, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
			ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=2.5, Angle=361, KBG=10, FKB=0, BKB=20, Size=3.2, X=0.0, Y=11.8, Z=10.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
			AttackModule::set_add_reaction_frame(ID=0, Frames=4.0, Unk=false)
			AttackModule::set_add_reaction_frame(ID=1, Frames=4.0, Unk=false)
			AttackModule::set_add_reaction_frame(ID=2, Frames=4.0, Unk=false)
			AttackModule::set_add_reaction_frame(ID=3, Frames=4.0, Unk=false)
		}
		wait(Frames=2)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=7)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.5, Angle=361, KBG=15, FKB=0, BKB=25, Size=3.2, X=0.0, Y=11.8, Z=5.0, X2=0.0, Y2=5.5, Z2=5.0, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=2.5, Angle=361, KBG=15, FKB=0, BKB=25, Size=3.2, X=0.0, Y=11.8, Z=8.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
			ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=2.5, Angle=180, KBG=10, FKB=0, BKB=20, Size=3.5, X=0.0, Y=11.5, Z=11.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_FIGHTER, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
			ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=2.5, Angle=361, KBG=10, FKB=0, BKB=20, Size=3.5, X=0.0, Y=11.5, Z=11.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
			AttackModule::set_add_reaction_frame(ID=0, Frames=4.0, Unk=false)
			AttackModule::set_add_reaction_frame(ID=1, Frames=4.0, Unk=false)
			AttackModule::set_add_reaction_frame(ID=2, Frames=4.0, Unk=false)
			AttackModule::set_add_reaction_frame(ID=3, Frames=4.0, Unk=false)
		}
		wait(Frames=1)
		if(is_excute){
			AttackModule::clear_all()
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100)
		}
		wait(Frames=1)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
		}
		wait(Frames=3)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_RESTART)
		}
	});
}

#[acmd_script(agent = "zelda", script = "game_attack100", category = ACMD_GAME)]
unsafe fn zelda_game_attack100(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	acmd!(lua_state, {
		frame(Frame=1)
		for(1000000 Iterations){
			for(7 Iterations){
				if(is_excute){
					ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=0.2, Angle=361, KBG=10, FKB=0, BKB=15, Size=6.0, X=0.0, Y=8.5, Z=8.0, X2=0.0, Y2=8.5, Z2=13.0, Hitlag=0.5, SDI=0.3, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
					AttackModule::set_add_reaction_frame(ID=0, Frames=4.0, Unk=false)
					ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=8)
				}
				wait(Frames=1)
				if(is_excute){
					AttackModule::clear_all()
					WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK)
				}
				wait(Frames=1)
			}
			if(is_excute){
				ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=0.2, Angle=361, KBG=10, FKB=0, BKB=15, Size=6.0, X=0.0, Y=8.5, Z=8.0, X2=0.0, Y2=8.5, Z2=13.0, Hitlag=0.5, SDI=0.3, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
				AttackModule::set_add_reaction_frame(ID=0, Frames=4.0, Unk=false)
				ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=8)
			}
			wait(Frames=1)
			if(is_excute){
				AttackModule::clear_all()
				WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK)
			}
			wait_loop_clear(0)
			wait(Frames=2)
		}
	});
}

#[acmd_script(agent = "zelda", script = "game_attack100end", category = ACMD_GAME)]
unsafe fn zelda_game_attack100end(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	acmd!(lua_state, {
		frame(Frame=1)
		FT_MOTION_RATE(FSM=0.5)
		frame(Frame=3)
		FT_MOTION_RATE(FSM=1)
		frame(Frame=6)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=3.0, Angle=361, KBG=140, FKB=0, BKB=60, Size=6.0, X=0.0, Y=8.5, Z=10.0, X2=0.0, Y2=8.5, Z2=17.0, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
		}
		wait(Frames=2)
		if(is_excute){
			AttackModule::clear_all()
		}
	});
}

#[acmd_script(agent = "zelda", script = "game_attack100sub", category = ACMD_GAME)]
unsafe fn zelda_game_attack100sub(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	acmd!(lua_state, {
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=0.2, Angle=361, KBG=10, FKB=0, BKB=15, Size=6.0, X=0.0, Y=8.5, Z=8.0, X2=0.0, Y2=8.5, Z2=13.0, Hitlag=0.5, SDI=0.3, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
			AttackModule::set_add_reaction_frame(ID=0, Frames=4.0, Unk=false)
			ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=8)
		}
		wait(Frames=1)
		if(is_excute){
			AttackModule::clear_all()
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK)
		}
	});
}

#[acmd_script(agent = "zelda", script = "game_attackairb", category = ACMD_GAME)]
unsafe fn zelda_game_attackairb(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	acmd!(lua_state, {
		frame(Frame=3)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=6)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=20.0, Angle=361, KBG=86, FKB=0, BKB=50, Size=2.2, X=-1.0, Y=4.4, Z=-11.1, X2=1.0, Y2=4.4, Z2=-11.1, Hitlag=1.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_LL, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
			ATTACK(ID=1, Part=0, Bone=hash40("kneer"), Damage=5.0, Angle=361, KBG=90, FKB=0, BKB=20, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_KICK)
			ATTACK(ID=2, Part=0, Bone=hash40("hip"), Damage=5.0, Angle=361, KBG=90, FKB=0, BKB=20, Size=4.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_KICK)
			ATTACK(ID=3, Part=0, Bone=hash40("toer"), Damage=5.0, Angle=361, KBG=90, FKB=0, BKB=20, Size=5.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_KICK)
			AttackModule::set_optional_hit_effect(ID=0, Hash40::new_raw(0x1484fa7486))
		}
		wait(Frames=1)
		if(is_excute){
			AttackModule::clear(ID=0, false)
		}
		wait(Frames=4)
		if(is_excute){
			AttackModule::clear_all()
		}
		wait(Frames=2)
		FT_MOTION_RATE(FSM=0.9)
		frame(Frame=48)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
	});
}

#[acmd_script(agent = "zelda", script = "game_attackairf", category = ACMD_GAME)]
unsafe fn zelda_game_attackairf(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	acmd!(lua_state, {
		frame(Frame=1)
		FT_MOTION_RATE(FSM=0.5)
		frame(Frame=4)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=7)
		FT_MOTION_RATE(FSM=1)
		frame(Frame=9)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=20.0, Angle=361, KBG=86, FKB=0, BKB=50, Size=2.2, X=-1.0, Y=4.4, Z=11.6, X2=1.0, Y2=4.4, Z2=11.6, Hitlag=1.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_LL, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
			ATTACK(ID=1, Part=0, Bone=hash40("kneel"), Damage=5.0, Angle=361, KBG=90, FKB=0, BKB=20, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_KICK)
			ATTACK(ID=2, Part=0, Bone=hash40("hip"), Damage=5.0, Angle=361, KBG=90, FKB=0, BKB=20, Size=4.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_KICK)
			ATTACK(ID=3, Part=0, Bone=hash40("toel"), Damage=5.0, Angle=361, KBG=90, FKB=0, BKB=20, Size=5.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_KICK)
			AttackModule::set_optional_hit_effect(ID=0, Hash40::new_raw(0x148397b09f))
		}
		wait(Frames=1)
		if(is_excute){
			AttackModule::clear(ID=0, false)
		}
		wait(Frames=4)
		if(is_excute){
			AttackModule::clear_all()
		}
		wait(Frames=2)
		FT_MOTION_RATE(FSM=0.5)
		wait(Frames=4)
		FT_MOTION_RATE(FSM=1)
		frame(Frame=46)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
	});
}

#[acmd_script(agent = "zelda", script = "game_attackairhi", category = ACMD_GAME)]
unsafe fn zelda_game_attackairhi(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	acmd!(lua_state, {
		frame(Frame=5)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=14)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=17.0, Angle=90, KBG=72, FKB=0, BKB=60, Size=9.0, X=0.0, Y=26.5, Z=0.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_BOMB, Type=ATTACK_REGION_MAGIC)
		}
		wait(Frames=3)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=12.0, Angle=90, KBG=72, FKB=0, BKB=60, Size=9.0, X=0.0, Y=26.5, Z=0.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_BOMB, Type=ATTACK_REGION_MAGIC)
		}
		wait(Frames=3)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=54)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
	});
}

#[acmd_script(agent = "zelda", script = "game_attackairlw", category = ACMD_GAME)]
unsafe fn zelda_game_attackairlw(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	acmd!(lua_state, {
		frame(Frame=4)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=14)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("footl"), Damage=18.0, Angle=270, KBG=100, FKB=0, BKB=20, Size=3.2, X=2.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_LL, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
			ATTACK(ID=1, Part=0, Bone=hash40("footl"), Damage=18.0, Angle=270, KBG=100, FKB=0, BKB=20, Size=3.0, X=-1.0, Y=-0.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_LL, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
			ATTACK(ID=2, Part=0, Bone=hash40("kneel"), Damage=6.0, Angle=270, KBG=90, FKB=0, BKB=20, Size=5.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=3, Part=0, Bone=hash40("footl"), Damage=6.0, Angle=270, KBG=90, FKB=0, BKB=20, Size=5.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
		}
		frame(Frame=15)
		if(is_excute){
			AttackModule::clear(ID=0, false)
			AttackModule::clear(ID=1, false)
		}
		frame(Frame=25)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=40)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
	});
}

#[acmd_script(agent = "zelda", script = "game_attackairn", category = ACMD_GAME)]
unsafe fn zelda_game_attackairn(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	acmd!(lua_state, {
		frame(Frame=1)
		FT_MOTION_RATE(FSM=0.5)
		frame(Frame=3)
		FT_MOTION_RATE(FSM=1)
		frame(Frame=4)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=6)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.3, Angle=367, KBG=100, FKB=40, BKB=0, Size=5.0, X=0.0, Y=10.0, Z=7.6, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=0.2, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=2, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=1.3, Angle=95, KBG=100, FKB=70, BKB=0, Size=5.0, X=0.0, Y=10.0, Z=7.6, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=0.2, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=2, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
			ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=1.3, Angle=367, KBG=100, FKB=40, BKB=0, Size=5.0, X=0.0, Y=13.0, Z=-5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=0.2, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=2, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
			ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=1.3, Angle=95, KBG=100, FKB=70, BKB=0, Size=5.0, X=0.0, Y=13.0, Z=-5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=0.2, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=2, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
		}
		frame(Frame=22)
		if(is_excute){
			ATTACK(ID=0, Part=1, Bone=hash40("top"), Damage=5.0, Angle=361, KBG=140, FKB=0, BKB=40, Size=5.0, X=0.0, Y=13.5, Z=-7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
			ATTACK(ID=1, Part=1, Bone=hash40("top"), Damage=5.0, Angle=361, KBG=140, FKB=0, BKB=40, Size=5.0, X=0.0, Y=9.0, Z=8.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
			ATTACK(ID=2, Part=1, Bone=hash40("top"), Damage=5.0, Angle=361, KBG=140, FKB=0, BKB=40, Size=6.0, X=0.0, Y=7.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
			ATTACK(ID=3, Part=1, Bone=hash40("head"), Damage=5.0, Angle=361, KBG=140, FKB=0, BKB=40, Size=6.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
		}
		wait(Frames=2)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=38)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
	});
}

#[acmd_script(agent = "zelda", script = "game_attackdash", category = ACMD_GAME)]
unsafe fn zelda_game_attackdash(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	acmd!(lua_state, {
		frame(Frame=6)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=12.0, Angle=70, KBG=60, FKB=0, BKB=100, Size=2.8, X=0.0, Y=9.0, Z=12.6, X2=0.0, Y2=9.0, Z2=13.2, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=9.0, Angle=70, KBG=60, FKB=0, BKB=90, Size=5.2, X=0.0, Y=9.0, Z=13.6, X2=0.0, Y2=9.0, Z2=7.8, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
		}
		wait(Frames=3)
		if(is_excute){
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=6.0, Angle=70, KBG=65, FKB=0, BKB=80, Size=5.0, X=0.0, Y=9.0, Z=13.6, X2=0.0, Y2=9.0, Z2=7.8, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
			AttackModule::clear(ID=0, false)
		}
		wait(Frames=6)
		if(is_excute){
			AttackModule::clear_all()
		}
	});
}

#[acmd_script(agent = "zelda", script = "game_attackhi3", category = ACMD_GAME)]
unsafe fn zelda_game_attackhi3(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	acmd!(lua_state, {
		frame(Frame=1)
		FT_MOTION_RATE(FSM=0.5)
		frame(Frame=5)
		FT_MOTION_RATE(FSM=1)
		frame(Frame=7)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("armr"), Damage=8.0, Angle=80, KBG=105, FKB=0, BKB=50, Size=5.5, X=4.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
			ATTACK(ID=1, Part=0, Bone=hash40("armr"), Damage=8.0, Angle=80, KBG=105, FKB=0, BKB=50, Size=3.0, X=1.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
			ATTACK(ID=2, Part=0, Bone=hash40("shoulderr"), Damage=8.0, Angle=80, KBG=105, FKB=0, BKB=50, Size=3.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
		}
		wait(Frames=13)
		if(is_excute){
			AttackModule::clear_all()
		}
	});
}

#[acmd_script(agent = "zelda", script = "game_attackhi4", category = ACMD_GAME)]
unsafe fn zelda_game_attackhi4(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	acmd!(lua_state, {
		frame(Frame=4)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
		}
		frame(Frame=9)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=0.9, Angle=367, KBG=100, FKB=40, BKB=0, Size=4.6, X=0.0, Y=19.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=0.2, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=2, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=0.9, Angle=90, KBG=100, FKB=40, BKB=0, Size=4.6, X=0.0, Y=19.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=0.2, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=2, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
			ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=0.9, Angle=367, KBG=100, FKB=40, BKB=0, Size=4.0, X=0.0, Y=18.0, Z=-5.0, X2=0.0, Y2=18.0, Z2=5.0, Hitlag=0.5, SDI=0.2, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=2, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
			ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=0.9, Angle=100, KBG=100, FKB=40, BKB=0, Size=4.0, X=0.0, Y=18.0, Z=-5.0, X2=0.0, Y2=18.0, Z2=5.0, Hitlag=0.5, SDI=0.2, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=2, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
			ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=0.9, Angle=367, KBG=100, FKB=40, BKB=0, Size=4.0, X=0.0, Y=12.0, Z=-3.0, X2=0.0, Y2=12.0, Z2=3.0, Hitlag=0.5, SDI=0.2, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=2, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
			ATTACK(ID=5, Part=0, Bone=hash40("top"), Damage=0.9, Angle=90, KBG=100, FKB=70, BKB=0, Size=4.0, X=0.0, Y=12.0, Z=-3.0, X2=0.0, Y2=12.0, Z2=3.0, Hitlag=0.5, SDI=0.2, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=2, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
		}
		frame(Frame=34)
		if(is_excute){
			AttackModule::clear_all()
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=5.0, Angle=90, KBG=200, FKB=0, BKB=40, Size=7.0, X=0.0, Y=21.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=5.0, Angle=90, KBG=200, FKB=0, BKB=40, Size=6.5, X=0.0, Y=18.0, Z=-6.0, X2=0.0, Y2=18.0, Z2=6.0, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
			ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=5.0, Angle=90, KBG=200, FKB=0, BKB=40, Size=8.0, X=0.0, Y=15.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
		}
		wait(Frames=1)
		if(is_excute){
			AttackModule::clear_all()
		}
	});
}

#[acmd_script(agent = "zelda", script = "game_attacklw3", category = ACMD_GAME)]
unsafe fn zelda_game_attacklw3(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	acmd!(lua_state, {
		frame(Frame=1)
		FT_MOTION_RATE(FSM=0.5)
		frame(Frame=5)
		FT_MOTION_RATE(FSM=1)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=6.0, Angle=80, KBG=120, FKB=0, BKB=20, Size=2.6, X=0.0, Y=2.0, Z=2.5, X2=0.0, Y2=1.5, Z2=7.5, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_KICK)
			AttackModule::set_attack_height_all(AttackHeight(*ATTACK_HEIGHT_LOW), false)
		}
		wait(Frames=5)
		FT_MOTION_RATE(FSM=0.55)
		wait(Frames=2)
		if(is_excute){
			AttackModule::clear_all()
		}
	});
}

#[acmd_script(agent = "zelda", script = "game_attacklw4", category = ACMD_GAME)]
unsafe fn zelda_game_attacklw4(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	acmd!(lua_state, {
		frame(Frame=3)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
		}
		frame(Frame=5)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=13.0, Angle=20, KBG=96, FKB=0, BKB=20, Size=4.2, X=0.0, Y=3.0, Z=12.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=13.0, Angle=20, KBG=96, FKB=0, BKB=20, Size=3.0, X=0.0, Y=3.0, Z=11.0, X2=0.0, Y2=5.0, Z2=7.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_KICK)
			AttackModule::set_attack_height_all(AttackHeight(*ATTACK_HEIGHT_LOW), false)
		}
		wait(Frames=2)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=13)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=13.0, Angle=20, KBG=96, FKB=0, BKB=20, Size=4.2, X=0.0, Y=3.0, Z=-11.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=13.0, Angle=20, KBG=96, FKB=0, BKB=20, Size=3.0, X=0.0, Y=3.0, Z=-9.0, X2=0.0, Y2=7.0, Z2=-4.5, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_KICK)
			AttackModule::set_attack_height_all(AttackHeight(*ATTACK_HEIGHT_LOW), false)
		}
		wait(Frames=2)
		if(is_excute){
			AttackModule::clear_all()
		}
	});
}

#[acmd_script(agent = "zelda", script = "game_attacks3", category = ACMD_GAME)]
unsafe fn zelda_game_attacks3(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	acmd!(lua_state, {
		frame(Frame=1)
		FT_MOTION_RATE(FSM=0.5)
		frame(Frame=5)
		FT_MOTION_RATE(FSM=1)
		frame(Frame=11)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("arml"), Damage=12.0, Angle=361, KBG=72, FKB=0, BKB=70, Size=2.1, X=-2.0, Y=0.2, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_MAGIC)
			ATTACK(ID=1, Part=0, Bone=hash40("arml"), Damage=12.0, Angle=361, KBG=72, FKB=0, BKB=70, Size=2.2, X=0.7, Y=0.3, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_MAGIC)
			ATTACK(ID=2, Part=0, Bone=hash40("arml"), Damage=12.0, Angle=361, KBG=72, FKB=0, BKB=70, Size=2.3, X=3.2, Y=0.4, Z=-0.1, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_MAGIC)
			ATTACK(ID=3, Part=0, Bone=hash40("handl"), Damage=15.0, Angle=361, KBG=70, FKB=0, BKB=80, Size=2.4, X=3.5, Y=0.0, Z=0.7, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_MAGIC)
			ATTACK(ID=4, Part=0, Bone=hash40("handl"), Damage=15.0, Angle=361, KBG=70, FKB=0, BKB=80, Size=2.5, X=6.2, Y=0.0, Z=1.4, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_MAGIC)
		}
		wait(Frames=3)
		if(is_excute){
			AttackModule::clear_all()
		}
		wait(Frames=1)
		FT_MOTION_RATE(FSM=0.85)
	});
}

#[acmd_script(agent = "zelda", script = "game_attacks3hi", category = ACMD_GAME)]
unsafe fn zelda_game_attacks3hi(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	acmd!(lua_state, {
		frame(Frame=1)
		FT_MOTION_RATE(FSM=0.5)
		frame(Frame=5)
		FT_MOTION_RATE(FSM=1)
		frame(Frame=11)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("arml"), Damage=12.0, Angle=361, KBG=72, FKB=0, BKB=70, Size=2.1, X=-2.0, Y=0.2, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_MAGIC)
			ATTACK(ID=1, Part=0, Bone=hash40("arml"), Damage=12.0, Angle=361, KBG=72, FKB=0, BKB=70, Size=2.2, X=0.7, Y=0.3, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_MAGIC)
			ATTACK(ID=2, Part=0, Bone=hash40("arml"), Damage=12.0, Angle=361, KBG=72, FKB=0, BKB=70, Size=2.3, X=3.2, Y=0.4, Z=-0.1, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_MAGIC)
			ATTACK(ID=3, Part=0, Bone=hash40("handl"), Damage=15.0, Angle=361, KBG=70, FKB=0, BKB=80, Size=2.4, X=3.5, Y=0.0, Z=0.7, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_MAGIC)
			ATTACK(ID=4, Part=0, Bone=hash40("handl"), Damage=15.0, Angle=361, KBG=70, FKB=0, BKB=80, Size=2.5, X=6.2, Y=0.0, Z=1.4, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_MAGIC)
			AttackModule::set_attack_height_all(AttackHeight(*ATTACK_HEIGHT_HIGH), false)
		}
		wait(Frames=3)
		if(is_excute){
			AttackModule::clear_all()
		}
		wait(Frames=1)
		FT_MOTION_RATE(FSM=0.85)
	});
}

#[acmd_script(agent = "zelda", script = "game_attacks3lw", category = ACMD_GAME)]
unsafe fn zelda_game_attacks3lw(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	acmd!(lua_state, {
		frame(Frame=1)
		FT_MOTION_RATE(FSM=0.5)
		frame(Frame=5)
		FT_MOTION_RATE(FSM=1)
		frame(Frame=11)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("arml"), Damage=12.0, Angle=361, KBG=72, FKB=0, BKB=70, Size=2.1, X=-2.0, Y=0.2, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_MAGIC)
			ATTACK(ID=1, Part=0, Bone=hash40("arml"), Damage=12.0, Angle=361, KBG=72, FKB=0, BKB=70, Size=2.2, X=0.7, Y=0.3, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_MAGIC)
			ATTACK(ID=2, Part=0, Bone=hash40("arml"), Damage=12.0, Angle=361, KBG=72, FKB=0, BKB=70, Size=2.3, X=3.2, Y=0.4, Z=-0.1, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_MAGIC)
			ATTACK(ID=3, Part=0, Bone=hash40("handl"), Damage=15.0, Angle=361, KBG=70, FKB=0, BKB=80, Size=2.4, X=3.5, Y=0.0, Z=0.7, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_MAGIC)
			ATTACK(ID=4, Part=0, Bone=hash40("handl"), Damage=15.0, Angle=361, KBG=70, FKB=0, BKB=80, Size=2.5, X=6.2, Y=0.0, Z=1.4, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_MAGIC)
			AttackModule::set_attack_height_all(AttackHeight(*ATTACK_HEIGHT_LOW), false)
		}
		wait(Frames=3)
		if(is_excute){
			AttackModule::clear_all()
		}
		wait(Frames=1)
		FT_MOTION_RATE(FSM=0.85)
	});
}

#[acmd_script(agent = "zelda", script = "game_attacks4", category = ACMD_GAME)]
unsafe fn zelda_game_attacks4(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	acmd!(lua_state, {
		frame(Frame=11)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
		}
		frame(Frame=16)
		for(4 Iterations){
			if(is_excute){
				ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.5, Angle=367, KBG=100, FKB=40, BKB=0, Size=4.5, X=0.0, Y=9.1, Z=17.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=0.2, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
				ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=2.5, Angle=100, KBG=100, FKB=40, BKB=0, Size=4.5, X=0.0, Y=9.1, Z=17.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=0.2, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
				ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=2.5, Angle=367, KBG=100, FKB=40, BKB=0, Size=4.5, X=0.0, Y=9.1, Z=9.0, X2=0.0, Y2=9.1, Z2=17.0, Hitlag=0.8, SDI=0.2, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
				ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=2.5, Angle=100, KBG=100, FKB=40, BKB=0, Size=4.5, X=0.0, Y=9.1, Z=9.0, X2=0.0, Y2=9.1, Z2=17.0, Hitlag=0.8, SDI=0.2, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
			}
			wait(Frames=1)
			if(is_excute){
				AttackModule::clear_all()
			}
			wait(Frames=1)
		}
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=10.0, Angle=361, KBG=145, FKB=0, BKB=40, Size=6.0, X=0.0, Y=9.1, Z=10.5, X2=0.0, Y2=9.1, Z2=17.0, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
		}
		wait(Frames=1)
		if(is_excute){
			AttackModule::clear_all()
		}
	});
}

#[acmd_script(agent = "zelda", script = "game_catch", category = ACMD_GAME)]
unsafe fn zelda_game_catch(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter,	0.5);
	frame(fighter.lua_state_agent, 9.0);
	macros::FT_MOTION_RATE(fighter,	1.0);
	if macros::is_excute(fighter) {
		GrabModule::set_rebound(fighter.module_accessor, true);
	}
	frame(fighter.lua_state_agent, 10.0);
	if macros::is_excute(fighter) {
		macros::CATCH(fighter, 0, Hash40::new("top"), 3.8, 0.0, 9.0, 4.0, Some(0.0), Some(9.0), Some(11.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
		macros::CATCH(fighter, 1, Hash40::new("top"), 1.9, 0.0, 9.0, 2.1, Some(0.0), Some(9.0), Some(13.4), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
	}
	macros::game_CaptureCutCommon(fighter);
	wait(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(fighter.module_accessor, false);
	}
}

#[acmd_script(agent = "zelda", script = "game_catchattack", category = ACMD_GAME)]
unsafe fn zelda_game_catchattack(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 361, 100, 30, 0, 5.0, 0.0, 11.0, 8.0, None, None, None, 1.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
		AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
	}
	wait(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	macros::FT_MOTION_RATE(fighter,	0.5);
	wait(fighter.lua_state_agent, 2.0);
	macros::FT_MOTION_RATE(fighter,	1.0);
}

#[acmd_script(agent = "zelda", script = "game_catchdash", category = ACMD_GAME)]
unsafe fn zelda_game_catchdash(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter,	0.5);
	frame(fighter.lua_state_agent, 9.0);
	macros::FT_MOTION_RATE(fighter,	1.0);
	frame(fighter.lua_state_agent, 12.0);
	if macros::is_excute(fighter) {
		GrabModule::set_rebound(fighter.module_accessor, true);
	}
	frame(fighter.lua_state_agent, 13.0);
	if macros::is_excute(fighter) {
		macros::CATCH(fighter, 0, Hash40::new("top"), 3.0, 0.0, 9.0, 4.0, Some(0.0), Some(9.0), Some(13.3), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
		macros::CATCH(fighter, 1, Hash40::new("top"), 1.5, 0.0, 9.0, 2.5, Some(0.0), Some(9.0), Some(14.8), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
	}
	macros::game_CaptureCutCommon(fighter);
	wait(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(fighter.module_accessor, false);
	}
}

#[acmd_script(agent = "zelda", script = "game_catchturn", category = ACMD_GAME)]
unsafe fn zelda_game_catchturn(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter,	0.5);
	frame(fighter.lua_state_agent, 9.0);
	macros::FT_MOTION_RATE(fighter,	1.0);
	frame(fighter.lua_state_agent, 13.0);
	if macros::is_excute(fighter) {
		GrabModule::set_rebound(fighter.module_accessor, true);
	}
	frame(fighter.lua_state_agent, 14.0);
	if macros::is_excute(fighter) {
		macros::CATCH(fighter, 0, Hash40::new("top"), 3.8, 0.0, 9.0, -4.0, Some(0.0), Some(9.0), Some(-19.2), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
		macros::CATCH(fighter, 1, Hash40::new("top"), 1.9, 0.0, 9.0, -2.1, Some(0.0), Some(9.0), Some(-21.1), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
	}
	macros::game_CaptureCutCommon(fighter);
	wait(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(fighter.module_accessor, false);
	}
}

#[acmd_script(agent = "zelda", script = "game_specialairhi", category = ACMD_GAME)]
unsafe fn zelda_game_specialairhi(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	acmd!(lua_state, {
		frame(Frame=1)
		if(is_excute){
			JostleModule::set_status(true)
			sv_battle_object::notify_event_msc_cmd(0x2127e37c07, GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES)
			ATTACK(ID=0, Part=0, Bone=hash40("rot"), Damage=12.0, Angle=55, KBG=95, FKB=0, BKB=80, Size=16.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_MAGIC)
		}
		wait(Frames=3)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=12)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_ZELDA_STATUS_SPECIAL_HI_FLAG_1)
		}
		frame(Frame=15)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_ZELDA_STATUS_SPECIAL_HI_FLAG_DIVE)
			WorkModule::on_flag(Flag=FIGHTER_ZELDA_STATUS_SPECIAL_HI_FLAG_CONTROL)
		}
	});
}

#[acmd_script(agent = "zelda", script = "game_specialairhistart", category = ACMD_GAME)]
unsafe fn zelda_game_specialairhistart(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	acmd!(lua_state, {
		frame(Frame=1)
		if(is_excute){
			sv_battle_object::notify_event_msc_cmd(0x2127e37c07, GROUND_CLIFF_CHECK_KIND_NONE)
		}
		frame(Frame=3)
		if(is_excute){
			sv_module_access::damage(MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 5)
		}
		frame(Frame=6)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("rot"), Damage=6.0, Angle=80, KBG=105, FKB=0, BKB=60, Size=10.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_MAGIC)
		}
		wait(Frames=2)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=17)
		if(is_excute){
			sv_module_access::damage(MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_NORMAL, 0)
			sv_battle_object::notify_event_msc_cmd(0x2127e37c07, GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES)
		}
	});
}

#[acmd_script(agent = "zelda", script = "game_specialhi", category = ACMD_GAME)]
unsafe fn zelda_game_specialhi(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	acmd!(lua_state, {
		frame(Frame=1)
		if(is_excute){
			JostleModule::set_status(true)
			ATTACK(ID=0, Part=0, Bone=hash40("rot"), Damage=12.0, Angle=361, KBG=85, FKB=0, BKB=80, Size=15.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_MAGIC)
		}
		wait(Frames=3)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=10)
		FT_MOTION_RATE(FSM=1.25)
	});
}

#[acmd_script(agent = "zelda", script = "game_specialhistart", category = ACMD_GAME)]
unsafe fn zelda_game_specialhistart(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	acmd!(lua_state, {
		frame(Frame=3)
		if(is_excute){
			sv_module_access::damage(MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 5)
		}
		frame(Frame=6)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=6.0, Angle=90, KBG=100, FKB=180, BKB=5, Size=8.0, X=0.0, Y=6.0, Z=-4.0, X2=0.0, Y2=6.0, Z2=4.0, Hitlag=1.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_MAGIC)
		}
		wait(Frames=2)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=17)
		if(is_excute){
			sv_module_access::damage(MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_NORMAL, 0)
		}
	});
}

#[acmd_script(agent = "zelda", scripts = ["game_specialn", "game_specialairn"], category = ACMD_GAME)]
unsafe fn zelda_game_specialn(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	acmd!(lua_state, {
		frame(Frame=4)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_ZELDA_STATUS_SPECIAL_N_FLAG_REFLECTOR_START)
			sv_module_access::damage(MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 5)
		}
		FT_MOTION_RATE(FSM=0.625)
		frame(Frame=12)
		FT_MOTION_RATE(FSM=1)
		frame(Frame=13)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.3, Angle=367, KBG=100, FKB=40, BKB=0, Size=8.5, X=0.0, Y=7.0, Z=-0.5, X2=0.0, Y2=7.0, Z2=0.5, Hitlag=0.0, SDI=0.2, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=2, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_MAGIC)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=1.3, Angle=160, KBG=100, FKB=40, BKB=0, Size=8.5, X=0.0, Y=7.0, Z=-0.5, X2=0.0, Y2=7.0, Z2=0.5, Hitlag=0.0, SDI=0.2, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=2, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_MAGIC)
			ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=1.3, Angle=367, KBG=100, FKB=40, BKB=0, Size=4.0, X=0.0, Y=8.0, Z=-10.0, X2=0.0, Y2=8.0, Z2=10.0, Hitlag=0.0, SDI=0.2, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=2, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_MAGIC)
			ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=1.3, Angle=160, KBG=100, FKB=40, BKB=0, Size=4.0, X=0.0, Y=8.0, Z=-10.0, X2=0.0, Y2=8.0, Z2=10.0, Hitlag=0.0, SDI=0.2, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=2, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_MAGIC)
		}
		frame(Frame=28)
		if(is_excute){
			AttackModule::clear_all()
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=5.0, Angle=361, KBG=110, FKB=0, BKB=60, Size=7.0, X=0.0, Y=8.0, Z=-4.0, X2=0.0, Y2=8.0, Z2=4.0, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_MAGIC)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=5.0, Angle=361, KBG=110, FKB=0, BKB=60, Size=5.0, X=0.0, Y=8.0, Z=-11.0, X2=0.0, Y2=8.0, Z2=11.0, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_MAGIC)
		}
		wait(Frames=1)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=43)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_ZELDA_STATUS_SPECIAL_N_FLAG_REFLECTOR_END)
			sv_module_access::damage(MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_NORMAL, 0)
		}
	});
}

#[acmd_script(agent = "zelda", script = "game_throwb", category = ACMD_GAME)]
unsafe fn zelda_game_throwb(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 12.0, 45, 75, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
	}
	frame(fighter.lua_state_agent, 26.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_REVERSE_LR_FINISH_CAMERA_THROW_ORBIT);
		macros::CHECK_FINISH_CAMERA(fighter, 11, 7);
		let fighter_cutin_manager = *(FIGHTER_CUTIN_MANAGER_ADDR as *mut *mut smash::app::FighterCutInManager);
		FighterCutInManager::set_throw_finish_zoom_rate(fighter_cutin_manager, 1.3);
		FighterCutInManager::set_throw_finish_offset(fighter_cutin_manager, Vector3f{x: -5.0, y: 0.0, z: 0.0});
	}
	frame(fighter.lua_state_agent, 27.0);
	if macros::is_excute(fighter) {
		macros::REVERSE_LR(fighter);
		macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
	}
}

#[acmd_script(agent = "zelda", script = "game_throwf", category = ACMD_GAME)]
unsafe fn zelda_game_throwf(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 10.0, 40, 40, 0, 100, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
	}
	frame(fighter.lua_state_agent, 29.0);
	if macros::is_excute(fighter) {
		macros::CHECK_FINISH_CAMERA(fighter, 17, 4);
		let fighter_cutin_manager = *(FIGHTER_CUTIN_MANAGER_ADDR as *mut *mut smash::app::FighterCutInManager);
		FighterCutInManager::set_throw_finish_zoom_rate(fighter_cutin_manager, 1.25);
		FighterCutInManager::set_throw_finish_offset(fighter_cutin_manager, Vector3f{x: 9.0, y: 1.0, z: 0.0});
	}
	frame(fighter.lua_state_agent, 30.0);
	if macros::is_excute(fighter) {
		macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
	}
}

#[acmd_script(agent = "zelda", script = "game_throwhi", category = ACMD_GAME)]
unsafe fn zelda_game_throwhi(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 11.0, 90, 52, 0, 100, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
	}
	frame(fighter.lua_state_agent, 29.0);
	if macros::is_excute(fighter) {
		macros::CHECK_FINISH_CAMERA(fighter, 1, 25);
		let fighter_cutin_manager = *(FIGHTER_CUTIN_MANAGER_ADDR as *mut *mut smash::app::FighterCutInManager);
		FighterCutInManager::set_throw_finish_zoom_rate(fighter_cutin_manager, 1.5);
		FighterCutInManager::set_throw_finish_offset(fighter_cutin_manager, Vector3f{x: 0.0, y: 10.0, z: 0.0});
	}
	frame(fighter.lua_state_agent, 30.0);
	if macros::is_excute(fighter) {
		macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
	}
	macros::FT_MOTION_RATE(fighter,	0.8);
}

#[acmd_script(agent = "zelda", script = "game_throwlw", category = ACMD_GAME)]
unsafe fn zelda_game_throwlw(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::FT_LEAVE_NEAR_OTTOTTO(fighter, 1.5, 1.5);
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 2.0, 80, 115, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_THROW);
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
	}
	frame(fighter.lua_state_agent, 25.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 40, 100, 30, 0, 7.2, 0.0, 2.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
		AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
	}
	frame(fighter.lua_state_agent, 27.0);
	macros::ATK_POWER(fighter, 0, 0.5);
	frame(fighter.lua_state_agent, 50.0);
	if macros::is_excute(fighter) {
		macros::CHECK_FINISH_CAMERA(fighter, 2, 0);
		let fighter_cutin_manager = *(FIGHTER_CUTIN_MANAGER_ADDR as *mut *mut smash::app::FighterCutInManager);
		FighterCutInManager::set_throw_finish_zoom_rate(fighter_cutin_manager, 1.5);
		FighterCutInManager::set_throw_finish_offset(fighter_cutin_manager, Vector3f{x: 0.0, y: 5.0, z: 0.0});
	}
	frame(fighter.lua_state_agent, 51.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
		macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
	}
	frame(fighter.lua_state_agent, 52.0);
	macros::FT_MOTION_RATE(fighter,	0.625);
}

#[acmd_script(agent = "zelda_dein_s", script = "game_move", category = ACMD_GAME)]
unsafe fn zelda_dein_s_game_move(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	acmd!(lua_state, {
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.0, Angle=80, KBG=102, FKB=0, BKB=50, Size=3.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=true, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_BOMB)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=1.0, Angle=80, KBG=102, FKB=0, BKB=50, Size=5.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=true, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_BOMB)
			AREA_WIND_2ND_RAD_arg9(0, 2, 0.05, 200, 1, 0, 0, 12, 60)
		}
		frame(Frame=6)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=20)
		if(is_excute){
			AreaModule::erase_wind(0)
		}
	});
}

#[acmd_script(agent = "zelda_phantom", script = "game_attackkick", category = ACMD_GAME)]
unsafe fn zelda_phantom_game_attackkick(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	acmd!(lua_state, {
		frame(Frame=10)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.0, Angle=361, KBG=45, FKB=0, BKB=50, Size=5.5, X=0.0, Y=6.0, Z=14.0, X2=0.0, Y2=6.0, Z2=10.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
		}
		wait(Frames=2)
		if(is_excute){
			AttackModule::clear_all()
		}
	});
}

#[acmd_script(agent = "zelda_phantom", script = "game_attackl", category = ACMD_GAME)]
unsafe fn zelda_phantom_game_attackl(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	acmd!(lua_state, {
		frame(Frame=0)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=0.0, Angle=361, KBG=100, FKB=110, BKB=0, Size=5.5, X=0.0, Y=8.0, Z=10.0, X2=0.0, Y2=8.0, Z2=4.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=true, Absorbable=false, Flinchless=true, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_NONE)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=0.0, Angle=361, KBG=100, FKB=80, BKB=0, Size=7.0, X=0.0, Y=8.0, Z=10.0, X2=0.0, Y2=8.0, Z2=4.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=true, Absorbable=false, Flinchless=true, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_NONE)
		}
		frame(Frame=7)
		if(is_excute){
			AttackModule::clear_all()
			ATTACK(ID=0, Part=0, Bone=hash40("handr"), Damage=5.0, Angle=361, KBG=65, FKB=0, BKB=60, Size=5.0, X=2.0, Y=0.0, Z=1.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-2.5, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
			ATTACK(ID=1, Part=0, Bone=hash40("handr"), Damage=5.0, Angle=361, KBG=65, FKB=0, BKB=60, Size=5.6, X=2.0, Y=0.0, Z=8.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-2.5, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
			ATTACK(ID=2, Part=0, Bone=hash40("handr"), Damage=5.0, Angle=361, KBG=65, FKB=0, BKB=60, Size=5.6, X=2.0, Y=0.0, Z=16.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-2.5, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
			ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=5.0, Angle=361, KBG=65, FKB=0, BKB=60, Size=5.0, X=0.0, Y=8.5, Z=11.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-2.5, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
			AttackModule::set_attack_height_all(AttackHeight(*ATTACK_HEIGHT_HIGH), false)
		}
		wait(Frames=4)
		if(is_excute){
			AttackModule::clear_all()
		}
	});
}

#[acmd_script(agent = "zelda_phantom", script = "game_attackmax", category = ACMD_GAME)]
unsafe fn zelda_phantom_game_attackmax(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	acmd!(lua_state, {
		frame(Frame=0)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=0.0, Angle=361, KBG=100, FKB=130, BKB=0, Size=6.0, X=0.0, Y=8.0, Z=13.0, X2=0.0, Y2=8.0, Z2=8.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=true, Absorbable=false, Flinchless=true, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_NONE)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=0.0, Angle=6, KBG=100, FKB=85, BKB=0, Size=8.5, X=0.0, Y=8.0, Z=13.0, X2=0.0, Y2=8.0, Z2=8.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=true, Absorbable=false, Flinchless=true, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_NONE)
		}
		frame(Frame=6)
		if(is_excute){
			AttackModule::clear_all()
			ATTACK(ID=0, Part=0, Bone=hash40("handr"), Damage=6.0, Angle=45, KBG=70, FKB=0, BKB=60, Size=6.0, X=2.0, Y=0.0, Z=1.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-3, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
			ATTACK(ID=1, Part=0, Bone=hash40("handr"), Damage=6.0, Angle=45, KBG=70, FKB=0, BKB=60, Size=6.0, X=2.0, Y=1.0, Z=9.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-3, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
			ATTACK(ID=2, Part=0, Bone=hash40("handr"), Damage=6.0, Angle=45, KBG=70, FKB=0, BKB=60, Size=6.0, X=2.0, Y=2.0, Z=16.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-3, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
		}
		wait(Frames=11)
		if(is_excute){
			AttackModule::clear_all()
		}
	});
}

#[acmd_script(agent = "zelda_phantom", script = "game_attackpunch", category = ACMD_GAME)]
unsafe fn zelda_phantom_game_attackpunch(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	acmd!(lua_state, {
		frame(Frame=0)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=0.0, Angle=361, KBG=100, FKB=60, BKB=0, Size=4.0, X=0.0, Y=7.0, Z=11.0, X2=0.0, Y2=7.0, Z2=7.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=true, Absorbable=false, Flinchless=true, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_NONE)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=0.0, Angle=361, KBG=100, FKB=40, BKB=0, Size=6.0, X=0.0, Y=7.0, Z=11.0, X2=0.0, Y2=7.0, Z2=7.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=true, Absorbable=false, Flinchless=true, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_NONE)
		}
		frame(Frame=7)
		if(is_excute){
			AttackModule::clear_all()
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=3.0, Angle=361, KBG=55, FKB=0, BKB=60, Size=5.5, X=0.0, Y=10.0, Z=11.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-1.5, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_OBJECT)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=3.0, Angle=361, KBG=55, FKB=0, BKB=60, Size=4.5, X=0.0, Y=9.5, Z=19.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-1.5, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_OBJECT)
		}
		wait(Frames=5)
		if(is_excute){
			AttackModule::clear_all()
		}
	});
}

#[acmd_script(agent = "zelda_phantom", script = "game_attacks", category = ACMD_GAME)]
unsafe fn zelda_phantom_game_attacks(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	acmd!(lua_state, {
		frame(Frame=0)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=0.0, Angle=361, KBG=100, FKB=50, BKB=0, Size=5.0, X=0.0, Y=8.0, Z=10.0, X2=0.0, Y2=8.0, Z2=4.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=4, Reflectable=true, Absorbable=false, Flinchless=true, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_NONE)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=0.0, Angle=361, KBG=100, FKB=40, BKB=0, Size=7.0, X=0.0, Y=8.0, Z=10.0, X2=0.0, Y2=8.0, Z2=4.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=4, Reflectable=true, Absorbable=false, Flinchless=true, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_NONE)
		}
		frame(Frame=7)
		if(is_excute){
			AttackModule::clear_all()
			ATTACK(ID=0, Part=0, Bone=hash40("handr"), Damage=4.0, Angle=361, KBG=65, FKB=0, BKB=60, Size=5.8, X=0.0, Y=0.0, Z=1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-2, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
			ATTACK(ID=1, Part=0, Bone=hash40("handr"), Damage=4.0, Angle=361, KBG=65, FKB=0, BKB=60, Size=5.8, X=0.0, Y=0.0, Z=8.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-2, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
			ATTACK(ID=2, Part=0, Bone=hash40("handr"), Damage=4.0, Angle=361, KBG=65, FKB=0, BKB=60, Size=5.8, X=0.0, Y=0.0, Z=16.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-2, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
			ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=4.0, Angle=361, KBG=65, FKB=0, BKB=60, Size=6.5, X=0.0, Y=8.0, Z=10.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-2, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
		}
		wait(Frames=4)
		if(is_excute){
			AttackModule::clear_all()
		}
	});
}

pub fn install() {
	smashline::install_acmd_scripts!(
		zelda_game_attack11,
		zelda_game_attack100,
		zelda_game_attack100end,
		zelda_game_attack100sub,
		zelda_game_attackairb,
		zelda_game_attackairf,
		zelda_game_attackairhi,
		zelda_game_attackairlw,
		zelda_game_attackairn,
		zelda_game_attackdash,
		zelda_game_attackhi3,
		zelda_game_attackhi4,
		zelda_game_attacklw3,
		zelda_game_attacklw4,
		zelda_game_attacks3,
		zelda_game_attacks3hi,
		zelda_game_attacks3lw,
		zelda_game_attacks4,
		zelda_game_catch,
		zelda_game_catchattack,
		zelda_game_catchdash,
		zelda_game_catchturn,
		zelda_game_specialairhi,
		zelda_game_specialairhistart,
		zelda_game_specialhi,
		zelda_game_specialhistart,
		zelda_game_specialn,
		zelda_game_throwb,
		zelda_game_throwf,
		zelda_game_throwhi,
		zelda_game_throwlw,
		zelda_dein_s_game_move,
		zelda_phantom_game_attackkick,
		zelda_phantom_game_attackl,
		zelda_phantom_game_attackmax,
		zelda_phantom_game_attackpunch,
		zelda_phantom_game_attacks,
	);
}
