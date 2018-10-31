use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StoreFriendCountOp;

const DOC : &str = "No longer used in Native. Apparently stores total number of active friendly agents. Possibly deprecated. 4research.";

pub const OP_CODE: u32 = 2381;

pub const IDENT: &str = "store_friend_count";

impl Operation for StoreFriendCountOp {
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
