use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentGetSpeedOp;

const DOC : &str = "Retrieves agent speed to (X,Y) coordinates of the position register. What do these mean - speed by world axis?";

pub const OP_CODE: u32 = 1689;

pub const IDENT: &str = "agent_get_speed";

impl Operation for AgentGetSpeedOp {
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
