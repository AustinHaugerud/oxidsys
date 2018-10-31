use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AddPointLightOp;

const DOC : &str = "Adds a point light source to an object with optional flickering magnitude (range 0..100) and flickering interval (in 1/100th of second). Uses position offset and color provided to previous calls to (set_position_delta) and (set_current_color). Can only be used in item triggers.";

pub const OP_CODE: u32 = 1960;

pub const IDENT: &str = "add_point_light";

impl Operation for AddPointLightOp {
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
            num_required: 0,
            num_optional: 2,
            param_docs: vec![
                make_param_doc("[flicker_magnitude]", ""),
                make_param_doc("[flicker_interval]", ""),
            ],
        }
    }
}
