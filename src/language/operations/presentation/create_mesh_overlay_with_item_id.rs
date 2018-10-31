use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct CreateMeshOverlayWithItemIdOp;

const DOC: &str = "Creates a mesh overlay, using the specified item mesh. Returns overlay_id.";

pub const OP_CODE: u32 = 944;

pub const IDENT: &str = "create_mesh_overlay_with_item_id";

impl Operation for CreateMeshOverlayWithItemIdOp {
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
                make_param_doc("<item_id>", ""),
            ],
        }
    }
}
