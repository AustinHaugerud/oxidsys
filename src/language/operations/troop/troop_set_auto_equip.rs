use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct TroopSetAutoEquipOp;

const DOC : &str = "Sets (value = 1) or disables (value = 0) auto-equipping the troop with any items added to it's inventory or purchased. Similar to tf_is_merchant flag.";

pub const OP_CODE: u32 = 1509;

pub const IDENT: &str = "troop_set_auto_equip";

impl Operation for TroopSetAutoEquipOp {
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
            num_required: 2,
            num_optional: 0,
            param_docs: vec![
                make_param_doc("<troop_id>", ""),
                make_param_doc("<value>", ""),
            ],
        }
    }
}
