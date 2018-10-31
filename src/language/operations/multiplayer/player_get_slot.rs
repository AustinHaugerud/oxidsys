use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PlayerGetSlotOp;

const DOC : &str = "player_slot_eq                               =  548   (player_slot_eq, <player_id>, <slot_no>, <value>),";

pub const OP_CODE: u32 = 528;

pub const IDENT: &str = "player_get_slot";

impl Operation for PlayerGetSlotOp {
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
                make_param_doc("<destination>", ""),
                make_param_doc("<player_id>", ""),
                make_param_doc("<slot_no>", ""),
            ],
        }
    }
}
