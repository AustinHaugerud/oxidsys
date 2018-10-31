use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct TryForPropInstancesOp;

const DOC : &str = "Version 1.161+. Runs a cycle, iterating all scene prop instances on the scene, or all scene prop instances of specific type if optional parameter is provided.";

pub const OP_CODE: u32 = 16;

pub const IDENT: &str = "try_for_prop_instances";

impl Operation for TryForPropInstancesOp {
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
            num_optional: 1,
            param_docs: vec![
                make_param_doc("<destination>", ""),
                make_param_doc("[<scene_prop_id>]", ""),
            ],
        }
    }
}
