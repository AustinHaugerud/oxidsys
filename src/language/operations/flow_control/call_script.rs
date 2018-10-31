use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct CallScriptOp;

const DOC: &str = "Calls specified script with or without parameters.";

pub const OP_CODE: u32 = 1;

pub const IDENT: &str = "call_script";

impl Operation for CallScriptOp {
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
            num_optional: 20,
            param_docs: vec![
                make_param_doc("<script_id>", ""),
                make_param_doc("<script_param>", ""),
                make_param_doc("[<script_param>...]", ""),
            ],
        }
    }
}
