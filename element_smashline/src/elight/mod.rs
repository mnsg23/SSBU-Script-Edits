use {
	crate::FIGHTER_CUTIN_MANAGER_ADDR,
	smash::{
		app::{
			lua_bind::*,
			sv_animcmd::*
		},
		hash40,
		lib::lua_const::*,
		lua2cpp::L2CAgentBase,
		phx::*
	},
	smash_script::*,
	smashline::*
};

#[acmd_script(agent = "elight", script = "game_attack11", category = ACMD_GAME)]
unsafe fn elight_game_attack11(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 0.5);
	frame(fighter.lua_state_agent, 3.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 361, 15, 0, 25, 2.5, 0.0, 8.0, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 361, 15, 0, 25, 2.5, 0.0, 8.0, 8.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.0, 180, 5, 0, 25, 2.5, 0.0, 8.0, 11.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 2.0, 361, 5, 0, 25, 2.5, 0.0, 8.0, 11.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 5.0, false);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 5.0, false);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, 2, 5.0, false);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, 3, 5.0, false);
	}
	frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
	}
	frame(fighter.lua_state_agent, 7.0);
	macros::FT_MOTION_RATE(fighter, 0.5);
	frame(fighter.lua_state_agent, 11.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_RESTART);
	}
}

#[acmd_script(agent = "elight", script = "game_attack12", category = ACMD_GAME)]
unsafe fn elight_game_attack12(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD) == true {
			ArticleModule::add_motion_partial(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 5.0, 5.0, false, false, 0.0, false, true, false);
		}
		if MotionModule::is_changing(fighter.module_accessor) == true {
			WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
		}
	}
	macros::FT_MOTION_RATE(fighter, 0.5);
	frame(fighter.lua_state_agent, 5.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 361, 20, 0, 25, 3.0, 0.0, 8.0, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 361, 20, 0, 25, 3.0, 0.0, 8.0, 9.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.0, 180, 10, 0, 25, 4.0, 0.0, 8.0, 11.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 2.0, 361, 10, 0, 25, 4.0, 0.0, 8.0, 11.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 5.0, false);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 5.0, false);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, 2, 5.0, false);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, 3, 5.0, false);
	}
	frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 2, Hash40::new("top"), 0.0, 8.0, 13.5);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 3, Hash40::new("top"), 0.0, 8.0, 13.5);
	}
	frame(fighter.lua_state_agent, 7.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	frame(fighter.lua_state_agent, 10.0);
	if macros::is_excute(fighter) {
		if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD) == true {
			ArticleModule::add_motion_partial(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_close"), 5.0, 5.0, false, false, 0.0, false, true, false);
		}
		if MotionModule::is_changing(fighter.module_accessor) == true {
			WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
		}
	}
	frame(fighter.lua_state_agent, 11.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
	}
	frame(fighter.lua_state_agent, 14.0);
	if macros::is_excute(fighter) {
		WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100);
	}
}

#[acmd_script(agent = "elight", script = "game_attack13", category = ACMD_GAME)]
unsafe fn elight_game_attack13(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD) == true {
			ArticleModule::add_motion_partial(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 5.0, 5.0, false, false, 0.0, false, true, false);
		}
		if MotionModule::is_changing(fighter.module_accessor) == true {
			WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
		}
	}
	macros::FT_MOTION_RATE(fighter, 0.5);
	frame(fighter.lua_state_agent, 3.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	frame(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 4.0, 60, 95, 0, 80, 3.5, 0.0, 4.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 1, 0, Hash40::new("haver"), 4.0, 60, 95, 0, 80, 3.0, -2.0, 8.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 2, 0, Hash40::new("haver"), 4.0, 60, 95, 0, 80, 2.5, -1.5, 13.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 4.0, 60, 95, 0, 80, 3.0, 0.0, 8.0, 5.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
	}
	frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 1, Hash40::new("haver"), 0.0, 8.0, 0.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 2, Hash40::new("haver"), 0.0, 13.0, 0.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 3, Hash40::new("top"), 0.0, 15.0, 5.0);
	}
	frame(fighter.lua_state_agent, 7.0);
	if macros::is_excute(fighter) {
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 1, Hash40::new("haver"), 2.0, 8.0, 0.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 2, Hash40::new("haver"), 1.5, 13.0, 0.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 3, Hash40::new("top"), 0.0, 16.0, 3.0);
	}
	frame(fighter.lua_state_agent, 8.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	frame(fighter.lua_state_agent, 11.0);
	if macros::is_excute(fighter) {
		if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD) == true {
			ArticleModule::add_motion_partial(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_close"), 5.0, 5.0, false, false, 0.0, false, true, false);
		}
		if MotionModule::is_changing(fighter.module_accessor) == true {
			WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
		}
	}
	macros::FT_MOTION_RATE(fighter, 0.75);
	frame(fighter.lua_state_agent, 31.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
}

#[acmd_script(agent = "elight", script = "game_attack100", category = ACMD_GAME)]
unsafe fn elight_game_attack100(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 2.0);
	for _ in 0..1000000 {
		for _ in 0..4 {
			if macros::is_excute(fighter) {
				macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.4, 361, 10, 0, 15, 6.0, 0.0, 7.0, 10.0, Some(0.0), Some(7.0), Some(15.0), 0.5, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
				AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 4.0, false);
				macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 8);
			}
			wait(fighter.lua_state_agent, 1.0);
			if macros::is_excute(fighter) {
				AttackModule::clear_all(fighter.module_accessor);
				WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
			}
			wait(fighter.lua_state_agent, 2.0);
		}
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.4, 361, 10, 0, 15, 6.0, 0.0, 7.0, 10.0, Some(0.0), Some(7.0), Some(15.0), 0.5, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
			AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 4.0, false);
			macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 8);
		}
		wait(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
			WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
		}
		wait(fighter.lua_state_agent, 3.0);
	}
}

#[acmd_script(agent = "elight", script = "game_attack100end", category = ACMD_GAME)]
unsafe fn elight_game_attack100end(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 0.25);
	frame(fighter.lua_state_agent, 5.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 40, 80, 0, 80, 8.0, 0.0, 10.0, 12.0, Some(0.0), Some(10.0), Some(18.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
	}
	frame(fighter.lua_state_agent, 8.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

#[acmd_script(agent = "elight", script = "game_attack100start", category = ACMD_GAME)]
unsafe fn elight_game_attack100start(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 0.25);
	frame(fighter.lua_state_agent, 9.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
}

#[acmd_script(agent = "elight", script = "game_attack100sub", category = ACMD_GAME)]
unsafe fn elight_game_attack100sub(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.4, 361, 10, 0, 15, 6.0, 0.0, 7.0, 10.0, Some(0.0), Some(7.0), Some(15.0), 0.5, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 4.0, false);
		macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 8);
	}
	wait(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
	}
}

#[acmd_script(agent = "elight", script = "game_attackairb", category = ACMD_GAME)]
unsafe fn elight_game_attackairb(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 2.0);
	macros::FT_MOTION_RATE(fighter, 0.25);
	frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	macros::FT_MOTION_RATE(fighter, 0.5);
	frame(fighter.lua_state_agent, 10.0);
	if macros::is_excute(fighter) {
		if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD) == true {
			ArticleModule::add_motion_partial(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 5.0, 5.0, false, false, 0.0, false, true, false);
		}
		if MotionModule::is_changing(fighter.module_accessor) == true {
			WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
		}
	}
	frame(fighter.lua_state_agent, 12.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	frame(fighter.lua_state_agent, 14.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 8.0, 75, 75, 0, 70, 3.0, 0.0, 6.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 1, 0, Hash40::new("haver"), 8.0, 75, 75, 0, 70, 2.5, 0.0, 9.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 2, 0, Hash40::new("haver"), 8.0, 75, 75, 0, 70, 2.5, 0.0, 12.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 3, 0, Hash40::new("haver"), 8.0, 75, 75, 0, 70, 3.0, 0.0, 0.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 4, 0, Hash40::new("haver"), 8.0, 75, 75, 0, 70, 7.0, 0.0, 7.0, -5.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
	}
	frame(fighter.lua_state_agent, 15.0);
	if macros::is_excute(fighter) {
		AttackModule::clear(fighter.module_accessor, 4, false);
	}
	frame(fighter.lua_state_agent, 17.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	frame(fighter.lua_state_agent, 18.0);
	if macros::is_excute(fighter) {
		if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD) == true {
			ArticleModule::add_motion_partial(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_close"), 5.0, 5.0, false, false, 0.0, false, true, false);
		}
		if MotionModule::is_changing(fighter.module_accessor) == true {
			WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
		}
	}
	macros::FT_MOTION_RATE(fighter, 0.7);
	frame(fighter.lua_state_agent, 34.0);
	if macros::is_excute(fighter) {
		WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	frame(fighter.lua_state_agent, 71.0);
	if macros::is_excute(fighter) {
		notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
	}
	frame(fighter.lua_state_agent, 78.0);
	macros::FT_MOTION_RATE(fighter,	1.0);
}

#[acmd_script(agent = "elight", script = "game_attackairf", category = ACMD_GAME)]
unsafe fn elight_game_attackairf(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 0.5);
	frame(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	frame(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD) == true {
			ArticleModule::add_motion_partial(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 5.0, 5.0, false, false, 0.0, false, true, false);
		}
		if MotionModule::is_changing(fighter.module_accessor) == true {
			WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
		}
	}
	macros::FT_MOTION_RATE(fighter, 1.0);
	frame(fighter.lua_state_agent, 8.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 7.0, 45, 90, 0, 50, 3.5, 2.0, 6.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 1, 0, Hash40::new("haver"), 7.0, 45, 90, 0, 50, 3.0, 2.0, 9.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 2, 0, Hash40::new("haver"), 7.0, 45, 90, 0, 50, 2.5, 2.0, 11.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 3, 0, Hash40::new("haver"), 7.0, 45, 90, 0, 50, 3.0, 2.0, 0.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 4, 0, Hash40::new("haver"), 7.0, 45, 90, 0, 50, 2.5, 2.0, -4.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 5, 0, Hash40::new("haver"), 7.0, 45, 90, 0, 50, 2.5, 2.0, 6.0, -4.0, Some(2.0), Some(11.0), Some(-4.0), 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
	}
	frame(fighter.lua_state_agent, 9.0);
	if macros::is_excute(fighter) {
		AttackModule::clear(fighter.module_accessor, 5, false);
	}
	frame(fighter.lua_state_agent, 10.0);
	if macros::is_excute(fighter) {
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 0, Hash40::new("haver"), -2.0, 6.0, 0.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 1, Hash40::new("haver"), -2.0, 9.0, 0.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 2, Hash40::new("haver"), -2.0, 11.0, 0.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 3, Hash40::new("haver"), -2.0, 0.0, 0.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 4, Hash40::new("haver"), -2.0, -4.0, 0.0);
		AttackModule::set_size(fighter.module_accessor, 4, 2.0);
	}
	frame(fighter.lua_state_agent, 11.0);
	if macros::is_excute(fighter) {
		AttackModule::clear(fighter.module_accessor, 4, false);
	}
	frame(fighter.lua_state_agent, 12.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	frame(fighter.lua_state_agent, 13.0);
	if macros::is_excute(fighter) {
		if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD) == true {
			ArticleModule::add_motion_partial(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_close"), 5.0, 5.0, false, false, 0.0, false, true, false);
		}
		if MotionModule::is_changing(fighter.module_accessor) == true {
			WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
		}
	}
	macros::FT_MOTION_RATE(fighter, 0.6);
	frame(fighter.lua_state_agent, 43.0);
	if macros::is_excute(fighter) {
		WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	frame(fighter.lua_state_agent, 63.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	frame(fighter.lua_state_agent, 69.0);
	if macros::is_excute(fighter) {
		notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
	}
}

#[acmd_script(agent = "elight", script = "game_attackairhi", category = ACMD_GAME)]
unsafe fn elight_game_attackairhi(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 0.2);
	frame(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	frame(fighter.lua_state_agent, 6.0);
	macros::FT_MOTION_RATE(fighter, 0.5);
	frame(fighter.lua_state_agent, 9.0);
	if macros::is_excute(fighter) {
		if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD) == true {
			ArticleModule::add_motion_partial(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 5.0, 5.0, false, false, 0.0, false, true, false);
		}
		if MotionModule::is_changing(fighter.module_accessor) == true {
			WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
		}
	}
	frame(fighter.lua_state_agent, 10.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	frame(fighter.lua_state_agent, 11.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 6.0, 80, 65, 0, 90, 4.0, 0.0, 5.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 1, 0, Hash40::new("haver"), 6.0, 80, 65, 0, 90, 3.5, 0.0, 8.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 2, 0, Hash40::new("haver"), 6.0, 80, 65, 0, 90, 2.5, 0.0, 11.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 3, 0, Hash40::new("haver"), 6.0, 80, 65, 0, 90, 3.0, 0.0, 0.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
	}
	frame(fighter.lua_state_agent, 12.0);
	if macros::is_excute(fighter) {
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 0, Hash40::new("haver"), -1.0, 5.0, 0.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 1, Hash40::new("haver"), -2.0, 8.0, 0.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 2, Hash40::new("haver"), -3.0, 11.0, 0.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 3, Hash40::new("haver"), -1.0, 0.0, 0.0);
	}
	frame(fighter.lua_state_agent, 17.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	macros::FT_MOTION_RATE(fighter, 0.5);
	frame(fighter.lua_state_agent, 19.0);
	if macros::is_excute(fighter) {
		if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD) == true {
			ArticleModule::add_motion_partial(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_close"), 5.0, 5.0, false, false, 0.0, false, true, false);
		}
		if MotionModule::is_changing(fighter.module_accessor) == true {
			WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
		}
		WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	frame(fighter.lua_state_agent, 43.0);
	macros::FT_MOTION_RATE(fighter, 0.75);
	frame(fighter.lua_state_agent, 75.0);
	if macros::is_excute(fighter) {
		notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
	}
	frame(fighter.lua_state_agent, 83.0);
	macros::FT_MOTION_RATE(fighter,	1.0);
}

#[acmd_script(agent = "elight", script = "game_attackairlw", category = ACMD_GAME)]
unsafe fn elight_game_attackairlw(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 0.25);
	frame(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	macros::FT_MOTION_RATE(fighter, 0.3333333);
	frame(fighter.lua_state_agent, 11.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	frame(fighter.lua_state_agent, 12.0);
	if macros::is_excute(fighter) {
		if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD) == true {
			ArticleModule::add_motion_partial(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 5.0, 5.0, false, false, 0.0, false, true, false);
		}
		if MotionModule::is_changing(fighter.module_accessor) == true {
			WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
		}
	}
	frame(fighter.lua_state_agent, 15.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 50, 80, 0, 60, 3.0, 0.0, 0.0, 3.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 8.0, 50, 80, 0, 60, 2.4, 0.0, -1.0, 7.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 8.0, 50, 80, 0, 60, 2.4, 0.0, -2.0, 11.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 8.0, 50, 80, 0, 60, 3.0, 0.0, 2.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
	}
	frame(fighter.lua_state_agent, 16.0);
	if macros::is_excute(fighter) {
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 0, Hash40::new("top"), 0.0, -2.0, -2.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 1, Hash40::new("top"), 0.0, -7.0, -1.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 2, Hash40::new("top"), 0.0, -9.0, 0.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 3, Hash40::new("top"), 0.0, 2.0, -3.0);
	}
	frame(fighter.lua_state_agent, 17.0);
	if macros::is_excute(fighter) {
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 0, Hash40::new("top"), 0.0, 1.0, -11.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 1, Hash40::new("top"), 0.0, -2.0, -13.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 2, Hash40::new("top"), 0.0, -5.0, -14.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 3, Hash40::new("top"), 0.0, 4.0, -7.0);
	}
	frame(fighter.lua_state_agent, 18.0);
	if macros::is_excute(fighter) {
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 0, Hash40::new("top"), 0.0, 6.0, -13.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 1, Hash40::new("top"), 0.0, 5.0, -16.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 2, Hash40::new("top"), 0.0, 4.0, -19.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 3, Hash40::new("top"), 0.0, 8.0, -8.0);
	}
	frame(fighter.lua_state_agent, 19.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	frame(fighter.lua_state_agent, 21.0);
	if macros::is_excute(fighter) {
		if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD) == true {
			ArticleModule::add_motion_partial(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_close"), 5.0, 5.0, false, false, 0.0, false, true, false);
		}
		if MotionModule::is_changing(fighter.module_accessor) == true {
			WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
		}
	}
	macros::FT_MOTION_RATE(fighter, 0.7);
	frame(fighter.lua_state_agent, 33.0);
	if macros::is_excute(fighter) {
		WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	frame(fighter.lua_state_agent, 51.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	frame(fighter.lua_state_agent, 72.0);
	if macros::is_excute(fighter) {
		notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
	}
}

#[acmd_script(agent = "elight", script = "game_attackairn", category = ACMD_GAME)]
unsafe fn elight_game_attackairn(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 0.25);
	frame(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD) == true {
			ArticleModule::add_motion_partial(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 5.0, 5.0, false, false, 0.0, false, true, false);
		}
		if MotionModule::is_changing(fighter.module_accessor) == true {
			WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
		}
	}
	frame(fighter.lua_state_agent, 5.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 2.0, 100, 100, 80, 0, 6.5, -4.0, 5.0, 0.0, None, None, None, 0.75, 0.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 1, 0, Hash40::new("haver"), 2.0, 100, 100, 80, 0, 5.0, -4.0, 12.0, 0.0, None, None, None, 0.75, 0.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 2, 0, Hash40::new("haver"), 2.0, 367, 100, 80, 0, 6.5, -4.0, 5.0, 0.0, None, None, None, 0.75, 0.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 3, 0, Hash40::new("haver"), 2.0, 367, 100, 80, 0, 5.0, -4.0, 12.0, 0.0, None, None, None, 0.75, 0.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
	}
	frame(fighter.lua_state_agent, 12.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
		macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 3.0, 60, 125, 0, 50, 6.5, -4.0, 5.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 1, 0, Hash40::new("haver"), 3.0, 60, 125, 0, 50, 5.0, -4.0, 12.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 4, 0, Hash40::new("top"), 3.0, 60, 125, 0, 50, 10.0, 0.0, 10.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
	}
	frame(fighter.lua_state_agent, 16.0);
	if macros::is_excute(fighter) {
		AttackModule::clear(fighter.module_accessor, 4, false);
	}
	frame(fighter.lua_state_agent, 18.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	frame(fighter.lua_state_agent, 24.0);
	if macros::is_excute(fighter) {
		if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD) == true {
			ArticleModule::add_motion_partial(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_close"), 5.0, 5.0, false, false, 0.0, false, true, false);
		}
		if MotionModule::is_changing(fighter.module_accessor) == true {
			WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
		}
	}
	frame(fighter.lua_state_agent, 36.0);
	if macros::is_excute(fighter) {
		WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	frame(fighter.lua_state_agent, 72.0);
	if macros::is_excute(fighter) {
		notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
	}
}

#[acmd_script(agent = "elight", script = "game_attackdash", category = ACMD_GAME)]
unsafe fn elight_game_attackdash(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 0.5);
	frame(fighter.lua_state_agent, 9.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	frame(fighter.lua_state_agent, 10.0);
	if macros::is_excute(fighter) {
		if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD) == true {
			ArticleModule::add_motion_partial(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 5.0, 5.0, false, false, 0.0, false, true, false);
		}
		if MotionModule::is_changing(fighter.module_accessor) == true {
			WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
		}
	}
	macros::FT_MOTION_RATE(fighter, 0.5);
	frame(fighter.lua_state_agent, 14.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 361, 55, 0, 90, 5.0, 0.0, 7.0, 13.0, Some(0.0), Some(7.0), Some(17.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 7.0, 361, 55, 0, 90, 5.0, 0.0, 7.0, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
	}
	frame(fighter.lua_state_agent, 16.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	frame(fighter.lua_state_agent, 29.0);
	if macros::is_excute(fighter) {
		if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD) == true {
			ArticleModule::add_motion_partial(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_close"), 5.0, 5.0, false, false, 0.0, false, true, false);
		}
		if MotionModule::is_changing(fighter.module_accessor) == true {
			WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
		}
	}
	macros::FT_MOTION_RATE(fighter, 0.7);
	frame(fighter.lua_state_agent, 69.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
}

#[acmd_script(agent = "elight", script = "game_attackhi3", category = ACMD_GAME)]
unsafe fn elight_game_attackhi3(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 0.2);
	frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD) == true {
			ArticleModule::add_motion_partial(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 5.0, 5.0, false, false, 0.0, false, true, false);
		}
		if MotionModule::is_changing(fighter.module_accessor) == true {
			WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
		}
	}
	macros::FT_MOTION_RATE(fighter, 0.5);
	frame(fighter.lua_state_agent, 8.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	frame(fighter.lua_state_agent, 9.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 95, 104, 0, 50, 2.6, 0.0, 17.0, -7.5, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 4.0, 95, 104, 0, 50, 2.4, 0.0, 15.0, -11.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 4.0, 95, 104, 0, 50, 2.0, 0.0, 13.0, -14.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 4.0, 95, 104, 0, 50, 2.2, 0.0, 18.0, -3.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
	}
	frame(fighter.lua_state_agent, 10.0);
	if macros::is_excute(fighter) {
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 0, Hash40::new("top"), 0.0, 21.0, -4.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 1, Hash40::new("top"), 0.0, 23.0, -8.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 2, Hash40::new("top"), 0.0, 25.0, -11.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 3, Hash40::new("top"), 0.0, 19.0, 0.0);
	}
	frame(fighter.lua_state_agent, 11.0);
	if macros::is_excute(fighter) {
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 0, Hash40::new("top"), 0.0, 22.0, 3.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 1, Hash40::new("top"), 0.0, 25.0, 2.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 2, Hash40::new("top"), 0.0, 28.0, 1.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 3, Hash40::new("top"), 0.0, 17.0, 4.0);
	}
	frame(fighter.lua_state_agent, 12.0);
	if macros::is_excute(fighter) {
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 0, Hash40::new("top"), 0.0, 17.0, 7.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 1, Hash40::new("top"), 0.0, 20.0, 8.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 2, Hash40::new("top"), 0.0, 24.0, 9.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 3, Hash40::new("top"), 0.0, 12.0, 5.0);
	}
	frame(fighter.lua_state_agent, 13.0);
	if macros::is_excute(fighter) {
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 0, Hash40::new("top"), 0.0, 11.0, 10.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 1, Hash40::new("top"), 0.0, 14.0, 12.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 2, Hash40::new("top"), 0.0, 16.0, 14.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 3, Hash40::new("top"), 0.0, 9.0, 5.0);
	}
	frame(fighter.lua_state_agent, 14.0);
	if macros::is_excute(fighter) {
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 0, Hash40::new("top"), 0.0, 5.0, 7.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 1, Hash40::new("top"), 0.0, 5.0, 11.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 2, Hash40::new("top"), 0.0, 5.0, 14.5);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 3, Hash40::new("top"), 0.0, 5.0, 5.0);
	}
	frame(fighter.lua_state_agent, 15.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	macros::FT_MOTION_RATE(fighter, 0.5);
	frame(fighter.lua_state_agent, 18.0);
	if macros::is_excute(fighter) {
		if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD) == true {
			ArticleModule::add_motion_partial(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_close"), 5.0, 5.0, false, false, 0.0, false, true, false);
		}
		if MotionModule::is_changing(fighter.module_accessor) == true {
			WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
		}
	}
	frame(fighter.lua_state_agent, 39.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
}

#[acmd_script(agent = "elight", script = "game_attackhi4", category = ACMD_GAME)]
unsafe fn elight_game_attackhi4(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD) == true {
			ArticleModule::add_motion_partial(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 5.0, 5.0, false, false, 0.0, false, true, false);
		}
		if MotionModule::is_changing(fighter.module_accessor) == true {
			WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
		}
	}
	macros::FT_MOTION_RATE(fighter, 0.5);
	frame(fighter.lua_state_agent, 7.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
		WorkModule::set_int64(fighter.module_accessor, hash40("attack_hi4_hold") as i64, *FIGHTER_ELIGHT_INSTANCE_WORK_ID_INT_ESWORD_INHERIT_OPEN_MOTION_KIND);
	}
	frame(fighter.lua_state_agent, 11.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	frame(fighter.lua_state_agent, 13.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 368, 100, 0, 0, 5.5, 0.0, 8.0, 8.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		AttackModule::set_vec_target_pos(fighter.module_accessor, 0, Hash40::new("top"), &Vector2f{x: 0.0, y: 23.0}, 7.0 as u32, false);
	}
	frame(fighter.lua_state_agent, 14.0);
	if macros::is_excute(fighter) {
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 0, Hash40::new("top"), 0.0, 14.0, 9.0);
	}
	frame(fighter.lua_state_agent, 15.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 1, Hash40::new("top"), 1.5, 366, 100, 50, 0, 2.0, 0.0, 26.0, 0.0, Some(0.0), Some(26.0), Some(7.0), 0.5, 0.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, -1.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 1, 1, Hash40::new("top"), 1.5, 368, 100, 0, 0, 2.0, 0.0, 21.0, 0.0, Some(0.0), Some(21.0), Some(5.0), 0.5, 0.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, -1.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 2, 1, Hash40::new("top"), 1.5, 368, 100, 0, 0, 4.5, 0.0, 21.0, 8.0, None, None, None, 0.5, 0.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, -1.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		AttackModule::set_vec_target_pos(fighter.module_accessor, 1, Hash40::new("top"), &Vector2f{x: 0.0, y: 23.0}, 10.0 as u32, false);
		AttackModule::set_vec_target_pos(fighter.module_accessor, 2, Hash40::new("top"), &Vector2f{x: 0.0, y: 23.0}, 10.0 as u32, false);
	}
	frame(fighter.lua_state_agent, 17.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 1, Hash40::new("top"), 1.5, 366, 100, 50, 0, 3.0, 0.0, 26.0, -5.0, Some(0.0), Some(26.0), Some(7.0), 0.5, 0.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, -1.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 1, 1, Hash40::new("top"), 1.5, 368, 100, 0, 0, 3.0, 0.0, 21.0, -3.0, Some(0.0), Some(21.0), Some(5.0), 0.5, 0.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, -1.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		AttackModule::set_vec_target_pos(fighter.module_accessor, 1, Hash40::new("top"), &Vector2f{x: 0.0, y: 23.0}, 10.0 as u32, false);
	}
	frame(fighter.lua_state_agent, 18.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 1, Hash40::new("top"), 1.5, 366, 100, 50, 0, 3.0, 0.0, 26.0, -10.0, Some(0.0), Some(26.0), Some(7.0), 0.5, 0.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, -1.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 1, 1, Hash40::new("top"), 1.5, 368, 100, 0, 0, 3.0, 0.0, 21.0, -8.0, Some(0.0), Some(21.0), Some(5.0), 0.5, 0.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, -1.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		AttackModule::set_vec_target_pos(fighter.module_accessor, 1, Hash40::new("top"), &Vector2f{x: 0.0, y: 23.0}, 10.0 as u32, false);
	}
	frame(fighter.lua_state_agent, 19.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 3, 1, Hash40::new("top"), 1.5, 368, 100, 0, 0, 4.5, 0.0, 21.0, -11.0, None, None, None, 0.5, 0.1, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, -1.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		AttackModule::set_vec_target_pos(fighter.module_accessor, 3, Hash40::new("top"), &Vector2f{x: 0.0, y: 23.0}, 10.0 as u32, false);
	}
	frame(fighter.lua_state_agent, 30.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 85, 105, 0, 80, 7.0, 0.0, 25.0, -7.0, Some(0.0), Some(25.0), Some(4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
	}
	frame(fighter.lua_state_agent, 32.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	frame(fighter.lua_state_agent, 34.0);
	if macros::is_excute(fighter) {
		if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD) == true {
			ArticleModule::add_motion_partial(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_close"), 5.0, 5.0, false, false, 0.0, false, true, false);
		}
		if MotionModule::is_changing(fighter.module_accessor) == true {
			WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
		}
	}
	macros::FT_MOTION_RATE(fighter, 0.7);
	frame(fighter.lua_state_agent, 84.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
}

#[acmd_script(agent = "elight", script = "game_attacklw3", category = ACMD_GAME)]
unsafe fn elight_game_attacklw3(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD) == true {
			ArticleModule::add_motion_partial(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 5.0, 5.0, false, false, 0.0, false, true, false);
		}
		if MotionModule::is_changing(fighter.module_accessor) == true {
			WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
		}
	}
	macros::FT_MOTION_RATE(fighter, 0.5);
	frame(fighter.lua_state_agent, 7.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 85, 96, 0, 60, 3.0, 0.0, 2.0, 4.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 4.0, 85, 96, 0, 60, 3.0, 0.0, 2.0, 8.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 4.0, 85, 96, 0, 60, 2.5, 0.0, 2.0, 13.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 4.0, 95, 96, 0, 60, 2.0, 0.0, 2.0, 16.5, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
	}
	frame(fighter.lua_state_agent, 10.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	frame(fighter.lua_state_agent, 13.0);
	if macros::is_excute(fighter) {
		if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD) == true {
			ArticleModule::add_motion_partial(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_close"), 5.0, 5.0, false, false, 0.0, false, true, false);
		}
		if MotionModule::is_changing(fighter.module_accessor) == true {
			WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
		}
	}
	macros::FT_MOTION_RATE(fighter, 0.5);
	frame(fighter.lua_state_agent, 27.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
}

#[acmd_script(agent = "elight", script = "game_attacklw4", category = ACMD_GAME)]
unsafe fn elight_game_attacklw4(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD) == true {
			ArticleModule::add_motion_partial(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 5.0, 5.0, false, false, 0.0, false, true, false);
		}
		if MotionModule::is_changing(fighter.module_accessor) == true {
			WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
		}
	}
	macros::FT_MOTION_RATE(fighter, 0.5);
	frame(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
		WorkModule::set_int64(fighter.module_accessor, hash40("attack_lw4_hold") as i64, *FIGHTER_ELIGHT_INSTANCE_WORK_ID_INT_ESWORD_INHERIT_OPEN_MOTION_KIND);
	}
	macros::FT_MOTION_RATE(fighter, 0.4);
	frame(fighter.lua_state_agent, 8.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	frame(fighter.lua_state_agent, 10.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 35, 69, 0, 80, 3.0, 0.0, 7.0, 3.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 10.5, 35, 68, 0, 80, 4.0, 0.0, 7.0, 15.0, Some(0.0), Some(7.0), Some(10.0), 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 6.0, 35, 65, 0, 80, 3.0, 0.0, 7.0, -4.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 7.5, 35, 65, 0, 80, 3.0, 0.0, 7.0, -12.0, Some(0.0), Some(7.0), Some(-10.0), 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	frame(fighter.lua_state_agent, 13.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	frame(fighter.lua_state_agent, 16.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 35, 69, 0, 80, 3.0, 0.0, 7.0, -3.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 10.5, 35, 68, 0, 80, 4.0, 0.0, 7.0, -15.0, Some(0.0), Some(7.0), Some(-10.0), 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 6.0, 35, 65, 0, 80, 3.0, 0.0, 7.0, 4.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 7.5, 35, 65, 0, 80, 3.0, 0.0, 7.0, 12.0, Some(0.0), Some(7.0), Some(10.0), 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	frame(fighter.lua_state_agent, 19.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	macros::FT_MOTION_RATE(fighter, 0.75);
	frame(fighter.lua_state_agent, 42.0);
	if macros::is_excute(fighter) {
		if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD) == true {
			ArticleModule::add_motion_partial(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_close"), 5.0, 5.0, false, false, 0.0, false, true, false);
		}
		if MotionModule::is_changing(fighter.module_accessor) == true {
			WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
		}
	}
}

#[acmd_script(agent = "elight", script = "game_attacks3", category = ACMD_GAME)]
unsafe fn elight_game_attacks3(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 0.25);
	frame(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD) == true {
			ArticleModule::add_motion_partial(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 5.0, 5.0, false, false, 0.0, false, true, false);
		}
		if MotionModule::is_changing(fighter.module_accessor) == true {
			WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
		}
	}
	macros::FT_MOTION_RATE(fighter, 0.5);
	frame(fighter.lua_state_agent, 9.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	frame(fighter.lua_state_agent, 10.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 6.0, 60, 90, 0, 50, 3.0, -1.5, 4.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 1, 0, Hash40::new("haver"), 6.0, 60, 90, 0, 50, 3.0, -1.5, 8.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 2, 0, Hash40::new("haver"), 6.0, 60, 90, 0, 50, 2.5, -1.5, 12.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 6.0, 60, 90, 0, 50, 3.0, 0.0, 10.0, 6.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
	}
	frame(fighter.lua_state_agent, 11.0);
	if macros::is_excute(fighter) {
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 0, Hash40::new("haver"), 2.0, 6.0, 0.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 1, Hash40::new("haver"), 2.0, 10.0, 0.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 2, Hash40::new("haver"), 2.0, 13.0, 0.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 3, Hash40::new("top"), 0.0, 5.0, 6.0);
		AttackModule::set_size(fighter.module_accessor, 0, 3.5);
		AttackModule::set_size(fighter.module_accessor, 1, 3.5);
		AttackModule::set_size(fighter.module_accessor, 2, 3.0);
	}
	frame(fighter.lua_state_agent, 12.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	macros::FT_MOTION_RATE(fighter, 0.5);
	frame(fighter.lua_state_agent, 22.0);
	if macros::is_excute(fighter) {
		if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD) == true {
			ArticleModule::add_motion_partial(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_close"), 5.0, 5.0, false, false, 0.0, false, true, false);
		}
		if MotionModule::is_changing(fighter.module_accessor) == true {
			WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
		}
	}
	frame(fighter.lua_state_agent, 30.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
}

#[acmd_script(agent = "elight", script = "game_attacks4", category = ACMD_GAME)]
unsafe fn elight_game_attacks4(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD) == true {
			ArticleModule::add_motion_partial(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 5.0, 5.0, false, false, 0.0, false, true, false);
		}
		if MotionModule::is_changing(fighter.module_accessor) == true {
			WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
		}
	}
	macros::FT_MOTION_RATE(fighter, 0.5);
	frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
		WorkModule::set_int64(fighter.module_accessor, hash40("attack_s4_hold") as i64, *FIGHTER_ELIGHT_INSTANCE_WORK_ID_INT_ESWORD_INHERIT_OPEN_MOTION_KIND);
	}
	frame(fighter.lua_state_agent, 9.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	frame(fighter.lua_state_agent, 18.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 361, 70, 0, 70, 3.5, 0.0, 12.0, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 14.0, 361, 70, 0, 70, 3.0, 0.0, 17.0, 10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 14.0, 361, 70, 0, 70, 2.5, 0.0, 21.0, 11.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 14.0, 361, 70, 0, 70, 2.0, 0.0, 21.0, 11.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 4, 0, Hash40::new("top"), 14.0, 361, 70, 0, 70, 3.0, 0.0, 8.0, 5.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
	}
	frame(fighter.lua_state_agent, 19.0);
	if macros::is_excute(fighter) {
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 0, Hash40::new("top"), 0.0, 7.0, 12.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 1, Hash40::new("top"), 0.0, 8.0, 15.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 2, Hash40::new("top"), 0.0, 11.0, 19.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 3, Hash40::new("top"), 0.0, 13.0, 22.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 4, Hash40::new("top"), 0.0, 5.0, 7.0);
	}
	frame(fighter.lua_state_agent, 20.0);
	if macros::is_excute(fighter) {
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 0, Hash40::new("top"), 0.0, 2.0, 11.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 1, Hash40::new("top"), 0.0, 2.0, 16.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 2, Hash40::new("top"), 0.0, 2.0, 20.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 3, Hash40::new("top"), 0.0, 2.0, 23.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 4, Hash40::new("top"), 0.0, 2.0, 5.5);
	}
	frame(fighter.lua_state_agent, 22.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	macros::FT_MOTION_RATE(fighter, 0.75);
	frame(fighter.lua_state_agent, 37.0);
	if macros::is_excute(fighter) {
		if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD) == true {
			ArticleModule::add_motion_partial(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_close"), 5.0, 5.0, false, false, 0.0, false, true, false);
		}
		if MotionModule::is_changing(fighter.module_accessor) == true {
			WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
		}
	}
	frame(fighter.lua_state_agent, 66.0);
	macros::FT_MOTION_RATE(fighter, 0.85);
	frame(fighter.lua_state_agent, 106.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
}

#[acmd_script(agent = "elight", script = "game_catch", category = ACMD_GAME)]
unsafe fn elight_game_catch(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		GrabModule::set_rebound(fighter.module_accessor, true);
	}
	frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		macros::CATCH(fighter, 0, Hash40::new("top"), 3.3, 0.0, 8.5, 4.0, Some(0.0), Some(8.5), Some(8.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
		macros::CATCH(fighter, 1, Hash40::new("top"), 1.65, 0.0, 8.5, 2.35, Some(0.0), Some(8.5), Some(10.35), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
	}
	macros::game_CaptureCutCommon(fighter);
	wait(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(fighter.module_accessor, false);
	}
}

#[acmd_script(agent = "elight", script = "game_catchattack", category = ACMD_GAME)]
unsafe fn elight_game_catchattack(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.3, 361, 100, 30, 0, 5.0, 0.0, 10.0, 10.0, None, None, None, 2.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
		AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
	}
	wait(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

#[acmd_script(agent = "elight", script = "game_catchdash", category = ACMD_GAME)]
unsafe fn elight_game_catchdash(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter,	0.5);
	frame(fighter.lua_state_agent, 9.0);
	macros::FT_MOTION_RATE(fighter,	1.0);
	frame(fighter.lua_state_agent, 11.0);
	if macros::is_excute(fighter) {
		GrabModule::set_rebound(fighter.module_accessor, true);
	}
	frame(fighter.lua_state_agent, 12.0);
	if macros::is_excute(fighter) {
		macros::CATCH(fighter, 0, Hash40::new("top"), 3.0, 0.0, 6.6, 3.0, Some(0.0), Some(6.6), Some(11.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
		macros::CATCH(fighter, 1, Hash40::new("top"), 1.5, 0.0, 6.6, 1.5, Some(0.0), Some(6.6), Some(13.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
	}
	macros::game_CaptureCutCommon(fighter);
	wait(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(fighter.module_accessor, false);
	}
}

#[acmd_script(agent = "elight", script = "game_catchturn", category = ACMD_GAME)]
unsafe fn elight_game_catchturn(fighter: &mut L2CAgentBase) {
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
		macros::CATCH(fighter, 0, Hash40::new("top"), 3.3, 0.0, 6.6, -6.0, Some(0.0), Some(6.6), Some(-14.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
		macros::CATCH(fighter, 1, Hash40::new("top"), 1.65, 0.0, 6.6, -4.35, Some(0.0), Some(6.6), Some(-16.35), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
	}
	macros::game_CaptureCutCommon(fighter);
	wait(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(fighter.module_accessor, false);
	}
}

#[acmd_script(agent = "elight", script = "game_landingairn", category = ACMD_GAME)]
unsafe fn elight_game_landingairn(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 65, 125, 0, 60, 3.0, 0.0, 3.0, -4.0, Some(0.0), Some(3.0), Some(4.0), 0.75, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 0.2);
	}
	frame(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

#[acmd_script(agent = "elight", script = "game_throwb", category = ACMD_GAME)]
unsafe fn elight_game_throwb(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 3.0, 45, 95, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
	}
	frame(fighter.lua_state_agent, 15.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 45, 135, 0, 40, 5.0, 0.0, 13.0, -11.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
		macros::REVERSE_LR(fighter);
	}
	frame(fighter.lua_state_agent, 16.0);
	if macros::is_excute(fighter) {
		macros::CHECK_FINISH_CAMERA(fighter, 18, 9);
		let fighter_cutin_manager = *(FIGHTER_CUTIN_MANAGER_ADDR as *mut *mut smash::app::FighterCutInManager);
		FighterCutInManager::set_throw_finish_zoom_rate(fighter_cutin_manager, 1.5);
		FighterCutInManager::set_throw_finish_offset(fighter_cutin_manager, Vector3f{x: 7.0, y: 3.0, z: 0.0});
	}
	frame(fighter.lua_state_agent, 17.0);
	if macros::is_excute(fighter) {
		macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
		AttackModule::clear_all(fighter.module_accessor);
	}
}

#[acmd_script(agent = "elight", script = "game_throwf", category = ACMD_GAME)]
unsafe fn elight_game_throwf(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 3.5, 35, 95, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
	}
	frame(fighter.lua_state_agent, 10.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.5, 35, 140, 0, 40, 6.0, 0.0, 11.0, 12.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
		AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
		macros::CHECK_FINISH_CAMERA(fighter, 13, 8);
		let fighter_cutin_manager = *(FIGHTER_CUTIN_MANAGER_ADDR as *mut *mut smash::app::FighterCutInManager);
		FighterCutInManager::set_throw_finish_zoom_rate(fighter_cutin_manager, 1.3);
		FighterCutInManager::set_throw_finish_offset(fighter_cutin_manager, Vector3f{x: 5.0, y: 0.0, z: 0.0});
	}
	frame(fighter.lua_state_agent, 11.0);
	if macros::is_excute(fighter) {
		macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
		AttackModule::clear_all(fighter.module_accessor);
	}
}

#[acmd_script(agent = "elight", script = "game_throwhi", category = ACMD_GAME)]
unsafe fn elight_game_throwhi(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 4.0, 90, 75, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
	}
	frame(fighter.lua_state_agent, 8.0);
	if macros::is_excute(fighter) {
		macros::CHECK_FINISH_CAMERA(fighter, 5, 7);
		let fighter_cutin_manager = *(FIGHTER_CUTIN_MANAGER_ADDR as *mut *mut smash::app::FighterCutInManager);
		FighterCutInManager::set_throw_finish_zoom_rate(fighter_cutin_manager, 1.5);
		FighterCutInManager::set_throw_finish_offset(fighter_cutin_manager, Vector3f{x: 0.0, y: 0.0, z: 0.0});
	}
	frame(fighter.lua_state_agent, 9.0);
	if macros::is_excute(fighter) {
		macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
	}
	frame(fighter.lua_state_agent, 15.0);
	macros::FT_MOTION_RATE(fighter,	0.3);
}

#[acmd_script(agent = "elight", script = "game_throwlw", category = ACMD_GAME)]
unsafe fn elight_game_throwlw(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		macros::FT_LEAVE_NEAR_OTTOTTO(fighter, -2.5, 2.5);
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 4.0, 80, 105, 0, 80, 0.5, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.5, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
	}
	frame(fighter.lua_state_agent, 10.0);
	if macros::is_excute(fighter) {
		if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD) == true {
			ArticleModule::add_motion_partial(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 3.33, 3.33, false, false, 0.0, false, true, false);
		}
		if MotionModule::is_changing(fighter.module_accessor) == true {
			WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
		}
	}
	frame(fighter.lua_state_agent, 20.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 80, 145, 0, 40, 4.5, 0.0, 3.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
	}
	wait(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		macros::CHECK_FINISH_CAMERA(fighter, 0, 0);
		let fighter_cutin_manager = *(FIGHTER_CUTIN_MANAGER_ADDR as *mut *mut smash::app::FighterCutInManager);
		FighterCutInManager::set_throw_finish_zoom_rate(fighter_cutin_manager, 1.2);
		FighterCutInManager::set_throw_finish_offset(fighter_cutin_manager, Vector3f{x: 0.0, y: 0.0, z: 0.0});
	}
	wait(fighter.lua_state_agent, 4.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
		macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
	}
	frame(fighter.lua_state_agent, 42.0);
	if macros::is_excute(fighter) {
		if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD) == true {
			ArticleModule::add_motion_partial(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_close"), 3.33, 3.33, false, false, 0.0, false, true, false);
		}
		if MotionModule::is_changing(fighter.module_accessor) == true {
			WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
		}
	}
}

#[acmd_script(agent = "elight_bunshin", script = 0x13ee1e5627, category = ACMD_GAME)]
unsafe fn elight_bunshin_0x13ee1e5627(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		if ArticleModule::is_exist(fighter.module_accessor, *WEAPON_ELIGHT_BUNSHIN_GENERATE_ARTICLE_ESWORD) == true {
			ArticleModule::add_motion_partial(fighter.module_accessor, *WEAPON_ELIGHT_BUNSHIN_GENERATE_ARTICLE_ESWORD, *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 10.0, 10.0, false, false, 0.0, false, true, false);
		}
	}
	frame(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 60, 60, 0, 90, 9.0, 0.0, 8.0, 17.0, Some(0.0), Some(8.0), Some(40.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		WorkModule::on_flag(fighter.module_accessor, *WEAPON_ELIGHT_BUNSHIN_INSTANCE_WORK_ID_FLAG_FINISH);
	}
	frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
		VisibilityModule::set_whole(fighter.module_accessor, false);
	}
}

#[acmd_script(agent = "elight_bunshin", scripts = ["game_specials5", "game_specialairs5"], category = ACMD_GAME)]
unsafe fn elight_bunshin_game_specials5(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		if ArticleModule::is_exist(fighter.module_accessor, *WEAPON_ELIGHT_BUNSHIN_GENERATE_ARTICLE_ESWORD) == true {
			ArticleModule::add_motion_partial(fighter.module_accessor, *WEAPON_ELIGHT_BUNSHIN_GENERATE_ARTICLE_ESWORD, *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 10.0, 10.0, false, false, 0.0, false, true, false);
		}
	}
	frame(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 60, 60, 0, 90, 9.0, 0.0, 8.0, 5.0, Some(0.0), Some(8.0), Some(20.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		WorkModule::on_flag(fighter.module_accessor, *WEAPON_ELIGHT_BUNSHIN_INSTANCE_WORK_ID_FLAG_FINISH);
	}
	frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

#[acmd_script(agent = "elight_exprosiveshot", script = "game_burst", category = ACMD_GAME)]
unsafe fn elight_exprosiveshot_game_burst(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_beams"), 0, false, 0);
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 55, 105, 0, 60, 6.0, 0.0, 3.0, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 55, 110, 0, 60, 11.0, 0.0, 3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
	}
}

#[acmd_script(agent = "elight_exprosiveshot", script = "game_fly", category = ACMD_GAME)]
unsafe fn elight_exprosiveshot_game_fly(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 70, 55, 0, 45, 6.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
	}
}

#[acmd_script(agent = "elight_spreadbullet", script = "game_fly", category = ACMD_GAME)]
unsafe fn elight_spreadbullet_game_fly(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.5, 50, 115, 0, 60, 2.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
	}
}

pub fn install() {
	smashline::install_acmd_scripts!(
		elight_game_attack11,
		elight_game_attack12,
		elight_game_attack13,
		elight_game_attack100,
		elight_game_attack100end,
		elight_game_attack100start,
		elight_game_attack100sub,
		elight_game_attackairb,
		elight_game_attackairf,
		elight_game_attackairhi,
		elight_game_attackairlw,
		elight_game_attackairn,
		elight_game_attackdash,
		elight_game_attackhi3,
		elight_game_attackhi4,
		elight_game_attacklw3,
		elight_game_attacklw4,
		elight_game_attacks3,
		elight_game_attacks4,
		elight_game_catch,
		elight_game_catchattack,
		elight_game_catchdash,
		elight_game_catchturn,
		elight_game_landingairn,
		elight_game_throwb,
		elight_game_throwf,
		elight_game_throwhi,
		elight_game_throwlw,
		elight_bunshin_0x13ee1e5627,
		elight_bunshin_game_specials5,
		elight_exprosiveshot_game_burst,
		elight_exprosiveshot_game_fly,
		elight_spreadbullet_game_fly,
	);
}
