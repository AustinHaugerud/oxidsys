use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentGetAmmoForSlotOp;

const DOC: &str = "Retrieves the amount of ammo agent has in the referenced slot (range 0..3).";

pub const OP_CODE: u32 = 1825;

pub const IDENT: &str = "agent_get_ammo_for_slot";

impl Operation for AgentGetAmmoForSlotOp {
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
                make_param_doc("<slot_no>", ""),
            ],
        }
    }
}
