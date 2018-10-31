use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ScenePropSetPruneTimeOp;

const DOC : &str = "Not documented. Not used in Native. Taleworlds comment: Prune time can only be set to objects that are already on the prune queue. Static objects are not affected by this operation.";

pub const OP_CODE: u32 = 1819;

pub const IDENT: &str = "scene_prop_set_prune_time";

impl Operation for ScenePropSetPruneTimeOp {
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
