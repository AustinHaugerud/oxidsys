use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct SetFixedPointMultiplierOp;

const DOC: &str = "Affects all operations dealing with fixed point numbers. Default value is 1.";

pub const OP_CODE: u32 = 2124;

pub const IDENT: &str = "set_fixed_point_multiplier";

impl Operation for SetFixedPointMultiplierOp {
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
            param_docs: vec![make_param_doc("<value>", "")],
        }
    }
}
