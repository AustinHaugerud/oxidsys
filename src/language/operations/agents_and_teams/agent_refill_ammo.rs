use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentRefillAmmoOp;

const DOC: &str =
    "Refills all ammo and throwing weapon stacks that the agent has in his equipment.";

pub const OP_CODE: u32 = 1728;

pub const IDENT: &str = "agent_refill_ammo";

impl Operation for AgentRefillAmmoOp {
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
