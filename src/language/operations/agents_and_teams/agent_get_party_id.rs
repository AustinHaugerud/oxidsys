use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentGetPartyIdOp;

const DOC : &str = "Retrieves the party that the specified agent belongs to (supposedly should only work in battle missions for agents spawned as starting/reinforcement waves).";

pub const OP_CODE: u32 = 1716;

pub const IDENT: &str = "agent_get_party_id";

impl Operation for AgentGetPartyIdOp {
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
                make_param_doc("<agent_id>", ""),
            ],
        }
    }
}
