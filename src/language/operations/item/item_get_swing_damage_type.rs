use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ItemGetSwingDamageTypeOp;

const DOC : &str = "Version 1.161+. Retrieves swing damage type for item (see definitions for \"cut\", \"pierce\" and \"blunt\" in header_items.py).";

pub const OP_CODE: u32 = 2721;

pub const IDENT: &str = "item_get_swing_damage_type";

impl Operation for ItemGetSwingDamageTypeOp {
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
                make_param_doc("<destination>", ""),
                make_param_doc("<item_kind_no>", ""),
            ],
        }
    }
}
