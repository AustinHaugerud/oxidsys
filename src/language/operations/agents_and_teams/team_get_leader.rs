use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct TeamGetLeaderOp;

const DOC: &str = "Retrieves the reference to the agent who is the leader of specified team.";

pub const OP_CODE: u32 = 1792;

pub const IDENT: &str = "team_get_leader";

impl Operation for TeamGetLeaderOp {
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
                make_param_doc("<destination>", ""),
                make_param_doc("<team_no>", ""),
            ],
        }
    }
}
