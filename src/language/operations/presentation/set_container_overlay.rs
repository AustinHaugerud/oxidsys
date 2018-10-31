use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct SetContainerOverlayOp;

const DOC : &str = "Defines the specified overlay as the container. All subsequently created overlays will be placed inside the container, and their coordinates will be based on container's position. All containers with their contents will be displayed *above* any non-container overlays. Use -1 to stop placing overlays to current container and resume normal behavior.";

pub const OP_CODE: u32 = 945;

pub const IDENT: &str = "set_container_overlay";

impl Operation for SetContainerOverlayOp {
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
            num_required: 1,
            num_optional: 0,
            param_docs: vec![make_param_doc("<overlay_id>", "")],
        }
    }
}
