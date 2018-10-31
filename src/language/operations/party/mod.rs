use language::operations::{make_param_doc, Operation, ParamInfo};
pub mod add_companion_party;
pub mod add_gold_to_party;
pub mod context_menu_add_item;
pub mod disable_party;
pub mod distribute_party_among_party_group;
pub mod get_party_ai_behavior;
pub mod get_party_ai_object;
pub mod heal_party;
pub mod hero_can_join;
pub mod hero_can_join_as_prisoner;
pub mod inflict_casualties_to_party_group;
pub mod main_party_has_troop;
pub mod party_add_leader;
pub mod party_add_members;
pub mod party_add_particle_system;
pub mod party_add_prisoners;
pub mod party_add_template;
pub mod party_add_xp;
pub mod party_add_xp_to_stack;
pub mod party_attach_to_party;
pub mod party_can_join;
pub mod party_can_join_as_prisoner;
pub mod party_can_join_party;
pub mod party_clear;
pub mod party_clear_particle_systems;
pub mod party_collect_attachments_to_party;
pub mod party_count_companions_of_type;
pub mod party_count_members_of_type;
pub mod party_count_prisoners_of_type;
pub mod party_detach;
pub mod party_end_battle;
pub mod party_force_add_members;
pub mod party_force_add_prisoners;
pub mod party_get_ai_initiative;
pub mod party_get_attached_party_with_rank;
pub mod party_get_attached_to;
pub mod party_get_battle_opponent;
pub mod party_get_cur_town;
pub mod party_get_current_terrain;
pub mod party_get_free_companions_capacity;
pub mod party_get_free_prisoners_capacity;
pub mod party_get_helpfulness;
pub mod party_get_icon;
pub mod party_get_ignore_with_player_party;
pub mod party_get_morale;
pub mod party_get_num_attached_parties;
pub mod party_get_num_companion_stacks;
pub mod party_get_num_companions;
pub mod party_get_num_prisoners;
pub mod party_get_position;
pub mod party_get_skill_level;
pub mod party_get_slot;
pub mod party_get_template_id;
pub mod party_ignore_player;
pub mod party_is_active;
pub mod party_is_in_any_town;
pub mod party_is_in_town;
pub mod party_join;
pub mod party_join_as_prisoner;
pub mod party_leave_cur_battle;
pub mod party_prisoner_stack_get_size;
pub mod party_prisoner_stack_get_troop_dna;
pub mod party_prisoner_stack_get_troop_id;
pub mod party_quick_attach_to_current_battle;
pub mod party_relocate_near_party;
pub mod party_remove_members;
pub mod party_remove_members_wounded_first;
pub mod party_remove_prisoners;
pub mod party_set_aggressiveness;
pub mod party_set_ai_behavior;
pub mod party_set_ai_initiative;
pub mod party_set_ai_object;
pub mod party_set_ai_patrol_radius;
pub mod party_set_ai_target_position;
pub mod party_set_bandit_attraction;
pub mod party_set_banner_icon;
pub mod party_set_courage;
pub mod party_set_extra_icon;
pub mod party_set_extra_text;
pub mod party_set_faction;
pub mod party_set_flags;
pub mod party_set_helpfulness;
pub mod party_set_icon;
pub mod party_set_ignore_with_player_party;
pub mod party_set_marshal;
pub mod party_set_morale;
pub mod party_set_name;
pub mod party_set_next_battle_simulation_time;
pub mod party_set_position;
pub mod party_set_slot;
pub mod party_slot_eq;
pub mod party_slot_ge;
pub mod party_stack_get_num_wounded;
pub mod party_stack_get_size;
pub mod party_stack_get_troop_dna;
pub mod party_stack_get_troop_id;
pub mod party_template_get_slot;
pub mod party_template_set_slot;
pub mod party_template_slot_eq;
pub mod party_template_slot_ge;
pub mod party_upgrade_with_xp;
pub mod party_wound_members;
pub mod remove_member_from_party;
pub mod remove_party;
pub mod remove_regular_prisoners;
pub mod remove_troops_from_companions;
pub mod remove_troops_from_prisoners;
pub mod set_camera_follow_party;
pub mod set_party_creation_random_limits;
pub mod set_spawn_radius;
pub mod spawn_around_party;
pub mod store01_random_parties_in_range;
pub mod store_distance_to_party_from_party;
pub mod store_faction_of_party;
pub mod store_num_parties_created;
pub mod store_num_parties_destroyed;
pub mod store_num_parties_destroyed_by_player;
pub mod store_num_parties_of_template;
pub mod store_num_regular_prisoners;
pub mod store_party_size;
pub mod store_party_size_wo_prisoners;
pub mod store_random_party_in_range;
pub mod store_random_party_of_template;
pub mod store_troop_count_companions;
pub mod store_troop_count_prisoners;
pub mod store_troop_kind_count;
pub mod troop_join;
pub mod troop_join_as_prisoner;
pub mod troops_can_join;
pub mod troops_can_join_as_prisoner;

pub fn load_operands() -> Vec<Box<Operation>> {
    let mut result: Vec<Box<Operation>> = vec![];
    result.push(Box::new(add_companion_party::AddCompanionPartyOp {}));
    result.push(Box::new(add_gold_to_party::AddGoldToPartyOp {}));
    result.push(Box::new(context_menu_add_item::ContextMenuAddItemOp {}));
    result.push(Box::new(disable_party::DisablePartyOp {}));
    result.push(Box::new(
        distribute_party_among_party_group::DistributePartyAmongPartyGroupOp {},
    ));
    result.push(Box::new(get_party_ai_behavior::GetPartyAiBehaviorOp {}));
    result.push(Box::new(get_party_ai_object::GetPartyAiObjectOp {}));
    result.push(Box::new(heal_party::HealPartyOp {}));
    result.push(Box::new(hero_can_join::HeroCanJoinOp {}));
    result.push(Box::new(
        hero_can_join_as_prisoner::HeroCanJoinAsPrisonerOp {},
    ));
    result.push(Box::new(
        inflict_casualties_to_party_group::InflictCasualtiesToPartyGroupOp {},
    ));
    result.push(Box::new(main_party_has_troop::MainPartyHasTroopOp {}));
    result.push(Box::new(party_add_leader::PartyAddLeaderOp {}));
    result.push(Box::new(party_add_members::PartyAddMembersOp {}));
    result.push(Box::new(
        party_add_particle_system::PartyAddParticleSystemOp {},
    ));
    result.push(Box::new(party_add_prisoners::PartyAddPrisonersOp {}));
    result.push(Box::new(party_add_template::PartyAddTemplateOp {}));
    result.push(Box::new(party_add_xp::PartyAddXpOp {}));
    result.push(Box::new(party_add_xp_to_stack::PartyAddXpToStackOp {}));
    result.push(Box::new(party_attach_to_party::PartyAttachToPartyOp {}));
    result.push(Box::new(party_can_join::PartyCanJoinOp {}));
    result.push(Box::new(
        party_can_join_as_prisoner::PartyCanJoinAsPrisonerOp {},
    ));
    result.push(Box::new(party_can_join_party::PartyCanJoinPartyOp {}));
    result.push(Box::new(party_clear::PartyClearOp {}));
    result.push(Box::new(
        party_clear_particle_systems::PartyClearParticleSystemsOp {},
    ));
    result.push(Box::new(
        party_collect_attachments_to_party::PartyCollectAttachmentsToPartyOp {},
    ));
    result.push(Box::new(
        party_count_companions_of_type::PartyCountCompanionsOfTypeOp {},
    ));
    result.push(Box::new(
        party_count_members_of_type::PartyCountMembersOfTypeOp {},
    ));
    result.push(Box::new(
        party_count_prisoners_of_type::PartyCountPrisonersOfTypeOp {},
    ));
    result.push(Box::new(party_detach::PartyDetachOp {}));
    result.push(Box::new(party_end_battle::PartyEndBattleOp {}));
    result.push(Box::new(party_force_add_members::PartyForceAddMembersOp {}));
    result.push(Box::new(
        party_force_add_prisoners::PartyForceAddPrisonersOp {},
    ));
    result.push(Box::new(party_get_ai_initiative::PartyGetAiInitiativeOp {}));
    result.push(Box::new(
        party_get_attached_party_with_rank::PartyGetAttachedPartyWithRankOp {},
    ));
    result.push(Box::new(party_get_attached_to::PartyGetAttachedToOp {}));
    result.push(Box::new(
        party_get_battle_opponent::PartyGetBattleOpponentOp {},
    ));
    result.push(Box::new(
        party_get_current_terrain::PartyGetCurrentTerrainOp {},
    ));
    result.push(Box::new(party_get_cur_town::PartyGetCurTownOp {}));
    result.push(Box::new(
        party_get_free_companions_capacity::PartyGetFreeCompanionsCapacityOp {},
    ));
    result.push(Box::new(
        party_get_free_prisoners_capacity::PartyGetFreePrisonersCapacityOp {},
    ));
    result.push(Box::new(party_get_helpfulness::PartyGetHelpfulnessOp {}));
    result.push(Box::new(party_get_icon::PartyGetIconOp {}));
    result.push(Box::new(
        party_get_ignore_with_player_party::PartyGetIgnoreWithPlayerPartyOp {},
    ));
    result.push(Box::new(party_get_morale::PartyGetMoraleOp {}));
    result.push(Box::new(
        party_get_num_attached_parties::PartyGetNumAttachedPartiesOp {},
    ));
    result.push(Box::new(
        party_get_num_companions::PartyGetNumCompanionsOp {},
    ));
    result.push(Box::new(
        party_get_num_companion_stacks::PartyGetNumCompanionStacksOp {},
    ));
    result.push(Box::new(party_get_num_prisoners::PartyGetNumPrisonersOp {}));
    result.push(Box::new(party_get_position::PartyGetPositionOp {}));
    result.push(Box::new(party_get_skill_level::PartyGetSkillLevelOp {}));
    result.push(Box::new(party_get_slot::PartyGetSlotOp {}));
    result.push(Box::new(party_get_template_id::PartyGetTemplateIdOp {}));
    result.push(Box::new(party_ignore_player::PartyIgnorePlayerOp {}));
    result.push(Box::new(party_is_active::PartyIsActiveOp {}));
    result.push(Box::new(party_is_in_any_town::PartyIsInAnyTownOp {}));
    result.push(Box::new(party_is_in_town::PartyIsInTownOp {}));
    result.push(Box::new(party_join::PartyJoinOp {}));
    result.push(Box::new(party_join_as_prisoner::PartyJoinAsPrisonerOp {}));
    result.push(Box::new(party_leave_cur_battle::PartyLeaveCurBattleOp {}));
    result.push(Box::new(
        party_prisoner_stack_get_size::PartyPrisonerStackGetSizeOp {},
    ));
    result.push(Box::new(
        party_prisoner_stack_get_troop_dna::PartyPrisonerStackGetTroopDnaOp {},
    ));
    result.push(Box::new(
        party_prisoner_stack_get_troop_id::PartyPrisonerStackGetTroopIdOp {},
    ));
    result.push(Box::new(
        party_quick_attach_to_current_battle::PartyQuickAttachToCurrentBattleOp {},
    ));
    result.push(Box::new(
        party_relocate_near_party::PartyRelocateNearPartyOp {},
    ));
    result.push(Box::new(party_remove_members::PartyRemoveMembersOp {}));
    result.push(Box::new(
        party_remove_members_wounded_first::PartyRemoveMembersWoundedFirstOp {},
    ));
    result.push(Box::new(party_remove_prisoners::PartyRemovePrisonersOp {}));
    result.push(Box::new(
        party_set_aggressiveness::PartySetAggressivenessOp {},
    ));
    result.push(Box::new(party_set_ai_behavior::PartySetAiBehaviorOp {}));
    result.push(Box::new(party_set_ai_initiative::PartySetAiInitiativeOp {}));
    result.push(Box::new(party_set_ai_object::PartySetAiObjectOp {}));
    result.push(Box::new(
        party_set_ai_patrol_radius::PartySetAiPatrolRadiusOp {},
    ));
    result.push(Box::new(
        party_set_ai_target_position::PartySetAiTargetPositionOp {},
    ));
    result.push(Box::new(
        party_set_bandit_attraction::PartySetBanditAttractionOp {},
    ));
    result.push(Box::new(party_set_banner_icon::PartySetBannerIconOp {}));
    result.push(Box::new(party_set_courage::PartySetCourageOp {}));
    result.push(Box::new(party_set_extra_icon::PartySetExtraIconOp {}));
    result.push(Box::new(party_set_extra_text::PartySetExtraTextOp {}));
    result.push(Box::new(party_set_faction::PartySetFactionOp {}));
    result.push(Box::new(party_set_flags::PartySetFlagsOp {}));
    result.push(Box::new(party_set_helpfulness::PartySetHelpfulnessOp {}));
    result.push(Box::new(party_set_icon::PartySetIconOp {}));
    result.push(Box::new(
        party_set_ignore_with_player_party::PartySetIgnoreWithPlayerPartyOp {},
    ));
    result.push(Box::new(party_set_marshal::PartySetMarshalOp {}));
    result.push(Box::new(party_set_morale::PartySetMoraleOp {}));
    result.push(Box::new(party_set_name::PartySetNameOp {}));
    result.push(Box::new(
        party_set_next_battle_simulation_time::PartySetNextBattleSimulationTimeOp {},
    ));
    result.push(Box::new(party_set_position::PartySetPositionOp {}));
    result.push(Box::new(party_set_slot::PartySetSlotOp {}));
    result.push(Box::new(party_slot_eq::PartySlotEqOp {}));
    result.push(Box::new(party_slot_ge::PartySlotGeOp {}));
    result.push(Box::new(
        party_stack_get_num_wounded::PartyStackGetNumWoundedOp {},
    ));
    result.push(Box::new(party_stack_get_size::PartyStackGetSizeOp {}));
    result.push(Box::new(
        party_stack_get_troop_dna::PartyStackGetTroopDnaOp {},
    ));
    result.push(Box::new(
        party_stack_get_troop_id::PartyStackGetTroopIdOp {},
    ));
    result.push(Box::new(party_template_get_slot::PartyTemplateGetSlotOp {}));
    result.push(Box::new(party_template_set_slot::PartyTemplateSetSlotOp {}));
    result.push(Box::new(party_template_slot_eq::PartyTemplateSlotEqOp {}));
    result.push(Box::new(party_template_slot_ge::PartyTemplateSlotGeOp {}));
    result.push(Box::new(party_upgrade_with_xp::PartyUpgradeWithXpOp {}));
    result.push(Box::new(party_wound_members::PartyWoundMembersOp {}));
    result.push(Box::new(
        remove_member_from_party::RemoveMemberFromPartyOp {},
    ));
    result.push(Box::new(remove_party::RemovePartyOp {}));
    result.push(Box::new(
        remove_regular_prisoners::RemoveRegularPrisonersOp {},
    ));
    result.push(Box::new(
        remove_troops_from_companions::RemoveTroopsFromCompanionsOp {},
    ));
    result.push(Box::new(
        remove_troops_from_prisoners::RemoveTroopsFromPrisonersOp {},
    ));
    result.push(Box::new(set_camera_follow_party::SetCameraFollowPartyOp {}));
    result.push(Box::new(
        set_party_creation_random_limits::SetPartyCreationRandomLimitsOp {},
    ));
    result.push(Box::new(set_spawn_radius::SetSpawnRadiusOp {}));
    result.push(Box::new(spawn_around_party::SpawnAroundPartyOp {}));
    result.push(Box::new(
        store01_random_parties_in_range::Store01RandomPartiesInRangeOp {},
    ));
    result.push(Box::new(
        store_distance_to_party_from_party::StoreDistanceToPartyFromPartyOp {},
    ));
    result.push(Box::new(store_faction_of_party::StoreFactionOfPartyOp {}));
    result.push(Box::new(
        store_num_parties_created::StoreNumPartiesCreatedOp {},
    ));
    result.push(Box::new(
        store_num_parties_destroyed::StoreNumPartiesDestroyedOp {},
    ));
    result.push(Box::new(
        store_num_parties_destroyed_by_player::StoreNumPartiesDestroyedByPlayerOp {},
    ));
    result.push(Box::new(
        store_num_parties_of_template::StoreNumPartiesOfTemplateOp {},
    ));
    result.push(Box::new(
        store_num_regular_prisoners::StoreNumRegularPrisonersOp {},
    ));
    result.push(Box::new(store_party_size::StorePartySizeOp {}));
    result.push(Box::new(
        store_party_size_wo_prisoners::StorePartySizeWoPrisonersOp {},
    ));
    result.push(Box::new(
        store_random_party_in_range::StoreRandomPartyInRangeOp {},
    ));
    result.push(Box::new(
        store_random_party_of_template::StoreRandomPartyOfTemplateOp {},
    ));
    result.push(Box::new(
        store_troop_count_companions::StoreTroopCountCompanionsOp {},
    ));
    result.push(Box::new(
        store_troop_count_prisoners::StoreTroopCountPrisonersOp {},
    ));
    result.push(Box::new(store_troop_kind_count::StoreTroopKindCountOp {}));
    result.push(Box::new(troops_can_join::TroopsCanJoinOp {}));
    result.push(Box::new(
        troops_can_join_as_prisoner::TroopsCanJoinAsPrisonerOp {},
    ));
    result.push(Box::new(troop_join::TroopJoinOp {}));
    result.push(Box::new(troop_join_as_prisoner::TroopJoinAsPrisonerOp {}));
    result
}
