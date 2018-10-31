use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PlayerSetGoldOp;

const DOC : &str = "Sets player's new gold amount and maximum allowed gold amount. Use 0 for <max_value> to remove gold limit.";

pub const OP_CODE: u32 = 408;

pub const IDENT: &str = "player_set_gold";

impl Operation for PlayerSetGoldOp {
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
                make_param_doc("<value>", ""),
                make_param_doc("<max_value>", ""),
            ],
        }
    }
}
