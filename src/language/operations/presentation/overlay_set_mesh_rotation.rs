use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct OverlaySetMeshRotationOp;

const DOC : &str = "Despite the name, works with any overlay, allowing you to put it on the screen in rotated position. To determine the angles, position's rotation values are used (not coordinates!). Usually you will want to only use rotation around Z axis (which results in clockwise or anti-clockwise rotation as seen by user). Note that rotating overlays which are placed inside a container may cause strange results, so some trial and error will be necessary in such situation.";

pub const OP_CODE: u32 = 930;

pub const IDENT: &str = "overlay_set_mesh_rotation";

impl Operation for OverlaySetMeshRotationOp {
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
                make_param_doc("<overlay_id>", ""),
                make_param_doc("<position>", ""),
            ],
        }
    }
}
