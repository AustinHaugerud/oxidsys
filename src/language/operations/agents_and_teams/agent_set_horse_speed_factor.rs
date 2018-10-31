use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentSetHorseSpeedFactorOp;

const DOC : &str = "Multiplies agent's horse speed (and maneuverability?) by the specified percentile value (using 100 will make the horse). Note that this is called on the rider, not on the horse! Supposedly will persist even if the agent changes horses. 4research.";

pub const OP_CODE: u32 = 1734;

pub const IDENT: &str = "agent_set_horse_speed_factor";

impl Operation for AgentSetHorseSpeedFactorOp {
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
            num_required: 1,
            num_optional: 0,
            param_docs: vec![make_param_doc("<agent_id>", "")],
        }
    }
}
