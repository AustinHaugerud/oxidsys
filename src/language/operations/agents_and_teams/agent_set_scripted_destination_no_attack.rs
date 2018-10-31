use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentSetScriptedDestinationNoAttackOp;

const DOC: &str = "Same as above, but the agent will not attack his enemies.";

pub const OP_CODE: u32 = 1748;

pub const IDENT: &str = "agent_set_scripted_destination_no_attack";

impl Operation for AgentSetScriptedDestinationNoAttackOp {
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
            num_required: 3,
            num_optional: 0,
            param_docs: vec![
                make_param_doc("<agent_id>", ""),
                make_param_doc("<position>", ""),
                make_param_doc("<auto_set_z_to_ground_level>", ""),
            ],
        }
    }
}
