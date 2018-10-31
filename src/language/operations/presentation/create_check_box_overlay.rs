use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct CreateCheckBoxOverlayOp;

const DOC: &str = "Creates a checkbox overlay. Returns checkbox overlay_id.";

pub const OP_CODE: u32 = 918;

pub const IDENT: &str = "create_check_box_overlay";

impl Operation for CreateCheckBoxOverlayOp {
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
                make_param_doc("<checkbox_off_mesh>", ""),
                make_param_doc("<checkbox_on_mesh>", ""),
            ],
        }
    }
}
