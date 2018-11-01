use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentDeliverDamageToAgentOp;

const DOC : &str = "Makes one agent deal damage to another. Parameter damage_amount is optional, if it is skipped or <= 0, then damage will be calculated using attacker's weapon item and stats (like a normal weapon attack). Optional parameter weapon_item_id was added in 1.153 and will force the game the calculate the damage using this weapon.";

pub const OP_CODE: u32 = 1722;

pub const IDENT: &str = "agent_deliver_damage_to_agent";

impl Operation for AgentDeliverDamageToAgentOp {
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
            num_optional: 2,
            param_docs: vec![
                make_param_doc("<agent_id_deliverer>", ""),
                make_param_doc("<agent_id>", ""),
                make_param_doc("[damage_amount]", ""),
                make_param_doc("[weapon_item_id]", ""),
            ],
        }
    }
}
