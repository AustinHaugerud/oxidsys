use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StoreNormalizedTeamCountOp;

const DOC : &str = "Stores the number of agents belonging to specified team, normalized according to battle_size and advantage. Commonly used to calculate advantage and possibly reinforcement wave sizes.";

pub const OP_CODE: u32 = 2385;

pub const IDENT: &str = "store_normalized_team_count";

impl Operation for StoreNormalizedTeamCountOp {
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
