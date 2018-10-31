use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StoreAllyCountOp;

const DOC : &str = "No longer used in Native. Apparently stores total number of active allied agents (how is it different from friends?). Possibly deprecated. 4research.";

pub const OP_CODE: u32 = 2382;

pub const IDENT: &str = "store_ally_count";

impl Operation for StoreAllyCountOp {
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
