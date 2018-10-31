use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StoreScriptParamOp;

const DOC : &str = "Retrieve the value of arbitrary script parameter (generally used when script accepts more than two). Parameters are enumerated starting from 1.";

pub const OP_CODE: u32 = 23;

pub const IDENT: &str = "store_script_param";

impl Operation for StoreScriptParamOp {
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
            num_required: 2,
            num_optional: 0,
            param_docs: vec![
                make_param_doc("<destination>", ""),
                make_param_doc("<script_param_index>", ""),
            ],
        }
    }
}
