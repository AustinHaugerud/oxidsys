use language::operations::Operation;
pub mod faction_get_color;
pub mod faction_get_slot;
pub mod faction_set_color;
pub mod faction_set_name;
pub mod faction_set_slot;
pub mod faction_slot_eq;
pub mod faction_slot_ge;
pub mod set_relation;
pub mod store_relation;

pub fn load_operands() -> Vec<Box<Operation>> {
    let mut result: Vec<Box<Operation>> = vec![];
    result.push(Box::new(faction_get_color::FactionGetColorOp {}));
    result.push(Box::new(faction_get_slot::FactionGetSlotOp {}));
    result.push(Box::new(faction_set_color::FactionSetColorOp {}));
    result.push(Box::new(faction_set_name::FactionSetNameOp {}));
    result.push(Box::new(faction_set_slot::FactionSetSlotOp {}));
    result.push(Box::new(faction_slot_eq::FactionSlotEqOp {}));
    result.push(Box::new(faction_slot_ge::FactionSlotGeOp {}));
    result.push(Box::new(set_relation::SetRelationOp {}));
    result.push(Box::new(store_relation::StoreRelationOp {}));
    result
}
