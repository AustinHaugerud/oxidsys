use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ConvertToFixedPointOp;

const DOC: &str =
    "Converts integer value to fixed point (multiplies by the fixed point multiplier).";

pub const OP_CODE: u32 = 2130;

pub const IDENT: &str = "convert_to_fixed_point";

impl Operation for ConvertToFixedPointOp {
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
            param_docs: vec![make_param_doc("<destination_fixed_point>", "")],
        }
    }
}
