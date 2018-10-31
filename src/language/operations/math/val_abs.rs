use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ValAbsOp;

const DOC: &str = "Assigns <destination> := ABS (<destination>)";

pub const OP_CODE: u32 = 2113;

pub const IDENT: &str = "val_abs";

impl Operation for ValAbsOp {
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
