use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct TroopSlotEqOp;

const DOC : &str = "troop_slot_ge                            =  560   (troop_slot_ge, <troop_id>, <slot_no>, <value>),";

pub const OP_CODE: u32 = 540;

pub const IDENT: &str = "troop_slot_eq";

impl Operation for TroopSlotEqOp {
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
