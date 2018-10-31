use language::operations::{make_param_doc, Operation, ParamInfo};
pub mod add_gold_as_xp;
pub mod add_xp_as_reward;
pub mod add_xp_to_troop;
pub mod class_set_name;
pub mod face;
pub mod get_level_boundary;
pub mod player_has_item;
pub mod store_attribute_level;
pub mod store_character_level;
pub mod store_faction_of_troop;
pub mod store_free_inventory_capacity;
pub mod store_item_kind_count;
pub mod store_proficiency_level;
pub mod store_skill_level;
pub mod store_troop_faction;
pub mod store_troop_gold;
pub mod store_troop_health;
pub mod store_troop_value;
pub mod troop_add_gold;
pub mod troop_add_item;
pub mod troop_add_items;
pub mod troop_add_merchandise;
pub mod troop_add_merchandise_with_faction;
pub mod troop_add_proficiency_points;
pub mod troop_clear_inventory;
pub mod troop_ensure_inventory_space;
pub mod troop_equip_items;
pub mod troop_get_class;
pub mod troop_get_inventory_capacity;
pub mod troop_get_inventory_slot;
pub mod troop_get_inventory_slot_modifier;
pub mod troop_get_slot;
pub mod troop_get_type;
pub mod troop_get_upgrade_troop;
pub mod troop_get_xp;
pub mod troop_has_item_equipped;
pub mod troop_inventory_slot_get_item_amount;
pub mod troop_inventory_slot_get_item_max_amount;
pub mod troop_inventory_slot_set_item_amount;
pub mod troop_is_guarantee_horse;
pub mod troop_is_guarantee_ranged;
pub mod troop_is_hero;
pub mod troop_is_mounted;
pub mod troop_is_wounded;
pub mod troop_loot_troop;
pub mod troop_raise_proficiency;
pub mod troop_raise_proficiency_linear;
pub mod troop_raise_skill;
pub mod troop_remove_gold;
pub mod troop_remove_item;
pub mod troop_remove_items;
pub mod troop_set_age;
pub mod troop_set_auto_equip;
pub mod troop_set_class;
pub mod troop_set_face_key_from_current_profile;
pub mod troop_set_faction;
pub mod troop_set_health;
pub mod troop_set_inventory_slot;
pub mod troop_set_inventory_slot_modifier;
pub mod troop_set_name;
pub mod troop_set_plural_name;
pub mod troop_set_slot;
pub mod troop_set_type;
pub mod troop_slot_eq;
pub mod troop_slot_ge;
pub mod troop_sort_inventory;

pub fn load_operands() -> Vec<Box<Operation>> {
    let mut result: Vec<Box<Operation>> = vec![];
    result.push(Box::new(add_gold_as_xp::AddGoldAsXpOp {}));
    result.push(Box::new(add_xp_as_reward::AddXpAsRewardOp {}));
    result.push(Box::new(add_xp_to_troop::AddXpToTroopOp {}));
    result.push(Box::new(class_set_name::ClassSetNameOp {}));
    result.push(Box::new(get_level_boundary::GetLevelBoundaryOp {}));
    result.push(Box::new(player_has_item::PlayerHasItemOp {}));
    result.push(Box::new(store_attribute_level::StoreAttributeLevelOp {}));
    result.push(Box::new(store_character_level::StoreCharacterLevelOp {}));
    result.push(Box::new(store_faction_of_troop::StoreFactionOfTroopOp {}));
    result.push(Box::new(
        store_free_inventory_capacity::StoreFreeInventoryCapacityOp {},
    ));
    result.push(Box::new(store_item_kind_count::StoreItemKindCountOp {}));
    result.push(Box::new(
        store_proficiency_level::StoreProficiencyLevelOp {},
    ));
    result.push(Box::new(store_skill_level::StoreSkillLevelOp {}));
    result.push(Box::new(store_troop_faction::StoreTroopFactionOp {}));
    result.push(Box::new(store_troop_gold::StoreTroopGoldOp {}));
    result.push(Box::new(store_troop_health::StoreTroopHealthOp {}));
    result.push(Box::new(store_troop_value::StoreTroopValueOp {}));
    result.push(Box::new(troop_add_gold::TroopAddGoldOp {}));
    result.push(Box::new(troop_add_item::TroopAddItemOp {}));
    result.push(Box::new(troop_add_items::TroopAddItemsOp {}));
    result.push(Box::new(troop_add_merchandise::TroopAddMerchandiseOp {}));
    result.push(Box::new(
        troop_add_merchandise_with_faction::TroopAddMerchandiseWithFactionOp {},
    ));
    result.push(Box::new(
        troop_add_proficiency_points::TroopAddProficiencyPointsOp {},
    ));
    result.push(Box::new(troop_clear_inventory::TroopClearInventoryOp {}));
    result.push(Box::new(
        troop_ensure_inventory_space::TroopEnsureInventorySpaceOp {},
    ));
    result.push(Box::new(troop_equip_items::TroopEquipItemsOp {}));
    result.push(Box::new(troop_get_class::TroopGetClassOp {}));
    result.push(Box::new(
        troop_get_inventory_capacity::TroopGetInventoryCapacityOp {},
    ));
    result.push(Box::new(
        troop_get_inventory_slot::TroopGetInventorySlotOp {},
    ));
    result.push(Box::new(
        troop_get_inventory_slot_modifier::TroopGetInventorySlotModifierOp {},
    ));
    result.push(Box::new(troop_get_slot::TroopGetSlotOp {}));
    result.push(Box::new(troop_get_type::TroopGetTypeOp {}));
    result.push(Box::new(troop_get_upgrade_troop::TroopGetUpgradeTroopOp {}));
    result.push(Box::new(troop_get_xp::TroopGetXpOp {}));
    result.push(Box::new(troop_has_item_equipped::TroopHasItemEquippedOp {}));
    result.push(Box::new(
        troop_inventory_slot_get_item_amount::TroopInventorySlotGetItemAmountOp {},
    ));
    result.push(Box::new(
        troop_inventory_slot_get_item_max_amount::TroopInventorySlotGetItemMaxAmountOp {},
    ));
    result.push(Box::new(
        troop_inventory_slot_set_item_amount::TroopInventorySlotSetItemAmountOp {},
    ));
    result.push(Box::new(
        troop_is_guarantee_horse::TroopIsGuaranteeHorseOp {},
    ));
    result.push(Box::new(
        troop_is_guarantee_ranged::TroopIsGuaranteeRangedOp {},
    ));
    result.push(Box::new(troop_is_hero::TroopIsHeroOp {}));
    result.push(Box::new(troop_is_mounted::TroopIsMountedOp {}));
    result.push(Box::new(troop_is_wounded::TroopIsWoundedOp {}));
    result.push(Box::new(troop_loot_troop::TroopLootTroopOp {}));
    result.push(Box::new(
        troop_raise_proficiency::TroopRaiseProficiencyOp {},
    ));
    result.push(Box::new(
        troop_raise_proficiency_linear::TroopRaiseProficiencyLinearOp {},
    ));
    result.push(Box::new(troop_raise_skill::TroopRaiseSkillOp {}));
    result.push(Box::new(troop_remove_gold::TroopRemoveGoldOp {}));
    result.push(Box::new(troop_remove_item::TroopRemoveItemOp {}));
    result.push(Box::new(troop_remove_items::TroopRemoveItemsOp {}));
    result.push(Box::new(troop_set_age::TroopSetAgeOp {}));
    result.push(Box::new(troop_set_auto_equip::TroopSetAutoEquipOp {}));
    result.push(Box::new(troop_set_class::TroopSetClassOp {}));
    result.push(Box::new(
        troop_set_face_key_from_current_profile::TroopSetFaceKeyFromCurrentProfileOp {},
    ));
    result.push(Box::new(troop_set_faction::TroopSetFactionOp {}));
    result.push(Box::new(troop_set_health::TroopSetHealthOp {}));
    result.push(Box::new(
        troop_set_inventory_slot::TroopSetInventorySlotOp {},
    ));
    result.push(Box::new(
        troop_set_inventory_slot_modifier::TroopSetInventorySlotModifierOp {},
    ));
    result.push(Box::new(troop_set_name::TroopSetNameOp {}));
    result.push(Box::new(troop_set_plural_name::TroopSetPluralNameOp {}));
    result.push(Box::new(troop_set_slot::TroopSetSlotOp {}));
    result.push(Box::new(troop_set_type::TroopSetTypeOp {}));
    result.push(Box::new(troop_slot_eq::TroopSlotEqOp {}));
    result.push(Box::new(troop_slot_ge::TroopSlotGeOp {}));
    result.push(Box::new(troop_sort_inventory::TroopSortInventoryOp {}));
    result.extend(face::load_operands());
    result
}
