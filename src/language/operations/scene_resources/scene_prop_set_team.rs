use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ScenePropSetTeamOp;

const DOC: &str = "Assigns the scene prop instance to a certain team.";

pub const OP_CODE: u32 = 1818;

pub const IDENT: &str = "scene_prop_set_team";

impl Operation for ScenePropSetTeamOp {
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
