use smash::hash40;
use smash::lua2cpp::L2CAgentBase;
use smashline::*;

#[acmd_script(agent = "eflame", script = "game_attack100start", category = ACMD_GAME)]
unsafe fn eflame_game_attack100start(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	acmd!(lua_state, {
		frame(Frame=1)
		FT_MOTION_RATE(FSM=0.375)
		frame(Frame=9)
		FT_MOTION_RATE(FSM=1)
	});
}

#[acmd_script(agent = "eflame", script = "sound_deathscytheswing4", category = ACMD_SOUND)]
unsafe fn eflame_sound_deathscytheswing4(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	acmd!(lua_state, {
		frame(Frame=8)
		if(is_excute){
			STOP_SE(hash40("se_common_smash_start_02"))
		}
		wait(Frames=5)
		if(is_excute){
			PLAY_SE(hash40("vc_eflame_attack07"))
		}
	});
}

#[acmd_script(agent = "eflame", script = "sound_deathscytheswing4charge", category = ACMD_SOUND)]
unsafe fn eflame_sound_deathscytheswing4charge(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	acmd!(lua_state, {
		if(is_excute){
			PLAY_SE(hash40("se_common_smash_start_02"))
		}
	});
}

pub fn install() {
	smashline::install_acmd_scripts!(
		eflame_game_attack100start,
		eflame_sound_deathscytheswing4,
		eflame_sound_deathscytheswing4charge,
	);
}
