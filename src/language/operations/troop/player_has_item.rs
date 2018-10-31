use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PlayerHasItemOp;

const DOC: &str = "Checks that player has the specified item.";

pub const OP_CODE: u32 = 150;

pub const IDENT: &str = "player_has_item";

impl Operation for PlayerHasItemOp {
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
            param_docs: vec![make_param_doc("<item_id>", "")],
        }
    }
}
