use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct CreateButtonOverlayOp;

const DOC : &str = "Creates a generic button overlay and returns it's overlay_id. The only difference between this and subsequent two operations is that they use different button meshes.";

pub const OP_CODE: u32 = 912;

pub const IDENT: &str = "create_button_overlay";

impl Operation for CreateButtonOverlayOp {
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
                make_param_doc("<string_id>", ""),
            ],
        }
    }
}
