use language::operations::Operation;
pub mod add_missile;
pub mod add_reinforcements_to_entry;
pub mod agent_add_offer_with_timeout;
pub mod agent_add_relation_with_agent;
pub mod agent_ai_get_behavior_target;
pub mod agent_ai_get_cached_enemy;
pub mod agent_ai_get_look_target;
pub mod agent_ai_get_move_target;
pub mod agent_ai_get_num_cached_enemies;
pub mod agent_ai_set_aggressiveness;
pub mod agent_ai_set_always_attack_in_melee;
pub mod agent_ai_set_can_crouch;
pub mod agent_ai_set_interact_with_player;
pub mod agent_check_offer_from_agent;
pub mod agent_clear_relations_with_agents;
pub mod agent_clear_scripted_mode;
pub mod agent_deliver_damage_to_agent;
pub mod agent_deliver_damage_to_agent_advanced;
pub mod agent_equip_item;
pub mod agent_fade_out;
pub mod agent_force_rethink;
pub mod agent_get_action_dir;
pub mod agent_get_ammo;
pub mod agent_get_ammo_for_slot;
pub mod agent_get_animation;
pub mod agent_get_attached_scene_prop;
pub mod agent_get_attack_action;
pub mod agent_get_bone_position;
pub mod agent_get_class;
pub mod agent_get_combat_state;
pub mod agent_get_crouch_mode;
pub mod agent_get_defend_action;
pub mod agent_get_division;
pub mod agent_get_entry_no;
pub mod agent_get_group;
pub mod agent_get_horse;
pub mod agent_get_item_cur_ammo;
pub mod agent_get_item_id;
pub mod agent_get_item_slot;
pub mod agent_get_kill_count;
pub mod agent_get_look_position;
pub mod agent_get_number_of_enemies_following;
pub mod agent_get_party_id;
pub mod agent_get_position;
pub mod agent_get_rider;
pub mod agent_get_scripted_destination;
pub mod agent_get_simple_behavior;
pub mod agent_get_slot;
pub mod agent_get_speed;
pub mod agent_get_team;
pub mod agent_get_time_elapsed_since_removed;
pub mod agent_get_troop_id;
pub mod agent_get_wielded_item;
pub mod agent_has_item_equipped;
pub mod agent_is_active;
pub mod agent_is_alarmed;
pub mod agent_is_alive;
pub mod agent_is_ally;
pub mod agent_is_defender;
pub mod agent_is_human;
pub mod agent_is_in_line_of_sight;
pub mod agent_is_in_parried_animation;
pub mod agent_is_in_special_mode;
pub mod agent_is_non_player;
pub mod agent_is_routed;
pub mod agent_is_wounded;
pub mod agent_play_sound;
pub mod agent_refill_ammo;
pub mod agent_refill_wielded_shield_hit_points;
pub mod agent_set_accuracy_modifier;
pub mod agent_set_ammo;
pub mod agent_set_animation;
pub mod agent_set_animation_progress;
pub mod agent_set_attached_scene_prop;
pub mod agent_set_attached_scene_prop_x;
pub mod agent_set_attached_scene_prop_y;
pub mod agent_set_attached_scene_prop_z;
pub mod agent_set_attack_action;
pub mod agent_set_crouch_mode;
pub mod agent_set_damage_modifier;
pub mod agent_set_defend_action;
pub mod agent_set_division;
pub mod agent_set_group;
pub mod agent_set_hit_points;
pub mod agent_set_horse_speed_factor;
pub mod agent_set_invulnerable_shield;
pub mod agent_set_is_alarmed;
pub mod agent_set_kick_allowed;
pub mod agent_set_look_target_agent;
pub mod agent_set_look_target_position;
pub mod agent_set_max_hit_points;
pub mod agent_set_no_death_knock_down_only;
pub mod agent_set_no_dynamics;
pub mod agent_set_position;
pub mod agent_set_ranged_damage_modifier;
pub mod agent_set_reload_speed_modifier;
pub mod agent_set_scripted_destination;
pub mod agent_set_scripted_destination_no_attack;
pub mod agent_set_slot;
pub mod agent_set_speed_limit;
pub mod agent_set_speed_modifier;
pub mod agent_set_stand_animation;
pub mod agent_set_team;
pub mod agent_set_use_speed_modifier;
pub mod agent_set_visibility;
pub mod agent_set_walk_forward_animation;
pub mod agent_set_wielded_item;
pub mod agent_slot_eq;
pub mod agent_slot_ge;
pub mod agent_start_running_away;
pub mod agent_stop_running_away;
pub mod agent_stop_sound;
pub mod agent_unequip_item;
pub mod class_is_listening_order;
pub mod get_player_agent_no;
pub mod remove_agent;
pub mod set_cheer_at_no_enemy;
pub mod set_spawn_position;
pub mod spawn_agent;
pub mod spawn_horse;
pub mod store_agent_hit_points;
pub mod store_ally_count;
pub mod store_attacker_count;
pub mod store_defender_count;
pub mod store_enemy_count;
pub mod store_friend_count;
pub mod store_normalized_team_count;
pub mod store_remaining_team_no;
pub mod team_get_gap_distance;
pub mod team_get_hold_fire_order;
pub mod team_get_leader;
pub mod team_get_movement_order;
pub mod team_get_order_position;
pub mod team_get_riding_order;
pub mod team_get_slot;
pub mod team_get_weapon_usage_order;
pub mod team_give_order;
pub mod team_set_leader;
pub mod team_set_order_listener;
pub mod team_set_order_position;
pub mod team_set_relation;
pub mod team_set_slot;
pub mod team_slot_eq;
pub mod team_slot_ge;
pub mod teams_are_enemies;

pub fn load_operands() -> Vec<Box<Operation>> {
    let mut result: Vec<Box<Operation>> = vec![];
    result.push(Box::new(add_missile::AddMissileOp {}));
    result.push(Box::new(
        add_reinforcements_to_entry::AddReinforcementsToEntryOp {},
    ));
    result.push(Box::new(
        agent_add_offer_with_timeout::AgentAddOfferWithTimeoutOp {},
    ));
    result.push(Box::new(
        agent_add_relation_with_agent::AgentAddRelationWithAgentOp {},
    ));
    result.push(Box::new(
        agent_ai_get_behavior_target::AgentAiGetBehaviorTargetOp {},
    ));
    result.push(Box::new(
        agent_ai_get_cached_enemy::AgentAiGetCachedEnemyOp {},
    ));
    result.push(Box::new(
        agent_ai_get_look_target::AgentAiGetLookTargetOp {},
    ));
    result.push(Box::new(
        agent_ai_get_move_target::AgentAiGetMoveTargetOp {},
    ));
    result.push(Box::new(
        agent_ai_get_num_cached_enemies::AgentAiGetNumCachedEnemiesOp {},
    ));
    result.push(Box::new(
        agent_ai_set_aggressiveness::AgentAiSetAggressivenessOp {},
    ));
    result.push(Box::new(
        agent_ai_set_always_attack_in_melee::AgentAiSetAlwaysAttackInMeleeOp {},
    ));
    result.push(Box::new(agent_ai_set_can_crouch::AgentAiSetCanCrouchOp {}));
    result.push(Box::new(
        agent_ai_set_interact_with_player::AgentAiSetInteractWithPlayerOp {},
    ));
    result.push(Box::new(
        agent_check_offer_from_agent::AgentCheckOfferFromAgentOp {},
    ));
    result.push(Box::new(
        agent_clear_relations_with_agents::AgentClearRelationsWithAgentsOp {},
    ));
    result.push(Box::new(
        agent_clear_scripted_mode::AgentClearScriptedModeOp {},
    ));
    result.push(Box::new(
        agent_deliver_damage_to_agent::AgentDeliverDamageToAgentOp {},
    ));
    result.push(Box::new(
        agent_deliver_damage_to_agent_advanced::AgentDeliverDamageToAgentAdvancedOp {},
    ));
    result.push(Box::new(agent_equip_item::AgentEquipItemOp {}));
    result.push(Box::new(agent_fade_out::AgentFadeOutOp {}));
    result.push(Box::new(agent_force_rethink::AgentForceRethinkOp {}));
    result.push(Box::new(agent_get_action_dir::AgentGetActionDirOp {}));
    result.push(Box::new(agent_get_ammo::AgentGetAmmoOp {}));
    result.push(Box::new(agent_get_ammo_for_slot::AgentGetAmmoForSlotOp {}));
    result.push(Box::new(agent_get_animation::AgentGetAnimationOp {}));
    result.push(Box::new(
        agent_get_attached_scene_prop::AgentGetAttachedScenePropOp {},
    ));
    result.push(Box::new(agent_get_attack_action::AgentGetAttackActionOp {}));
    result.push(Box::new(agent_get_bone_position::AgentGetBonePositionOp {}));
    result.push(Box::new(agent_get_class::AgentGetClassOp {}));
    result.push(Box::new(agent_get_combat_state::AgentGetCombatStateOp {}));
    result.push(Box::new(agent_get_crouch_mode::AgentGetCrouchModeOp {}));
    result.push(Box::new(agent_get_defend_action::AgentGetDefendActionOp {}));
    result.push(Box::new(agent_get_division::AgentGetDivisionOp {}));
    result.push(Box::new(agent_get_entry_no::AgentGetEntryNoOp {}));
    result.push(Box::new(agent_get_group::AgentGetGroupOp {}));
    result.push(Box::new(agent_get_horse::AgentGetHorseOp {}));
    result.push(Box::new(agent_get_item_cur_ammo::AgentGetItemCurAmmoOp {}));
    result.push(Box::new(agent_get_item_id::AgentGetItemIdOp {}));
    result.push(Box::new(agent_get_item_slot::AgentGetItemSlotOp {}));
    result.push(Box::new(agent_get_kill_count::AgentGetKillCountOp {}));
    result.push(Box::new(agent_get_look_position::AgentGetLookPositionOp {}));
    result.push(Box::new(
        agent_get_number_of_enemies_following::AgentGetNumberOfEnemiesFollowingOp {},
    ));
    result.push(Box::new(agent_get_party_id::AgentGetPartyIdOp {}));
    result.push(Box::new(agent_get_position::AgentGetPositionOp {}));
    result.push(Box::new(agent_get_rider::AgentGetRiderOp {}));
    result.push(Box::new(
        agent_get_scripted_destination::AgentGetScriptedDestinationOp {},
    ));
    result.push(Box::new(
        agent_get_simple_behavior::AgentGetSimpleBehaviorOp {},
    ));
    result.push(Box::new(agent_get_slot::AgentGetSlotOp {}));
    result.push(Box::new(agent_get_speed::AgentGetSpeedOp {}));
    result.push(Box::new(agent_get_team::AgentGetTeamOp {}));
    result.push(Box::new(
        agent_get_time_elapsed_since_removed::AgentGetTimeElapsedSinceRemovedOp {},
    ));
    result.push(Box::new(agent_get_troop_id::AgentGetTroopIdOp {}));
    result.push(Box::new(agent_get_wielded_item::AgentGetWieldedItemOp {}));
    result.push(Box::new(agent_has_item_equipped::AgentHasItemEquippedOp {}));
    result.push(Box::new(agent_is_active::AgentIsActiveOp {}));
    result.push(Box::new(agent_is_alarmed::AgentIsAlarmedOp {}));
    result.push(Box::new(agent_is_alive::AgentIsAliveOp {}));
    result.push(Box::new(agent_is_ally::AgentIsAllyOp {}));
    result.push(Box::new(agent_is_defender::AgentIsDefenderOp {}));
    result.push(Box::new(agent_is_human::AgentIsHumanOp {}));
    result.push(Box::new(
        agent_is_in_line_of_sight::AgentIsInLineOfSightOp {},
    ));
    result.push(Box::new(
        agent_is_in_parried_animation::AgentIsInParriedAnimationOp {},
    ));
    result.push(Box::new(
        agent_is_in_special_mode::AgentIsInSpecialModeOp {},
    ));
    result.push(Box::new(agent_is_non_player::AgentIsNonPlayerOp {}));
    result.push(Box::new(agent_is_routed::AgentIsRoutedOp {}));
    result.push(Box::new(agent_is_wounded::AgentIsWoundedOp {}));
    result.push(Box::new(agent_play_sound::AgentPlaySoundOp {}));
    result.push(Box::new(agent_refill_ammo::AgentRefillAmmoOp {}));
    result.push(Box::new(
        agent_refill_wielded_shield_hit_points::AgentRefillWieldedShieldHitPointsOp {},
    ));
    result.push(Box::new(
        agent_set_accuracy_modifier::AgentSetAccuracyModifierOp {},
    ));
    result.push(Box::new(agent_set_ammo::AgentSetAmmoOp {}));
    result.push(Box::new(agent_set_animation::AgentSetAnimationOp {}));
    result.push(Box::new(
        agent_set_animation_progress::AgentSetAnimationProgressOp {},
    ));
    result.push(Box::new(
        agent_set_attached_scene_prop::AgentSetAttachedScenePropOp {},
    ));
    result.push(Box::new(
        agent_set_attached_scene_prop_x::AgentSetAttachedScenePropXOp {},
    ));
    result.push(Box::new(
        agent_set_attached_scene_prop_y::AgentSetAttachedScenePropYOp {},
    ));
    result.push(Box::new(
        agent_set_attached_scene_prop_z::AgentSetAttachedScenePropZOp {},
    ));
    result.push(Box::new(agent_set_attack_action::AgentSetAttackActionOp {}));
    result.push(Box::new(agent_set_crouch_mode::AgentSetCrouchModeOp {}));
    result.push(Box::new(
        agent_set_damage_modifier::AgentSetDamageModifierOp {},
    ));
    result.push(Box::new(agent_set_defend_action::AgentSetDefendActionOp {}));
    result.push(Box::new(agent_set_division::AgentSetDivisionOp {}));
    result.push(Box::new(agent_set_group::AgentSetGroupOp {}));
    result.push(Box::new(agent_set_hit_points::AgentSetHitPointsOp {}));
    result.push(Box::new(
        agent_set_horse_speed_factor::AgentSetHorseSpeedFactorOp {},
    ));
    result.push(Box::new(
        agent_set_invulnerable_shield::AgentSetInvulnerableShieldOp {},
    ));
    result.push(Box::new(agent_set_is_alarmed::AgentSetIsAlarmedOp {}));
    result.push(Box::new(agent_set_kick_allowed::AgentSetKickAllowedOp {}));
    result.push(Box::new(
        agent_set_look_target_agent::AgentSetLookTargetAgentOp {},
    ));
    result.push(Box::new(
        agent_set_look_target_position::AgentSetLookTargetPositionOp {},
    ));
    result.push(Box::new(
        agent_set_max_hit_points::AgentSetMaxHitPointsOp {},
    ));
    result.push(Box::new(
        agent_set_no_death_knock_down_only::AgentSetNoDeathKnockDownOnlyOp {},
    ));
    result.push(Box::new(agent_set_no_dynamics::AgentSetNoDynamicsOp {}));
    result.push(Box::new(agent_set_position::AgentSetPositionOp {}));
    result.push(Box::new(
        agent_set_ranged_damage_modifier::AgentSetRangedDamageModifierOp {},
    ));
    result.push(Box::new(
        agent_set_reload_speed_modifier::AgentSetReloadSpeedModifierOp {},
    ));
    result.push(Box::new(
        agent_set_scripted_destination::AgentSetScriptedDestinationOp {},
    ));
    result.push(Box::new(
        agent_set_scripted_destination_no_attack::AgentSetScriptedDestinationNoAttackOp {},
    ));
    result.push(Box::new(agent_set_slot::AgentSetSlotOp {}));
    result.push(Box::new(agent_set_speed_limit::AgentSetSpeedLimitOp {}));
    result.push(Box::new(
        agent_set_speed_modifier::AgentSetSpeedModifierOp {},
    ));
    result.push(Box::new(
        agent_set_stand_animation::AgentSetStandAnimationOp {},
    ));
    result.push(Box::new(agent_set_team::AgentSetTeamOp {}));
    result.push(Box::new(
        agent_set_use_speed_modifier::AgentSetUseSpeedModifierOp {},
    ));
    result.push(Box::new(agent_set_visibility::AgentSetVisibilityOp {}));
    result.push(Box::new(
        agent_set_walk_forward_animation::AgentSetWalkForwardAnimationOp {},
    ));
    result.push(Box::new(agent_set_wielded_item::AgentSetWieldedItemOp {}));
    result.push(Box::new(agent_slot_eq::AgentSlotEqOp {}));
    result.push(Box::new(agent_slot_ge::AgentSlotGeOp {}));
    result.push(Box::new(
        agent_start_running_away::AgentStartRunningAwayOp {},
    ));
    result.push(Box::new(agent_stop_running_away::AgentStopRunningAwayOp {}));
    result.push(Box::new(agent_stop_sound::AgentStopSoundOp {}));
    result.push(Box::new(agent_unequip_item::AgentUnequipItemOp {}));
    result.push(Box::new(
        class_is_listening_order::ClassIsListeningOrderOp {},
    ));
    result.push(Box::new(get_player_agent_no::GetPlayerAgentNoOp {}));
    result.push(Box::new(remove_agent::RemoveAgentOp {}));
    result.push(Box::new(set_cheer_at_no_enemy::SetCheerAtNoEnemyOp {}));
    result.push(Box::new(set_spawn_position::SetSpawnPositionOp {}));
    result.push(Box::new(spawn_agent::SpawnAgentOp {}));
    result.push(Box::new(spawn_horse::SpawnHorseOp {}));
    result.push(Box::new(store_agent_hit_points::StoreAgentHitPointsOp {}));
    result.push(Box::new(store_ally_count::StoreAllyCountOp {}));
    result.push(Box::new(store_attacker_count::StoreAttackerCountOp {}));
    result.push(Box::new(store_defender_count::StoreDefenderCountOp {}));
    result.push(Box::new(store_enemy_count::StoreEnemyCountOp {}));
    result.push(Box::new(store_friend_count::StoreFriendCountOp {}));
    result.push(Box::new(
        store_normalized_team_count::StoreNormalizedTeamCountOp {},
    ));
    result.push(Box::new(store_remaining_team_no::StoreRemainingTeamNoOp {}));
    result.push(Box::new(teams_are_enemies::TeamsAreEnemiesOp {}));
    result.push(Box::new(team_get_gap_distance::TeamGetGapDistanceOp {}));
    result.push(Box::new(
        team_get_hold_fire_order::TeamGetHoldFireOrderOp {},
    ));
    result.push(Box::new(team_get_leader::TeamGetLeaderOp {}));
    result.push(Box::new(team_get_movement_order::TeamGetMovementOrderOp {}));
    result.push(Box::new(team_get_order_position::TeamGetOrderPositionOp {}));
    result.push(Box::new(team_get_riding_order::TeamGetRidingOrderOp {}));
    result.push(Box::new(team_get_slot::TeamGetSlotOp {}));
    result.push(Box::new(
        team_get_weapon_usage_order::TeamGetWeaponUsageOrderOp {},
    ));
    result.push(Box::new(team_give_order::TeamGiveOrderOp {}));
    result.push(Box::new(team_set_leader::TeamSetLeaderOp {}));
    result.push(Box::new(team_set_order_listener::TeamSetOrderListenerOp {}));
    result.push(Box::new(team_set_order_position::TeamSetOrderPositionOp {}));
    result.push(Box::new(team_set_relation::TeamSetRelationOp {}));
    result.push(Box::new(team_set_slot::TeamSetSlotOp {}));
    result.push(Box::new(team_slot_eq::TeamSlotEqOp {}));
    result.push(Box::new(team_slot_ge::TeamSlotGeOp {}));
    result
}
