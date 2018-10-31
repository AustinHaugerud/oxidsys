use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentSetDamageModifierOp;

const DOC : &str = "Version 1.153+. Changes the damage delivered by this agent. Value is in percentage, 100 is default, 1000 is max possible value.";

pub const OP_CODE: u32 = 2091;

pub const IDENT: &str = "agent_set_damage_modifier";

impl Operation for AgentSetDamageModifierOp {
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
