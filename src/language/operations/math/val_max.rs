use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ValMaxOp;

const DOC: &str = "Assigns <destination> := MAX (<destination>, <value>)";

pub const OP_CODE: u32 = 2111;

pub const IDENT: &str = "val_max";

impl Operation for ValMaxOp {
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
                make_param_doc("<value>", ""),
            ],
        }
    }
}
