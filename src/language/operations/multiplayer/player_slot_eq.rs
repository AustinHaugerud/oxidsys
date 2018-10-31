use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PlayerSlotEqOp;

const DOC : &str = "player_slot_ge                               =  568   (player_slot_ge, <player_id>, <slot_no>, <value>),";

pub const OP_CODE: u32 = 548;

pub const IDENT: &str = "player_slot_eq";

impl Operation for PlayerSlotEqOp {
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
                make_param_doc("<player_id>", ""),
                make_param_doc("<slot_no>", ""),
                make_param_doc("<value>", ""),
            ],
        }
    }
}
