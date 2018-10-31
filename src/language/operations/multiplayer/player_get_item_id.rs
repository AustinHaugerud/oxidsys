use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PlayerGetItemIdOp;

const DOC : &str = "Server operation. Retrieves item that's currently equipped by specified player in <item_slot_no> equipment slot.";

pub const OP_CODE: u32 = 422;

pub const IDENT: &str = "player_get_item_id";

impl Operation for PlayerGetItemIdOp {
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
                make_param_doc("<item_slot_no>", ""),
            ],
        }
    }
}
