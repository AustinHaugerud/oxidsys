use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StoreAgentHitPointsOp;

const DOC : &str = "Retrieves current agent health. Optional last parameter determines whether actual health (absolute = 1) or relative percentile health (absolute = 0) is returned. Default is relative.";

pub const OP_CODE: u32 = 1720;

pub const IDENT: &str = "store_agent_hit_points";

impl Operation for StoreAgentHitPointsOp {
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
                make_param_doc("<destination>", ""),
                make_param_doc("<agent_id>", ""),
                make_param_doc("[absolute]", ""),
            ],
        }
    }
}
