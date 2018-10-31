use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentSetIsAlarmedOp;

const DOC: &str = "Sets agent's status as alarmed (value = 1) or peaceful (value = 0).";

pub const OP_CODE: u32 = 1807;

pub const IDENT: &str = "agent_set_is_alarmed";

impl Operation for AgentSetIsAlarmedOp {
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
