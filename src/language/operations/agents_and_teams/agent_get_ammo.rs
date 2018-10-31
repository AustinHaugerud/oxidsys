use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentGetAmmoOp;

const DOC : &str = "Retrieves the current ammo amount agent has for his wielded item (value = 1) or all his items (value = 0).";

pub const OP_CODE: u32 = 1727;

pub const IDENT: &str = "agent_get_ammo";

impl Operation for AgentGetAmmoOp {
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
                make_param_doc("<destination>", ""),
                make_param_doc("<agent_id>", ""),
                make_param_doc("<value>", ""),
            ],
        }
    }
}
