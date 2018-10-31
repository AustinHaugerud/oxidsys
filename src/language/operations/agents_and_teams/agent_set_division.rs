use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentSetDivisionOp;

const DOC : &str = "Puts the agent into the specified division. This does not affect agent's troop class. Note that there's a bug in Warband: if an order is issued to agent's original division, the agent will immediately switch back to it's original division number. Therefore, if you want to manipulate agent divisions dynamically during the battle, you need to implement some workarounds for this bug.";

pub const OP_CODE: u32 = 1783;

pub const IDENT: &str = "agent_set_division";

impl Operation for AgentSetDivisionOp {
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
                make_param_doc("<value>", ""),
            ],
        }
    }
}
