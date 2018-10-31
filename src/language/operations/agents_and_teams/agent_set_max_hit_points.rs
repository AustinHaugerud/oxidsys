use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentSetMaxHitPointsOp;

const DOC : &str = "Version 1.153+. Changes agent's max hit points. Optional flag [absolute] determines if <value> is an absolute number of his points, or relative percentage (0..1000) of default value. Treated as percentage by default.";

pub const OP_CODE: u32 = 2090;

pub const IDENT: &str = "agent_set_max_hit_points";

impl Operation for AgentSetMaxHitPointsOp {
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
            num_optional: 1,
            param_docs: vec![
                make_param_doc("<agent_id>", ""),
                make_param_doc("<value>", ""),
                make_param_doc("[absolute]", ""),
            ],
        }
    }
}
