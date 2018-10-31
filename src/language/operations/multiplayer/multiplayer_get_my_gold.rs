use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct MultiplayerGetMyGoldOp;

const DOC: &str = "Client operation. Retrieves current player's gold amount.";

pub const OP_CODE: u32 = 414;

pub const IDENT: &str = "multiplayer_get_my_gold";

impl Operation for MultiplayerGetMyGoldOp {
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
            param_docs: vec![make_param_doc("<destination>", "")],
        }
    }
}
