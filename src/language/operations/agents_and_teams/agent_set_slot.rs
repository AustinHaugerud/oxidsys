use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentSetSlotOp;

const DOC : &str = "agent_get_slot                           =  525   (agent_get_slot, <destination>, <agent_id>, <slot_no>),";

pub const OP_CODE: u32 = 505;

pub const IDENT: &str = "agent_set_slot";

impl Operation for AgentSetSlotOp {
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
                make_param_doc("<slot_no>", ""),
                make_param_doc("<value>", ""),
            ],
        }
    }
}
