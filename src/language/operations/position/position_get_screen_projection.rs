use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PositionGetScreenProjectionOp;

const DOC : &str = "Calculates the screen coordinates of the position and stores it as position_screen's X and Y coordinates.";

pub const OP_CODE: u32 = 750;

pub const IDENT: &str = "position_get_screen_projection";

impl Operation for PositionGetScreenProjectionOp {
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
                make_param_doc("<position_screen>", ""),
                make_param_doc("<position_world>", ""),
            ],
        }
    }
}
