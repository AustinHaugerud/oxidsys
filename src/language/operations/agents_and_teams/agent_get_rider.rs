use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentGetRiderOp;

const DOC : &str = "Retrieves the reference to the rider agent who is riding the specified horse, or -1 if there's no rider or the specified agent is not a horse.";

pub const OP_CODE: u32 = 1715;

pub const IDENT: &str = "agent_get_rider";

impl Operation for AgentGetRiderOp {
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
                make_param_doc("<horse_agent_id>", ""),
            ],
        }
    }
}
