use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentSetAttackActionOp;

const DOC : &str = "Forces the agent to perform an attack action. Direction value: -2 = cancel any action (1.153+), 0 = thrust, 1 = slashright, 2 = slashleft, 3 = overswing. Action value: 0 = ready and release, 1 = ready and hold.";

pub const OP_CODE: u32 = 1745;

pub const IDENT: &str = "agent_set_attack_action";

impl Operation for AgentSetAttackActionOp {
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
                make_param_doc("<direction_value>", ""),
                make_param_doc("<action_value>", ""),
            ],
        }
    }
}
