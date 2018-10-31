use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StoreScriptParam1Op;

const DOC: &str = "Retrieve the value of the first script parameter.";

pub const OP_CODE: u32 = 21;

pub const IDENT: &str = "store_script_param_1";

impl Operation for StoreScriptParam1Op {
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
