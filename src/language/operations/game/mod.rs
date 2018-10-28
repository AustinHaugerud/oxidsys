use language::operations::Operation;
pub mod auto_save;
pub mod get_achievement_stat;
pub mod get_average_game_difficulty;
pub mod get_operation_set_version;
pub mod get_player_agent_kill_count;
pub mod get_player_agent_own_troop_kill_count;
pub mod is_edit_mode_enabled;
pub mod is_trial_version;
pub mod options_get_battle_size;
pub mod options_get_campaign_ai;
pub mod options_get_combat_ai;
pub mod options_get_combat_speed;
pub mod options_get_damage_to_friends;
pub mod options_get_damage_to_player;
pub mod options_set_battle_size;
pub mod options_set_campaign_ai;
pub mod options_set_combat_ai;
pub mod options_set_combat_speed;
pub mod options_set_damage_to_friends;
pub mod options_set_damage_to_player;
pub mod set_achievement_stat;
pub mod set_player_troop;
pub mod show_object_details_overlay;
pub mod unlock_achievement;

pub fn load_operands() -> Vec<Box<Operation>> {
    let mut result: Vec<Box<Operation>> = vec![];
    result.push(Box::new(auto_save::AutoSaveOp {}));
    result.push(Box::new(get_achievement_stat::GetAchievementStatOp {}));
    result.push(Box::new(
        get_average_game_difficulty::GetAverageGameDifficultyOp {},
    ));
    result.push(Box::new(
        get_operation_set_version::GetOperationSetVersionOp {},
    ));
    result.push(Box::new(
        get_player_agent_kill_count::GetPlayerAgentKillCountOp {},
    ));
    result.push(Box::new(
        get_player_agent_own_troop_kill_count::GetPlayerAgentOwnTroopKillCountOp {},
    ));
    result.push(Box::new(is_edit_mode_enabled::IsEditModeEnabledOp {}));
    result.push(Box::new(is_trial_version::IsTrialVersionOp {}));
    result.push(Box::new(options_get_battle_size::OptionsGetBattleSizeOp {}));
    result.push(Box::new(options_get_campaign_ai::OptionsGetCampaignAiOp {}));
    result.push(Box::new(options_get_combat_ai::OptionsGetCombatAiOp {}));
    result.push(Box::new(
        options_get_combat_speed::OptionsGetCombatSpeedOp {},
    ));
    result.push(Box::new(
        options_get_damage_to_friends::OptionsGetDamageToFriendsOp {},
    ));
    result.push(Box::new(
        options_get_damage_to_player::OptionsGetDamageToPlayerOp {},
    ));
    result.push(Box::new(options_set_battle_size::OptionsSetBattleSizeOp {}));
    result.push(Box::new(options_set_campaign_ai::OptionsSetCampaignAiOp {}));
    result.push(Box::new(options_set_combat_ai::OptionsSetCombatAiOp {}));
    result.push(Box::new(
        options_set_combat_speed::OptionsSetCombatSpeedOp {},
    ));
    result.push(Box::new(
        options_set_damage_to_friends::OptionsSetDamageToFriendsOp {},
    ));
    result.push(Box::new(
        options_set_damage_to_player::OptionsSetDamageToPlayerOp {},
    ));
    result.push(Box::new(set_achievement_stat::SetAchievementStatOp {}));
    result.push(Box::new(set_player_troop::SetPlayerTroopOp {}));
    result.push(Box::new(
        show_object_details_overlay::ShowObjectDetailsOverlayOp {},
    ));
    result.push(Box::new(unlock_achievement::UnlockAchievementOp {}));
    result
}
