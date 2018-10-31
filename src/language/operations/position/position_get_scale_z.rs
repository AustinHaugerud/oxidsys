use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PositionGetScaleZOp;

const DOC: &str = "Retrieves position scaling along Z axis.";

pub const OP_CODE: u32 = 737;

pub const IDENT: &str = "position_get_scale_z";

impl Operation for PositionGetScaleZOp {
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
                make_param_doc("<destination_fixed_point>", ""),
                make_param_doc("<position>", ""),
            ],
        }
    }
}
