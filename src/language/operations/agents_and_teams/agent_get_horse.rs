use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentGetHorseOp;

const DOC : &str = "Retrieves the reference to the horse agent that the specified agent is riding, or -1 if he's not riding a horse (or is a horse himself).";

pub const OP_CODE: u32 = 1714;

pub const IDENT: &str = "agent_get_horse";

impl Operation for AgentGetHorseOp {
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
                make_param_doc("<destination>", ""),
                make_param_doc("<agent_id>", ""),
            ],
        }
    }
}
