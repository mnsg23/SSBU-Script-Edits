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

#[acmd_script(agent = "eflame", script = "game_attack11", category = ACMD_GAME)]
unsafe fn eflame_game_attack11(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 361, 15, 0, 25, 2.5, 0.0, 8.0, 7.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 361, 15, 0, 25, 2.5, 0.0, 8.0, 8.5, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 3.0, 180, 5, 0, 25, 2.5, 0.0, 8.0, 11.5, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 3.0, 361, 5, 0, 25, 2.5, 0.0, 8.0, 11.5, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 6.0, false);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 6.0, false);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, 2, 6.0, false);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, 3, 6.0, false);
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

#[acmd_script(agent = "eflame", script = "game_attack12", category = ACMD_GAME)]
unsafe fn eflame_game_attack12(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 0.5);
	frame(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD) == true {
			ArticleModule::add_motion_partial(fighter.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, *WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 5.0, 5.0, false, false, 0.0, false, true, false);
		}
		if MotionModule::is_changing(fighter.module_accessor) == true {
			WorkModule::on_flag(fighter.module_accessor, *FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
		}
	}
	macros::FT_MOTION_RATE(fighter, 1.0);
	frame(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 361, 20, 0, 25, 3.0, 0.0, 8.0, 7.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 361, 20, 0, 25, 3.0, 0.0, 8.0, 9.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.0, 180, 10, 0, 25, 4.0, 0.0, 8.0, 11.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 2.0, 361, 10, 0, 25, 4.0, 0.0, 8.0, 11.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 6.0, false);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 6.0, false);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, 2, 6.0, false);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, 3, 6.0, false);
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
	frame(fighter.lua_state_agent, 11.0);
	if macros::is_excute(fighter) {
		if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD) == true {
			ArticleModule::add_motion_partial(fighter.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, *WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_close"), 5.0, 5.0, false, false, 0.0, false, true, false);
		}
		if MotionModule::is_changing(fighter.module_accessor) == true {
			WorkModule::on_flag(fighter.module_accessor, *FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
		}
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
	}
	frame(fighter.lua_state_agent, 14.0);
	if macros::is_excute(fighter) {
		WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100);
	}
}

#[acmd_script(agent = "eflame", script = "game_attack13", category = ACMD_GAME)]
unsafe fn eflame_game_attack13(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD) == true {
			ArticleModule::add_motion_partial(fighter.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, *WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 5.0, 5.0, false, false, 0.0, false, true, false);
		}
		if MotionModule::is_changing(fighter.module_accessor) == true {
			WorkModule::on_flag(fighter.module_accessor, *FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
		}
	}
	frame(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 5.0, 361, 100, 0, 80, 3.5, 0.0, 4.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 1, 0, Hash40::new("haver"), 5.0, 361, 100, 0, 80, 3.0, -2.0, 8.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 2, 0, Hash40::new("haver"), 5.0, 361, 100, 0, 80, 3.0, -1.5, 13.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 3, 0, Hash40::new("haver"), 5.0, 361, 100, 0, 80, 2.5, -1.5, 15.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 4, 0, Hash40::new("top"), 5.0, 361, 100, 0, 80, 3.0, 0.0, 8.0, 5.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
	}
	frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 1, Hash40::new("haver"), 0.0, 8.0, 0.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 2, Hash40::new("haver"), 0.0, 13.0, 0.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 3, Hash40::new("haver"), 0.0, 15.0, 0.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 4, Hash40::new("top"), 0.0, 15.0, 5.0);
	}
	frame(fighter.lua_state_agent, 7.0);
	if macros::is_excute(fighter) {
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 1, Hash40::new("haver"), 2.0, 8.0, 0.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 2, Hash40::new("haver"), 1.5, 13.0, 0.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 3, Hash40::new("haver"), 1.5, 15.0, 0.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 4, Hash40::new("top"), 0.0, 16.0, 3.0);
	}
	frame(fighter.lua_state_agent, 8.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	frame(fighter.lua_state_agent, 21.0);
	if macros::is_excute(fighter) {
		if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD) == true {
			ArticleModule::add_motion_partial(fighter.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, *WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_close"), 5.0, 5.0, false, false, 0.0, false, true, false);
		}
		if MotionModule::is_changing(fighter.module_accessor) == true {
			WorkModule::on_flag(fighter.module_accessor, *FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
		}
	}
	macros::FT_MOTION_RATE(fighter, 0.5);
	frame(fighter.lua_state_agent, 31.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
}

#[acmd_script(agent = "eflame", script = "game_attack100", category = ACMD_GAME)]
unsafe fn eflame_game_attack100(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 2.0);
	for _ in 0..1000000 {
		for _ in 0..5 {
			if macros::is_excute(fighter) {
				macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.6, 361, 10, 0, 15, 6.0, 0.0, 7.0, 10.0, Some(0.0), Some(7.0), Some(15.0), 0.5, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
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
		wait(fighter.lua_state_agent, 1.0);
	}
}

#[acmd_script(agent = "eflame", script = "game_attack100end", category = ACMD_GAME)]
unsafe fn eflame_game_attack100end(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 0.5);
	frame(fighter.lua_state_agent, 5.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 40, 80, 0, 80, 8.0, 0.0, 10.0, 10.0, Some(0.0), Some(10.0), Some(18.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
	}
	frame(fighter.lua_state_agent, 8.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

#[acmd_script(agent = "eflame", script = "game_attack100start", category = ACMD_GAME)]
unsafe fn eflame_game_attack100start(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 0.375);
	frame(fighter.lua_state_agent, 9.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
}

#[acmd_script(agent = "eflame", script = "game_attack100sub", category = ACMD_GAME)]
unsafe fn eflame_game_attack100sub(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.6, 361, 10, 0, 15, 6.0, 0.0, 7.0, 10.0, Some(0.0), Some(7.0), Some(15.0), 0.5, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 4.0, false);
		macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 8);
	}
	wait(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
	}
}

#[acmd_script(agent = "eflame", script = "game_attackairb", category = ACMD_GAME)]
unsafe fn eflame_game_attackairb(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	macros::FT_MOTION_RATE(fighter, 2.0);
	frame(fighter.lua_state_agent, 7.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	frame(fighter.lua_state_agent, 9.0);
	if macros::is_excute(fighter) {
		if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD) == true {
			ArticleModule::add_motion_partial(fighter.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, *WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 5.0, 5.0, false, false, 0.0, false, true, false);
		}
		if MotionModule::is_changing(fighter.module_accessor) == true {
			WorkModule::on_flag(fighter.module_accessor, *FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
		}
	}
	frame(fighter.lua_state_agent, 14.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 14.0, 60, 85, 0, 70, 3.0, 0.0, 6.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 1, 0, Hash40::new("haver"), 14.0, 60, 85, 0, 70, 2.4, 0.0, 9.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 2, 0, Hash40::new("haver"), 14.0, 60, 85, 0, 70, 2.4, 0.0, 12.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 3, 0, Hash40::new("haver"), 14.0, 60, 85, 0, 70, 2.0, 0.0, 15.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 4, 0, Hash40::new("haver"), 14.0, 60, 85, 0, 70, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 5, 0, Hash40::new("haver"), 14.0, 60, 85, 0, 70, 8.0, 0.0, 7.0, -5.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
	}
	frame(fighter.lua_state_agent, 15.0);
	if macros::is_excute(fighter) {
		AttackModule::clear(fighter.module_accessor, 5, false);
	}
	frame(fighter.lua_state_agent, 17.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	frame(fighter.lua_state_agent, 24.0);
	if macros::is_excute(fighter) {
		if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD) == true {
			ArticleModule::add_motion_partial(fighter.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, *WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_close"), 5.0, 5.0, false, false, 0.0, false, true, false);
		}
		if MotionModule::is_changing(fighter.module_accessor) == true {
			WorkModule::on_flag(fighter.module_accessor, *FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
		}
	}
	frame(fighter.lua_state_agent, 34.0);
	if macros::is_excute(fighter) {
		WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	frame(fighter.lua_state_agent, 71.0);
	if macros::is_excute(fighter) {
		notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
	}
}

#[acmd_script(agent = "eflame", script = "game_attackairf", category = ACMD_GAME)]
unsafe fn eflame_game_attackairf(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	macros::FT_MOTION_RATE(fighter, 2.0);
	frame(fighter.lua_state_agent, 4.0);
	if macros::is_excute(fighter) {
		if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD) == true {
			ArticleModule::add_motion_partial(fighter.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, *WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 5.0, 5.0, false, false, 0.0, false, true, false);
		}
		if MotionModule::is_changing(fighter.module_accessor) == true {
			WorkModule::on_flag(fighter.module_accessor, *FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
		}
	}
	frame(fighter.lua_state_agent, 6.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	frame(fighter.lua_state_agent, 8.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 13.0, 45, 82, 0, 70, 3.5, 2.0, 6.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 1, 0, Hash40::new("haver"), 13.0, 45, 82, 0, 70, 3.0, 2.0, 9.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 2, 0, Hash40::new("haver"), 13.0, 45, 82, 0, 70, 3.0, 2.0, 12.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 3, 0, Hash40::new("haver"), 13.0, 45, 82, 0, 70, 2.0, 2.0, 14.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 4, 0, Hash40::new("haver"), 13.0, 45, 82, 0, 70, 3.0, 2.0, 0.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 5, 0, Hash40::new("haver"), 13.0, 45, 82, 0, 70, 2.5, 2.0, -4.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 6, 0, Hash40::new("haver"), 13.0, 45, 82, 0, 70, 2.5, 2.0, 6.0, -4.0, Some(2.0), Some(13.0), Some(-4.0), 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
	}
	frame(fighter.lua_state_agent, 9.0);
	if macros::is_excute(fighter) {
		AttackModule::clear(fighter.module_accessor, 6, false);
	}
	frame(fighter.lua_state_agent, 10.0);
	if macros::is_excute(fighter) {
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 0, Hash40::new("haver"), -2.0, 6.0, 0.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 1, Hash40::new("haver"), -2.0, 9.0, 0.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 2, Hash40::new("haver"), -2.0, 12.0, 0.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 3, Hash40::new("haver"), -2.0, 14.0, 0.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 4, Hash40::new("haver"), -2.0, 0.0, 0.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 5, Hash40::new("haver"), -2.0, -4.0, 0.0);
		AttackModule::set_size(fighter.module_accessor, 5, 2.0);
	}
	frame(fighter.lua_state_agent, 11.0);
	if macros::is_excute(fighter) {
		AttackModule::clear(fighter.module_accessor, 5, false);
	}
	frame(fighter.lua_state_agent, 12.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	frame(fighter.lua_state_agent, 19.0);
	if macros::is_excute(fighter) {
		if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD) == true {
			ArticleModule::add_motion_partial(fighter.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, *WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_close"), 5.0, 5.0, false, false, 0.0, false, true, false);
		}
		if MotionModule::is_changing(fighter.module_accessor) == true {
			WorkModule::on_flag(fighter.module_accessor, *FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
		}
	}
	frame(fighter.lua_state_agent, 43.0);
	if macros::is_excute(fighter) {
		WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	frame(fighter.lua_state_agent, 69.0);
	if macros::is_excute(fighter) {
		notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
	}
}

#[acmd_script(agent = "eflame", script = "game_attackairhi", category = ACMD_GAME)]
unsafe fn eflame_game_attackairhi(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD) == true {
			ArticleModule::add_motion_partial(fighter.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, *WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 5.0, 5.0, false, false, 0.0, false, true, false);
		}
		if MotionModule::is_changing(fighter.module_accessor) == true {
			WorkModule::on_flag(fighter.module_accessor, *FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
		}
	}
	frame(fighter.lua_state_agent, 11.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 13.0, 80, 61, 0, 90, 4.0, 0.0, 5.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 1, 0, Hash40::new("haver"), 13.0, 80, 61, 0, 90, 3.5, 0.0, 8.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 2, 0, Hash40::new("haver"), 13.0, 80, 61, 0, 90, 3.0, 0.0, 12.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 3, 0, Hash40::new("haver"), 13.0, 80, 61, 0, 90, 2.5, 0.0, 16.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 4, 0, Hash40::new("haver"), 13.0, 80, 61, 0, 90, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
	}
	frame(fighter.lua_state_agent, 12.0);
	if macros::is_excute(fighter) {
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 0, Hash40::new("haver"), -1.0, 5.0, 0.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 1, Hash40::new("haver"), -2.0, 8.0, 0.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 2, Hash40::new("haver"), -3.0, 12.0, 0.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 3, Hash40::new("haver"), -3.0, 16.0, 0.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 4, Hash40::new("haver"), -1.0, 0.0, 0.0);
	}
	frame(fighter.lua_state_agent, 17.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	frame(fighter.lua_state_agent, 19.0);
	if macros::is_excute(fighter) {
		WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
	}
	frame(fighter.lua_state_agent, 23.0);
	if macros::is_excute(fighter) {
		if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD) == true {
			ArticleModule::add_motion_partial(fighter.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, *WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_close"), 5.0, 5.0, false, false, 0.0, false, true, false);
		}
		if MotionModule::is_changing(fighter.module_accessor) == true {
			WorkModule::on_flag(fighter.module_accessor, *FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
		}
	}
	frame(fighter.lua_state_agent, 75.0);
	if macros::is_excute(fighter) {
		notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
	}
}

#[acmd_script(agent = "eflame", script = "game_attackairlw", category = ACMD_GAME)]
unsafe fn eflame_game_attackairlw(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	frame(fighter.lua_state_agent, 9.0);
	if macros::is_excute(fighter) {
		if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD) == true {
			ArticleModule::add_motion_partial(fighter.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, *WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 5.0, 5.0, false, false, 0.0, false, true, false);
		}
		if MotionModule::is_changing(fighter.module_accessor) == true {
			WorkModule::on_flag(fighter.module_accessor, *FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
		}
	}
	frame(fighter.lua_state_agent, 15.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 13.0, 270, 115, 0, 20, 3.0, 0.0, 0.0, 3.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 13.0, 270, 115, 0, 20, 2.4, 0.0, -1.0, 7.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 13.0, 270, 115, 0, 20, 2.4, 0.0, -2.0, 11.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 13.0, 270, 115, 0, 20, 2.4, 0.0, -3.0, 14.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 4, 0, Hash40::new("top"), 13.0, 270, 115, 0, 20, 3.0, 0.0, 2.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
	}
	frame(fighter.lua_state_agent, 16.0);
	if macros::is_excute(fighter) {
		macros::ATK_POWER(fighter, 0, 16.0);
		macros::ATK_POWER(fighter, 1, 16.0);
		macros::ATK_POWER(fighter, 2, 16.0);
		macros::ATK_POWER(fighter, 3, 16.0);
		macros::ATK_POWER(fighter, 4, 16.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 0, Hash40::new("top"), 0.0, -2.0, -2.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 1, Hash40::new("top"), 0.0, -7.0, -1.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 2, Hash40::new("top"), 0.0, -10.0, 0.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 3, Hash40::new("top"), 0.0, -13.0, 1.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 4, Hash40::new("top"), 0.0, 2.0, -3.0);
	}
	frame(fighter.lua_state_agent, 17.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 60, 90, 0, 60, 3.0, 0.0, 1.0, -11.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 12.0, 60, 90, 0, 60, 2.4, 0.0, -2.0, -13.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 12.0, 60, 90, 0, 60, 2.4, 0.0, -5.0, -14.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 12.0, 60, 90, 0, 60, 2.4, 0.0, -8.0, -15.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 4, 0, Hash40::new("top"), 12.0, 60, 90, 0, 60, 3.0, 0.0, 4.0, -7.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
	}
	frame(fighter.lua_state_agent, 18.0);
	if macros::is_excute(fighter) {
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 0, Hash40::new("top"), 0.0, 6.0, -13.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 1, Hash40::new("top"), 0.0, 5.0, -16.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 2, Hash40::new("top"), 0.0, 4.0, -19.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 3, Hash40::new("top"), 0.0, 3.0, -22.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 4, Hash40::new("top"), 0.0, 8.0, -8.0);
	}
	frame(fighter.lua_state_agent, 19.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	frame(fighter.lua_state_agent, 26.0);
	if macros::is_excute(fighter) {
		if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD) == true {
			ArticleModule::add_motion_partial(fighter.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, *WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_close"), 5.0, 5.0, false, false, 0.0, false, true, false);
		}
		if MotionModule::is_changing(fighter.module_accessor) == true {
			WorkModule::on_flag(fighter.module_accessor, *FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
		}
	}
	frame(fighter.lua_state_agent, 33.0);
	if macros::is_excute(fighter) {
		WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	frame(fighter.lua_state_agent, 72.0);
	if macros::is_excute(fighter) {
		notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
	}
}

#[acmd_script(agent = "eflame", script = "game_attackairn", category = ACMD_GAME)]
unsafe fn eflame_game_attackairn(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 4.0);
	frame(fighter.lua_state_agent, 2.0);
	macros::FT_MOTION_RATE(fighter, 2.0);
	frame(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD) == true {
			ArticleModule::add_motion_partial(fighter.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, *WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 5.0, 5.0, false, false, 0.0, false, true, false);
		}
		if MotionModule::is_changing(fighter.module_accessor) == true {
			WorkModule::on_flag(fighter.module_accessor, *FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
		}
	}
	macros::FT_MOTION_RATE(fighter, 1.0);
	frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 10.0, 75, 60, 0, 105, 6.5, -4.0, 5.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 1, 0, Hash40::new("haver"), 10.0, 75, 60, 0, 105, 5.0, -4.0, 12.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
	}
	frame(fighter.lua_state_agent, 12.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 8.0, 75, 60, 0, 100, 6.5, -4.0, 5.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 1, 0, Hash40::new("haver"), 8.0, 75, 60, 0, 100, 5.0, -4.0, 12.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
	}
	frame(fighter.lua_state_agent, 18.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	frame(fighter.lua_state_agent, 23.0);
	if macros::is_excute(fighter) {
		if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD) == true {
			ArticleModule::add_motion_partial(fighter.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, *WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_close"), 5.0, 5.0, false, false, 0.0, false, true, false);
		}
		if MotionModule::is_changing(fighter.module_accessor) == true {
			WorkModule::on_flag(fighter.module_accessor, *FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
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

#[acmd_script(agent = "eflame", script = "game_attackdash", category = ACMD_GAME)]
unsafe fn eflame_game_attackdash(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 2.0);
	frame(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD) == true {
			ArticleModule::add_motion_partial(fighter.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, *WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 5.0, 5.0, false, false, 0.0, false, true, false);
		}
		if MotionModule::is_changing(fighter.module_accessor) == true {
			WorkModule::on_flag(fighter.module_accessor, *FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
		}
	}
	macros::FT_MOTION_RATE(fighter, 1.0);
	frame(fighter.lua_state_agent, 8.0);
	macros::FT_MOTION_RATE(fighter, 0.5);
	frame(fighter.lua_state_agent, 14.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 13.5, 361, 64, 0, 90, 5.0, 0.0, 7.0, 13.0, Some(0.0), Some(7.0), Some(20.0), 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 11.5, 361, 64, 0, 90, 5.0, 0.0, 7.0, 5.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
		macros::ATK_SET_SHIELD_SETOFF_MUL_arg3(fighter, 0, 1, 1.5);
	}
	frame(fighter.lua_state_agent, 16.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	frame(fighter.lua_state_agent, 24.0);
	if macros::is_excute(fighter) {
		if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD) == true {
			ArticleModule::add_motion_partial(fighter.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, *WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_close"), 5.0, 5.0, false, false, 0.0, false, true, false);
		}
		if MotionModule::is_changing(fighter.module_accessor) == true {
			WorkModule::on_flag(fighter.module_accessor, *FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
		}
	}
}

#[acmd_script(agent = "eflame", script = "game_attackhi3", category = ACMD_GAME)]
unsafe fn eflame_game_attackhi3(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 2.0);
	frame(fighter.lua_state_agent, 2.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	frame(fighter.lua_state_agent, 7.0);
	if macros::is_excute(fighter) {
		if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD) == true {
			ArticleModule::add_motion_partial(fighter.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, *WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 5.0, 5.0, false, false, 0.0, false, true, false);
		}
		if MotionModule::is_changing(fighter.module_accessor) == true {
			WorkModule::on_flag(fighter.module_accessor, *FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
		}
	}
	frame(fighter.lua_state_agent, 9.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 95, 94, 0, 70, 2.6, 0.0, 17.0, -7.5, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 95, 94, 0, 70, 2.4, 0.0, 15.0, -11.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 10.0, 95, 94, 0, 70, 2.0, 0.0, 13.0, -14.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 10.0, 95, 94, 0, 70, 1.5, 0.0, 12.0, -16.5, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 4, 0, Hash40::new("top"), 10.0, 95, 94, 0, 70, 2.2, 0.0, 18.0, -3.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
	}
	frame(fighter.lua_state_agent, 10.0);
	if macros::is_excute(fighter) {
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 0, Hash40::new("top"), 0.0, 21.0, -4.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 1, Hash40::new("top"), 0.0, 23.0, -8.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 2, Hash40::new("top"), 0.0, 25.0, -11.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 3, Hash40::new("top"), 0.0, 26.0, -13.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 4, Hash40::new("top"), 0.0, 19.0, 0.0);
	}
	frame(fighter.lua_state_agent, 11.0);
	if macros::is_excute(fighter) {
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 0, Hash40::new("top"), 0.0, 22.0, 3.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 1, Hash40::new("top"), 0.0, 25.0, 2.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 2, Hash40::new("top"), 0.0, 28.0, 1.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 3, Hash40::new("top"), 0.0, 31.0, 0.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 4, Hash40::new("top"), 0.0, 17.0, 4.0);
	}
	frame(fighter.lua_state_agent, 12.0);
	if macros::is_excute(fighter) {
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 0, Hash40::new("top"), 0.0, 17.0, 7.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 1, Hash40::new("top"), 0.0, 20.0, 8.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 2, Hash40::new("top"), 0.0, 24.0, 9.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 3, Hash40::new("top"), 0.0, 26.0, 10.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 4, Hash40::new("top"), 0.0, 12.0, 5.0);
	}
	frame(fighter.lua_state_agent, 13.0);
	if macros::is_excute(fighter) {
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 0, Hash40::new("top"), 0.0, 11.0, 10.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 1, Hash40::new("top"), 0.0, 14.0, 12.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 2, Hash40::new("top"), 0.0, 16.0, 14.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 3, Hash40::new("top"), 0.0, 18.0, 16.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 4, Hash40::new("top"), 0.0, 9.0, 5.0);
	}
	frame(fighter.lua_state_agent, 14.0);
	if macros::is_excute(fighter) {
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 0, Hash40::new("top"), 0.0, 5.0, 7.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 1, Hash40::new("top"), 0.0, 5.0, 11.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 2, Hash40::new("top"), 0.0, 5.0, 14.5);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 3, Hash40::new("top"), 0.0, 5.0, 17.5);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 4, Hash40::new("top"), 0.0, 5.0, 5.0);
	}
	frame(fighter.lua_state_agent, 15.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	frame(fighter.lua_state_agent, 37.0);
	if macros::is_excute(fighter) {
		if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD) == true {
			ArticleModule::add_motion_partial(fighter.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, *WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_close"), 5.0, 5.0, false, false, 0.0, false, true, false);
		}
		if MotionModule::is_changing(fighter.module_accessor) == true {
			WorkModule::on_flag(fighter.module_accessor, *FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
		}
	}
}

#[acmd_script(agent = "eflame", script = "game_attackhi4", category = ACMD_GAME)]
unsafe fn eflame_game_attackhi4(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 7.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
		WorkModule::set_int64(fighter.module_accessor, hash40("attack_hi4_hold") as i64, *FIGHTER_EFLAME_INSTANCE_WORK_ID_INT_ESWORD_INHERIT_OPEN_MOTION_KIND);
		if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD) == true {
			ArticleModule::add_motion_partial(fighter.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, *WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 5.0, 5.0, false, false, 0.0, false, true, false);
		}
		if MotionModule::is_changing(fighter.module_accessor) == true {
			WorkModule::on_flag(fighter.module_accessor, *FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
		}
	}
	frame(fighter.lua_state_agent, 8.0);
	macros::FT_MOTION_RATE(fighter, 2.0);
	frame(fighter.lua_state_agent, 10.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	frame(fighter.lua_state_agent, 13.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.5, 368, 100, 0, 0, 8.0, 0.0, 8.0, 8.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
		AttackModule::set_vec_target_pos(fighter.module_accessor, 0, Hash40::new("top"), &Vector2f{x: 0.0, y: 23.0}, 7.0 as u32, false);
	}
	frame(fighter.lua_state_agent, 14.0);
	if macros::is_excute(fighter) {
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 0, Hash40::new("top"), 0.0, 12.0, 8.0);
	}
	frame(fighter.lua_state_agent, 15.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 1, Hash40::new("top"), 13.5, 85, 85, 0, 80, 5.0, 0.0, 22.0, 3.0, Some(0.0), Some(22.0), Some(11.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
	}
	frame(fighter.lua_state_agent, 16.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 1, Hash40::new("top"), 13.5, 85, 85, 0, 80, 5.0, 0.0, 24.0, 0.0, Some(0.0), Some(24.0), Some(8.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
	}
	frame(fighter.lua_state_agent, 17.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 1, Hash40::new("top"), 13.5, 85, 85, 0, 80, 5.0, 0.0, 24.0, -3.0, Some(0.0), Some(24.0), Some(8.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
	}
	frame(fighter.lua_state_agent, 18.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 1, Hash40::new("top"), 13.5, 85, 85, 0, 80, 5.0, 0.0, 24.0, -8.0, Some(0.0), Some(24.0), Some(8.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
	}
	frame(fighter.lua_state_agent, 19.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 1, Hash40::new("top"), 13.5, 85, 85, 0, 80, 5.0, 0.0, 24.0, -11.0, Some(0.0), Some(24.0), Some(8.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
	}
	frame(fighter.lua_state_agent, 22.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 1, Hash40::new("top"), 12.0, 85, 80, 0, 80, 5.0, 0.0, 24.0, -11.0, Some(0.0), Some(24.0), Some(8.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
	}
	frame(fighter.lua_state_agent, 32.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	frame(fighter.lua_state_agent, 38.0);
	if macros::is_excute(fighter) {
		if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD) == true {
			ArticleModule::add_motion_partial(fighter.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, *WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_close"), 5.0, 5.0, false, false, 0.0, false, true, false);
		}
		if MotionModule::is_changing(fighter.module_accessor) == true {
			WorkModule::on_flag(fighter.module_accessor, *FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
		}
	}
	frame(fighter.lua_state_agent, 50.0);
	macros::FT_MOTION_RATE(fighter, 1.1);
	frame(fighter.lua_state_agent, 60.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
}

#[acmd_script(agent = "eflame", script = "game_attacklw3", category = ACMD_GAME)]
unsafe fn eflame_game_attacklw3(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 2.0);
	frame(fighter.lua_state_agent, 2.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	frame(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD) == true {
			ArticleModule::add_motion_partial(fighter.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, *WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 5.0, 5.0, false, false, 0.0, false, true, false);
		}
		if MotionModule::is_changing(fighter.module_accessor) == true {
			WorkModule::on_flag(fighter.module_accessor, *FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
		}
	}
	frame(fighter.lua_state_agent, 7.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 85, 91, 0, 70, 5.0, 0.0, 4.0, 7.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 8.0, 85, 91, 0, 70, 3.5, 0.0, 3.0, 13.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 8.0, 95, 91, 0, 70, 3.0, 0.0, 2.0, 17.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
	}
	frame(fighter.lua_state_agent, 10.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	frame(fighter.lua_state_agent, 15.0);
	if macros::is_excute(fighter) {
		if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD) == true {
			ArticleModule::add_motion_partial(fighter.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, *WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_close"), 5.0, 5.0, false, false, 0.0, false, true, false);
		}
		if MotionModule::is_changing(fighter.module_accessor) == true {
			WorkModule::on_flag(fighter.module_accessor, *FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
		}
	}
	macros::FT_MOTION_RATE(fighter, 0.75);
	frame(fighter.lua_state_agent, 27.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
}

#[acmd_script(agent = "eflame", script = "game_attacklw4", category = ACMD_GAME)]
unsafe fn eflame_game_attacklw4(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
		WorkModule::set_int64(fighter.module_accessor, hash40("attack_lw4_hold") as i64, *FIGHTER_EFLAME_INSTANCE_WORK_ID_INT_ESWORD_INHERIT_OPEN_MOTION_KIND);
		if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD) == true {
			ArticleModule::add_motion_partial(fighter.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, *WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 5.0, 5.0, false, false, 0.0, false, true, false);
		}
		if MotionModule::is_changing(fighter.module_accessor) == true {
			WorkModule::on_flag(fighter.module_accessor, *FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
		}
	}
	macros::FT_MOTION_RATE(fighter, 3.0);
	frame(fighter.lua_state_agent, 4.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	frame(fighter.lua_state_agent, 10.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 35, 73, 0, 80, 3.0, 0.0, 7.0, 3.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 13.5, 35, 72, 0, 80, 4.0, 0.0, 7.0, 16.0, Some(0.0), Some(7.0), Some(10.0), 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 8.0, 35, 66, 0, 80, 3.0, 0.0, 7.0, -4.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 10.0, 35, 65, 0, 80, 3.0, 0.0, 7.0, -12.0, Some(0.0), Some(7.0), Some(-10.0), 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	frame(fighter.lua_state_agent, 13.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	frame(fighter.lua_state_agent, 16.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 35, 73, 0, 80, 3.0, 0.0, 7.0, -3.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 13.5, 35, 72, 0, 80, 4.0, 0.0, 7.0, -16.0, Some(0.0), Some(7.0), Some(-10.0), 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 8.0, 35, 66, 0, 80, 3.0, 0.0, 7.0, 4.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 10.0, 35, 65, 0, 80, 3.0, 0.0, 7.0, 12.0, Some(0.0), Some(7.0), Some(10.0), 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	frame(fighter.lua_state_agent, 19.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	frame(fighter.lua_state_agent, 45.0);
	if macros::is_excute(fighter) {
		if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD) == true {
			ArticleModule::add_motion_partial(fighter.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, *WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_close"), 5.0, 5.0, false, false, 0.0, false, true, false);
		}
		if MotionModule::is_changing(fighter.module_accessor) == true {
			WorkModule::on_flag(fighter.module_accessor, *FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
		}
	}
}

#[acmd_script(agent = "eflame", script = "game_attacks3", category = ACMD_GAME)]
unsafe fn eflame_game_attacks3(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 2.0);
	frame(fighter.lua_state_agent, 2.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	frame(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD) == true {
			ArticleModule::add_motion_partial(fighter.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, *WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 5.0, 5.0, false, false, 0.0, false, true, false);
		}
		if MotionModule::is_changing(fighter.module_accessor) == true {
			WorkModule::on_flag(fighter.module_accessor, *FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
		}
	}
	frame(fighter.lua_state_agent, 10.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 13.0, 35, 72, 0, 70, 3.0, -1.5, 4.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 1, 0, Hash40::new("haver"), 13.0, 35, 72, 0, 70, 3.0, -1.5, 8.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 2, 0, Hash40::new("haver"), 13.0, 35, 72, 0, 70, 2.5, -1.5, 12.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 3, 0, Hash40::new("haver"), 13.0, 35, 72, 0, 70, 2.0, -1.5, 16.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 4, 0, Hash40::new("top"), 13.0, 35, 72, 0, 70, 3.0, 0.0, 10.0, 6.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
		macros::ATK_SET_SHIELD_SETOFF_MUL_arg5(fighter, 0, 1, 2, 3, 1.5);
		macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 4, 1.5);
	}
	frame(fighter.lua_state_agent, 11.0);
	if macros::is_excute(fighter) {
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 0, Hash40::new("haver"), 2.0, 6.0, 0.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 1, Hash40::new("haver"), 2.0, 10.0, 0.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 2, Hash40::new("haver"), 2.0, 13.0, 0.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 3, Hash40::new("haver"), 2.0, 17.0, 0.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 4, Hash40::new("top"), 0.0, 5.0, 6.0);
		AttackModule::set_size(fighter.module_accessor, 0, 3.5);
		AttackModule::set_size(fighter.module_accessor, 1, 3.5);
		AttackModule::set_size(fighter.module_accessor, 2, 3.0);
		AttackModule::set_size(fighter.module_accessor, 3, 2.5);
	}
	frame(fighter.lua_state_agent, 12.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	frame(fighter.lua_state_agent, 26.0);
	if macros::is_excute(fighter) {
		if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD) == true {
			ArticleModule::add_motion_partial(fighter.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, *WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_close"), 5.0, 5.0, false, false, 0.0, false, true, false);
		}
		if MotionModule::is_changing(fighter.module_accessor) == true {
			WorkModule::on_flag(fighter.module_accessor, *FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
		}
	}
}

#[acmd_script(agent = "eflame", script = "game_attacks4", category = ACMD_GAME)]
unsafe fn eflame_game_attacks4(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
		WorkModule::set_int64(fighter.module_accessor, hash40("attack_s4_hold") as i64, *FIGHTER_EFLAME_INSTANCE_WORK_ID_INT_ESWORD_INHERIT_OPEN_MOTION_KIND);
		if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD) == true {
			ArticleModule::add_motion_partial(fighter.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, *WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 5.0, 5.0, false, false, 0.0, false, true, false);
		}
		if MotionModule::is_changing(fighter.module_accessor) == true {
			WorkModule::on_flag(fighter.module_accessor, *FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
		}
	}
	frame(fighter.lua_state_agent, 10.0);
	macros::FT_MOTION_RATE(fighter, 2.0);
	frame(fighter.lua_state_agent, 12.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	frame(fighter.lua_state_agent, 18.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 20.0, 361, 80, 0, 70, 3.5, 0.0, 12.0, 8.0, None, None, None, 1.35, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 20.0, 361, 80, 0, 70, 3.0, 0.0, 17.0, 10.0, None, None, None, 1.35, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 20.0, 361, 80, 0, 70, 2.5, 0.0, 19.0, 11.5, None, None, None, 1.35, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 20.0, 361, 80, 0, 70, 2.0, 0.0, 19.0, 11.5, None, None, None, 1.35, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, 4, 0, Hash40::new("top"), 20.0, 361, 80, 0, 70, 3.0, 0.0, 8.0, 5.5, None, None, None, 1.35, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
		AttackModule::set_optional_hit_effect(fighter.module_accessor, 0, Hash40::new("eflame_attack_s4s_explosion"));
		AttackModule::set_optional_hit_effect(fighter.module_accessor, 1, Hash40::new("eflame_attack_s4s_explosion"));
		AttackModule::set_optional_hit_effect(fighter.module_accessor, 2, Hash40::new("eflame_attack_s4s_explosion"));
		AttackModule::set_optional_hit_effect(fighter.module_accessor, 3, Hash40::new("eflame_attack_s4s_explosion"));
		AttackModule::set_optional_hit_effect(fighter.module_accessor, 4, Hash40::new("eflame_attack_s4s_explosion"));
	}
	frame(fighter.lua_state_agent, 19.0);
	if macros::is_excute(fighter) {
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 0, Hash40::new("top"), 0.0, 7.0, 12.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 1, Hash40::new("top"), 0.0, 9.0, 16.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 2, Hash40::new("top"), 0.0, 11.0, 20.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 3, Hash40::new("top"), 0.0, 13.0, 23.5);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 4, Hash40::new("top"), 0.0, 5.0, 7.0);
	}
	frame(fighter.lua_state_agent, 20.0);
	if macros::is_excute(fighter) {
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 0, Hash40::new("top"), 0.0, 4.0, 13.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 1, Hash40::new("top"), 0.0, 4.0, 20.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 2, Hash40::new("top"), 0.0, 2.0, 25.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 3, Hash40::new("top"), 0.0, 7.0, 27.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 4, Hash40::new("top"), 0.0, 2.0, 5.5);
		AttackModule::set_size(fighter.module_accessor, 0, 5.0);
		AttackModule::set_size(fighter.module_accessor, 1, 4.5);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_EFLAME_STATUS_ATTACK_FLAG_S4_GROUND_CHECK);
	}
	frame(fighter.lua_state_agent, 22.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	frame(fighter.lua_state_agent, 42.0);
	if macros::is_excute(fighter) {
		if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD) == true {
			ArticleModule::add_motion_partial(fighter.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, *WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_close"), 5.0, 5.0, false, false, 0.0, false, true, false);
		}
		if MotionModule::is_changing(fighter.module_accessor) == true {
			WorkModule::on_flag(fighter.module_accessor, *FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
		}
	}
}

#[acmd_script(agent = "eflame", script = "game_catch", category = ACMD_GAME)]
unsafe fn eflame_game_catch(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter,	2.0);
	frame(fighter.lua_state_agent, 2.0);
	macros::FT_MOTION_RATE(fighter,	1.0);
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

#[acmd_script(agent = "eflame", script = "game_catchattack", category = ACMD_GAME)]
unsafe fn eflame_game_catchattack(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.6, 361, 100, 30, 0, 5.0, 0.0, 10.0, 10.0, None, None, None, 2.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
		AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
	}
	wait(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

#[acmd_script(agent = "eflame", script = "game_catchdash", category = ACMD_GAME)]
unsafe fn eflame_game_catchdash(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter,	0.5);
	frame(fighter.lua_state_agent, 7.0);
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

#[acmd_script(agent = "eflame", script = "game_catchturn", category = ACMD_GAME)]
unsafe fn eflame_game_catchturn(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter,	0.5);
	frame(fighter.lua_state_agent, 7.0);
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

#[acmd_script(agent = "eflame", script = "game_throwb", category = ACMD_GAME)]
unsafe fn eflame_game_throwb(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 4.0, 45, 95, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
	}
	frame(fighter.lua_state_agent, 15.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 45, 135, 0, 40, 5.0, 0.0, 13.0, -11.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
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

#[acmd_script(agent = "eflame", script = "game_throwf", category = ACMD_GAME)]
unsafe fn eflame_game_throwf(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 5.0, 35, 95, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
	}
	frame(fighter.lua_state_agent, 10.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 35, 140, 0, 40, 6.0, 0.0, 11.0, 12.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
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

#[acmd_script(agent = "eflame", script = "game_throwhi", category = ACMD_GAME)]
unsafe fn eflame_game_throwhi(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 6.0, 90, 75, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
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

#[acmd_script(agent = "eflame", script = "game_throwlw", category = ACMD_GAME)]
unsafe fn eflame_game_throwlw(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		macros::FT_LEAVE_NEAR_OTTOTTO(fighter, -2.5, 2.5);
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 5.0, 80, 105, 0, 80, 0.5, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.5, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
	}
	frame(fighter.lua_state_agent, 10.0);
	if macros::is_excute(fighter) {
		if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD) == true {
			ArticleModule::add_motion_partial(fighter.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, *WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 3.33, 3.33, false, false, 0.0, false, true, false);
		}
		if MotionModule::is_changing(fighter.module_accessor) == true {
			WorkModule::on_flag(fighter.module_accessor, *FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
		}
	}
	frame(fighter.lua_state_agent, 20.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 80, 145, 0, 40, 4.5, 0.0, 3.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
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
	frame(fighter.lua_state_agent, 66.0);
	if macros::is_excute(fighter) {
		if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD) == true {
			ArticleModule::add_motion_partial(fighter.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, *WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_close"), 3.33, 3.33, false, false, 0.0, false, true, false);
		}
		if MotionModule::is_changing(fighter.module_accessor) == true {
			WorkModule::on_flag(fighter.module_accessor, *FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
		}
	}
}

#[acmd_script(agent = "eflame", script = "sound_attack100end", category = ACMD_SOUND)]
unsafe fn eflame_sound_attack100end(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_eflame_attack100_end"));
		macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_eflame_rnd_attack01"));
	}
}

#[acmd_script(agent = "eflame", script = "sound_deathscytheswing4", category = ACMD_SOUND)]
unsafe fn eflame_sound_deathscytheswing4(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 8.0);
	if macros::is_excute(fighter) {
		macros::STOP_SE(fighter, Hash40::new("se_common_smash_start_02"));
	}
	wait(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("vc_eflame_attack07"));
	}
}

#[acmd_script(agent = "eflame", script = "sound_deathscytheswing4charge", category = ACMD_SOUND)]
unsafe fn eflame_sound_deathscytheswing4charge(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_common_smash_start_02"));
	}
}

#[acmd_script(agent = "eflame_firepillar", script = "game_specialhi", category = ACMD_GAME)]
unsafe fn eflame_firepillar_game_specialhi(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 60, 100, 0, 100, 11.0, 0.0, 8.0, 2.0, Some(0.0), Some(8.0), Some(-2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 9.0, 60, 100, 0, 100, 11.0, 0.0, 25.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
	}
	frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 60, 100, 0, 90, 11.0, 0.0, 8.0, 2.0, Some(0.0), Some(8.0), Some(-2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 60, 100, 0, 90, 11.0, 0.0, 25.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
		macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 4.0, 60, 100, 0, 80, 7.0, 0.0, 40.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
	}
	frame(fighter.lua_state_agent, 11.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	frame(fighter.lua_state_agent, 13.0);
	if macros::is_excute(fighter) {
		notify_event_msc_cmd!(fighter, Hash40::new_raw(0x199c462b5d));
	}
}

pub fn install() {
	smashline::install_acmd_scripts!(
		eflame_game_attack11,
		eflame_game_attack12,
		eflame_game_attack13,
		eflame_game_attack100,
		eflame_game_attack100end,
		eflame_game_attack100start,
		eflame_game_attack100sub,
		eflame_game_attackairb,
		eflame_game_attackairf,
		eflame_game_attackairhi,
		eflame_game_attackairlw,
		eflame_game_attackairn,
		eflame_game_attackdash,
		eflame_game_attackhi3,
		eflame_game_attackhi4,
		eflame_game_attacklw3,
		eflame_game_attacklw4,
		eflame_game_attacks3,
		eflame_game_attacks4,
		eflame_game_catch,
		eflame_game_catchattack,
		eflame_game_catchdash,
		eflame_game_catchturn,
		eflame_game_throwb,
		eflame_game_throwf,
		eflame_game_throwhi,
		eflame_game_throwlw,
		eflame_sound_attack100end,
		eflame_sound_deathscytheswing4,
		eflame_sound_deathscytheswing4charge,
		eflame_firepillar_game_specialhi,
	);
}
