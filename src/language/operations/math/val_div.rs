use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ValDivOp;

const DOC: &str = "Assigns <destination> := <destination> / <value>";

pub const OP_CODE: u32 = 2108;

pub const IDENT: &str = "val_div";

impl Operation for ValDivOp {
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
