use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentCheckOfferFromAgentOp;

const DOC: &str = "Esoteric stuff. Used in multiplayer duels. Second agent_id is offerer.";

pub const OP_CODE: u32 = 1778;

pub const IDENT: &str = "agent_check_offer_from_agent";

impl Operation for AgentCheckOfferFromAgentOp {
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
                make_param_doc("<agent_id>", ""),
                make_param_doc("<offerer_agent_id>", ""),
            ],
        }
    }
}
