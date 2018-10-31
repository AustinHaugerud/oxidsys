use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentSetSpeedModifierOp;

const DOC : &str = "Version 1.153+. Changes agent's speed. Value is in percentage, 100 is default, value can be between [0..1000]";

pub const OP_CODE: u32 = 2093;

pub const IDENT: &str = "agent_set_speed_modifier";

impl Operation for AgentSetSpeedModifierOp {
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
