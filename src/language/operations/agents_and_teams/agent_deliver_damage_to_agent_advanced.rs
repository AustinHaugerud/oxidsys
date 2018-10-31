use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentDeliverDamageToAgentAdvancedOp;

const DOC : &str = "Version 1.153+. Same as (agent_deliver_damage_to_agent), but resulting damage is returned. Also operation takes relations between agents into account, which may result in no damage, or even damage to attacker due to friendly fire rules.";

pub const OP_CODE: u32 = 1827;

pub const IDENT: &str = "agent_deliver_damage_to_agent_advanced";

impl Operation for AgentDeliverDamageToAgentAdvancedOp {
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
            num_required: 4,
            num_optional: 1,
            param_docs: vec![
                make_param_doc("<destination>", ""),
                make_param_doc("<attacker_agent_id>", ""),
                make_param_doc("<agent_id>", ""),
                make_param_doc("<value>", ""),
                make_param_doc("[weapon_item_id]", ""),
            ],
        }
    }
}
