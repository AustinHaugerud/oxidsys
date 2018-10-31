use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentGetScriptedDestinationOp;

const DOC: &str =
    "Retrieves the position which is defined as agent's scripted destination, if any.";

pub const OP_CODE: u32 = 1731;

pub const IDENT: &str = "agent_get_scripted_destination";

impl Operation for AgentGetScriptedDestinationOp {
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
                make_param_doc("<position>", ""),
                make_param_doc("<agent_id>", ""),
            ],
        }
    }
}
