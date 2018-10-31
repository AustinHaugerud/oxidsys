use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct SceneGetSlotOp;

const DOC : &str = "scene_slot_eq                                =  543   (scene_slot_eq, <scene_id>, <slot_no>, <value>),";

pub const OP_CODE: u32 = 523;

pub const IDENT: &str = "scene_get_slot";

impl Operation for SceneGetSlotOp {
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
                make_param_doc("<scene_id>", ""),
                make_param_doc("<slot_no>", ""),
            ],
        }
    }
}
