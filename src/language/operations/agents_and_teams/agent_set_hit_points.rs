use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentSetHitPointsOp;

const DOC : &str = "Sets new value for agent health. Optional last parameter determines whether the value is interpreted as actual health (absolute = 1) or relative percentile health (absolute = 0). Default is relative.";

pub const OP_CODE: u32 = 1721;

pub const IDENT: &str = "agent_set_hit_points";

impl Operation for AgentSetHitPointsOp {
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
