use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PositionGetZOp;

const DOC : &str = "Return position Z coordinate (to the top). Base unit is meters. Use (set_fixed_point_multiplier) to set another measurement unit (100 will get you centimeters, 1000 will get you millimeters, etc).";

pub const OP_CODE: u32 = 728;

pub const IDENT: &str = "position_get_z";

impl Operation for PositionGetZOp {
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
