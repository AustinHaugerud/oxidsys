use language::operations::{make_param_doc, Operation, ParamInfo};
pub mod cur_item_add_mesh;
pub mod cur_item_set_material;
pub mod item_get_abundance;
pub mod item_get_accuracy;
pub mod item_get_body_armor;
pub mod item_get_difficulty;
pub mod item_get_food_quality;
pub mod item_get_head_armor;
pub mod item_get_hit_points;
pub mod item_get_horse_charge_damage;
pub mod item_get_horse_maneuver;
pub mod item_get_horse_scale;
pub mod item_get_horse_speed;
pub mod item_get_leg_armor;
pub mod item_get_max_ammo;
pub mod item_get_missile_speed;
pub mod item_get_shield_height;
pub mod item_get_slot;
pub mod item_get_speed_rating;
pub mod item_get_swing_damage;
pub mod item_get_swing_damage_type;
pub mod item_get_thrust_damage;
pub mod item_get_thrust_damage_type;
pub mod item_get_type;
pub mod item_get_value;
pub mod item_get_weapon_length;
pub mod item_get_weight;
pub mod item_has_capability;
pub mod item_has_faction;
pub mod item_has_modifier;
pub mod item_has_property;
pub mod item_set_slot;
pub mod item_slot_eq;
pub mod item_slot_ge;
pub mod store_item_value;
pub mod store_random_armor;
pub mod store_random_equipment;
pub mod store_random_horse;

pub fn load_operands() -> Vec<Box<Operation>> {
    let mut result: Vec<Box<Operation>> = vec![];
    result.push(Box::new(cur_item_add_mesh::CurItemAddMeshOp {}));
    result.push(Box::new(cur_item_set_material::CurItemSetMaterialOp {}));
    result.push(Box::new(item_get_abundance::ItemGetAbundanceOp {}));
    result.push(Box::new(item_get_accuracy::ItemGetAccuracyOp {}));
    result.push(Box::new(item_get_body_armor::ItemGetBodyArmorOp {}));
    result.push(Box::new(item_get_difficulty::ItemGetDifficultyOp {}));
    result.push(Box::new(item_get_food_quality::ItemGetFoodQualityOp {}));
    result.push(Box::new(item_get_head_armor::ItemGetHeadArmorOp {}));
    result.push(Box::new(item_get_hit_points::ItemGetHitPointsOp {}));
    result.push(Box::new(
        item_get_horse_charge_damage::ItemGetHorseChargeDamageOp {},
    ));
    result.push(Box::new(item_get_horse_maneuver::ItemGetHorseManeuverOp {}));
    result.push(Box::new(item_get_horse_scale::ItemGetHorseScaleOp {}));
    result.push(Box::new(item_get_horse_speed::ItemGetHorseSpeedOp {}));
    result.push(Box::new(item_get_leg_armor::ItemGetLegArmorOp {}));
    result.push(Box::new(item_get_max_ammo::ItemGetMaxAmmoOp {}));
    result.push(Box::new(item_get_missile_speed::ItemGetMissileSpeedOp {}));
    result.push(Box::new(item_get_shield_height::ItemGetShieldHeightOp {}));
    result.push(Box::new(item_get_slot::ItemGetSlotOp {}));
    result.push(Box::new(item_get_speed_rating::ItemGetSpeedRatingOp {}));
    result.push(Box::new(item_get_swing_damage::ItemGetSwingDamageOp {}));
    result.push(Box::new(
        item_get_swing_damage_type::ItemGetSwingDamageTypeOp {},
    ));
    result.push(Box::new(item_get_thrust_damage::ItemGetThrustDamageOp {}));
    result.push(Box::new(
        item_get_thrust_damage_type::ItemGetThrustDamageTypeOp {},
    ));
    result.push(Box::new(item_get_type::ItemGetTypeOp {}));
    result.push(Box::new(item_get_value::ItemGetValueOp {}));
    result.push(Box::new(item_get_weapon_length::ItemGetWeaponLengthOp {}));
    result.push(Box::new(item_get_weight::ItemGetWeightOp {}));
    result.push(Box::new(item_has_capability::ItemHasCapabilityOp {}));
    result.push(Box::new(item_has_faction::ItemHasFactionOp {}));
    result.push(Box::new(item_has_modifier::ItemHasModifierOp {}));
    result.push(Box::new(item_has_property::ItemHasPropertyOp {}));
    result.push(Box::new(item_set_slot::ItemSetSlotOp {}));
    result.push(Box::new(item_slot_eq::ItemSlotEqOp {}));
    result.push(Box::new(item_slot_ge::ItemSlotGeOp {}));
    result.push(Box::new(store_item_value::StoreItemValueOp {}));
    result.push(Box::new(store_random_armor::StoreRandomArmorOp {}));
    result.push(Box::new(store_random_equipment::StoreRandomEquipmentOp {}));
    result.push(Box::new(store_random_horse::StoreRandomHorseOp {}));
    result
}
