use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct SceneSetSlotOp;

const DOC : &str = "scene_get_slot                               =  523   (scene_get_slot, <destination>, <scene_id>, <slot_no>),";

pub const OP_CODE: u32 = 503;

pub const IDENT: &str = "scene_set_slot";

impl Operation for SceneSetSlotOp {
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
                make_param_doc("<scene_id>", ""),
                make_param_doc("<slot_no>", ""),
                make_param_doc("<value>", ""),
            ],
        }
    }
}
