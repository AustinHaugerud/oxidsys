use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PropInstanceEnablePhysicsOp;

const DOC : &str = "Enables (value = 1) or disables (value = 0) physics calculation (gravity, collision checks) for the scene prop instance.";

pub const OP_CODE: u32 = 1864;

pub const IDENT: &str = "prop_instance_enable_physics";

impl Operation for PropInstanceEnablePhysicsOp {
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
                make_param_doc("<scene_prop_id>", ""),
                make_param_doc("<value>", ""),
            ],
        }
    }
}
