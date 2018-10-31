use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentGetSlotOp;

const DOC : &str = "agent_slot_eq                            =  545   (agent_slot_eq, <agent_id>, <slot_no>, <value>),";

pub const OP_CODE: u32 = 525;

pub const IDENT: &str = "agent_get_slot";

impl Operation for AgentGetSlotOp {
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
