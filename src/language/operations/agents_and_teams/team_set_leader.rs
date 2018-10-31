use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct TeamSetLeaderOp;

const DOC: &str = "Sets the agent as the new leader of specified team.";

pub const OP_CODE: u32 = 1793;

pub const IDENT: &str = "team_set_leader";

impl Operation for TeamSetLeaderOp {
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
                make_param_doc("<team_no>", ""),
                make_param_doc("<new_leader_agent_id>", ""),
            ],
        }
    }
}
