use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct TeamSetSlotOp;

const DOC : &str = "team_get_slot                            =  529   (team_get_slot, <destination>, <player_id>, <slot_no>),";

pub const OP_CODE: u32 = 509;

pub const IDENT: &str = "team_set_slot";

impl Operation for TeamSetSlotOp {
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
                make_param_doc("<team_id>", ""),
                make_param_doc("<slot_no>", ""),
                make_param_doc("<value>", ""),
            ],
        }
    }
}
