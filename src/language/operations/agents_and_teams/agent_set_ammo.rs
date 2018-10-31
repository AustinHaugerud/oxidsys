use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentSetAmmoOp;

const DOC : &str = "Sets current agent ammo amount to the specified value between 0 and maximum ammo. Not clear what item_id means - weapon item or ammo item? 4research.";

pub const OP_CODE: u32 = 1776;

pub const IDENT: &str = "agent_set_ammo";

impl Operation for AgentSetAmmoOp {
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
                make_param_doc("<item_id>", ""),
                make_param_doc("<value>", ""),
            ],
        }
    }
}
