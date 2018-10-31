use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ValMinOp;

const DOC: &str = "Assigns <destination> := MIN (<destination>, <value>)";

pub const OP_CODE: u32 = 2110;

pub const IDENT: &str = "val_min";

impl Operation for ValMinOp {
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
