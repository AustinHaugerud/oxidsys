use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct TroopSetSlotOp;

const DOC : &str = "troop_get_slot                           =  520   (troop_get_slot, <destination>, <troop_id>, <slot_no>),";

pub const OP_CODE: u32 = 500;

pub const IDENT: &str = "troop_set_slot";

impl Operation for TroopSetSlotOp {
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
            num_required: 3,
            num_optional: 0,
            param_docs: vec![
                make_param_doc("<troop_id>", ""),
                make_param_doc("<slot_no>", ""),
                make_param_doc("<value>", ""),
            ],
        }
    }
}
