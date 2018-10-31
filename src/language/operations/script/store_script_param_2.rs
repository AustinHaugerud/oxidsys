use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StoreScriptParam2Op;

const DOC: &str = "Retrieve the value of the second script parameter.";

pub const OP_CODE: u32 = 22;

pub const IDENT: &str = "store_script_param_2";

impl Operation for StoreScriptParam2Op {
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
