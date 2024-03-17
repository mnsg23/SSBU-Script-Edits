use {
	smash::{
		app::sv_animcmd,
		lua2cpp::L2CAgentBase,
		phx::Hash40
	},
	smash_script::*,
	smashline::*
};

unsafe extern "C" fn diddy_sound_batswing4(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 45.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("vc_diddy_attack07"));
	}
}

unsafe extern "C" fn diddy_sound_itemswing4(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 8.0);
	if macros::is_excute(fighter) {
		macros::STOP_SE(fighter, Hash40::new("se_common_smash_start_04"));
	}
	sv_animcmd::wait(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("couple_stage_roof"));
	}
}

unsafe extern "C" fn diddy_sound_win1(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 13.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE_NO_3D(fighter, Hash40::new("vc_diddy_win01"));
	}
	sv_animcmd::frame(fighter.lua_state_agent, 14.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_diddy_special_n01_win01"));
	}
	sv_animcmd::frame(fighter.lua_state_agent, 30.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE_NO_3D(fighter, Hash40::new("vc_diddy_win01"));
	}
	sv_animcmd::frame(fighter.lua_state_agent, 39.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE_NO_3D(fighter, Hash40::new("se_diddy_special_n01_win01"));
	}
	sv_animcmd::frame(fighter.lua_state_agent, 43.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE_NO_3D(fighter, Hash40::new("vc_diddy_win01"));
	}
	sv_animcmd::frame(fighter.lua_state_agent, 63.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE_NO_3D(fighter, Hash40::new("se_diddy_special_n01_win01"));
	}
	sv_animcmd::frame(fighter.lua_state_agent, 92.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE_NO_3D(fighter, Hash40::new("se_diddy_squat_win01"));
	}
	sv_animcmd::frame(fighter.lua_state_agent, 121.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE_NO_3D(fighter, Hash40::new("se_diddy_special_n01_win01"));
	}
	sv_animcmd::frame(fighter.lua_state_agent, 125.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE_NO_3D(fighter, Hash40::new("vc_diddy_win01"));
	}
	sv_animcmd::frame(fighter.lua_state_agent, 138.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE_NO_3D(fighter, Hash40::new("vc_diddy_001"));
	}
}

pub fn install() {
	Agent::new("diddy")
		.sound_acmd("sound_batswing4", diddy_sound_batswing4)
		.sound_acmd("sound_lipstickswing4", diddy_sound_itemswing4)
		.sound_acmd("sound_starrodswing4", diddy_sound_itemswing4)
		.sound_acmd("sound_win1", diddy_sound_win1)
		.install();
}
