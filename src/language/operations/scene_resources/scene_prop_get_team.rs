use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ScenePropGetTeamOp;

const DOC: &str = "Retrieves the team controlling the scene prop instance.";

pub const OP_CODE: u32 = 1817;

pub const IDENT: &str = "scene_prop_get_team";

impl Operation for ScenePropGetTeamOp {
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
                make_param_doc("<value>", ""),
                make_param_doc("<scene_prop_id>", ""),
            ],
        }
    }
}
