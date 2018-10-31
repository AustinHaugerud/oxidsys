use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PropInstanceInitializeRotationAnglesOp;

const DOC : &str = "Should be called to initialize the scene prop instance prior to any calls to (prop_instance_rotate_to_position).";

pub const OP_CODE: u32 = 1866;

pub const IDENT: &str = "prop_instance_initialize_rotation_angles";

impl Operation for PropInstanceInitializeRotationAnglesOp {
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
            param_docs: vec![make_param_doc("<scene_prop_id>", "")],
        }
    }
}
