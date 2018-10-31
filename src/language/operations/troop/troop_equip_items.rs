use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct TroopEquipItemsOp;

const DOC : &str = "Makes the troop reconsider it's equipment. If troop has better stuff in it's inventory, he will equip it. Note this operation sucks with weapons and may force the troop to equip himself with 4 two-handed swords.";

pub const OP_CODE: u32 = 1533;

pub const IDENT: &str = "troop_equip_items";

impl Operation for TroopEquipItemsOp {
    fn op_code(&self) -> u32 {
        OP_CODE
    }

    fn documentation(&self) -> &'static str {
        DOC
    }

    fn identifier(&self) -> &'static str {
        IDENT
    }

    fn param_info(&self) -> ParamInfo {
        ParamInfo {
            num_required: 1,
            num_optional: 0,
            param_docs: vec![make_param_doc("<troop_id>", "")],
        }
    }
}
