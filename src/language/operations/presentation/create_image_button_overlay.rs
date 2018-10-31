use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct CreateImageButtonOverlayOp;

const DOC : &str = "Creates an image button, using two meshes for normal (1st mesh) and pressed (2nd mesh) status. Button does not have a textual label. Returns button overlay_id.";

pub const OP_CODE: u32 = 913;

pub const IDENT: &str = "create_image_button_overlay";

impl Operation for CreateImageButtonOverlayOp {
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
                make_param_doc("<mesh_id>", ""),
                make_param_doc("<mesh_id>", ""),
            ],
        }
    }
}
