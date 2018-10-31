use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentRefillWieldedShieldHitPointsOp;

const DOC: &str = "Restores all hit points for the shield the agent is currently wielding.";

pub const OP_CODE: u32 = 1692;

pub const IDENT: &str = "agent_refill_wielded_shield_hit_points";

impl Operation for AgentRefillWieldedShieldHitPointsOp {
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
            num_required: 1,
            num_optional: 0,
            param_docs: vec![make_param_doc("<agent_id>", "")],
        }
    }
}
