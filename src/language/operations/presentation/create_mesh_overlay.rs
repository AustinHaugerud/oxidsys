use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct CreateMeshOverlayOp;

const DOC: &str = "Creates a mesh overlay and returns it's overlay_id.";

pub const OP_CODE: u32 = 911;

pub const IDENT: &str = "create_mesh_overlay";

impl Operation for CreateMeshOverlayOp {
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
                make_param_doc("<mesh_id>", ""),
            ],
        }
    }
}
