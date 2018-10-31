use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct OverlaySetContainerOverlayOp;

const DOC : &str = "Allows you to put one overlay into a container, or remove it from container (if container_overlay_id = -1) without setting current overlay. May be unreliable.";

pub const OP_CODE: u32 = 951;

pub const IDENT: &str = "overlay_set_container_overlay";

impl Operation for OverlaySetContainerOverlayOp {
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
                make_param_doc("<container_overlay_id>", ""),
            ],
        }
    }
}
