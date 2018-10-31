use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ValClampOp;

const DOC: &str = "Enforces <destination> value to be within <lower_bound>..<upper_bound>-1 range.";

pub const OP_CODE: u32 = 2112;

pub const IDENT: &str = "val_clamp";

impl Operation for ValClampOp {
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
            num_required: 3,
            num_optional: 0,
            param_docs: vec![
                make_param_doc("<destination>", ""),
                make_param_doc("<lower_bound>", ""),
                make_param_doc("<upper_bound>", ""),
            ],
        }
    }
}
