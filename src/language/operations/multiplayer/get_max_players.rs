use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct GetMaxPlayersOp;

const DOC : &str = "Returns maximum possible number of connected players. Apparently always returns a constant value, however it's return value can change as maximum increases with new patches.";

pub const OP_CODE: u32 = 400;

pub const IDENT: &str = "get_max_players";

impl Operation for GetMaxPlayersOp {
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
