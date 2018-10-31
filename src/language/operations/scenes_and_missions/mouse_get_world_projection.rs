use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct MouseGetWorldProjectionOp;

const DOC : &str = "Version 1.161+. Returns current camera coordinates (first position) and mouse projection to the back of the world (second position). Rotation data of resulting positions seems unreliable.";

pub const OP_CODE: u32 = 751;

pub const IDENT: &str = "mouse_get_world_projection";

impl Operation for MouseGetWorldProjectionOp {
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
                make_param_doc("<position_no_1>", ""),
                make_param_doc("<position_no_2>", ""),
            ],
        }
    }
}
