use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PartySetAiPatrolRadiusOp;

const DOC: &str = "Sets a radius for AI patrolling behavior.";

pub const OP_CODE: u32 = 1643;

pub const IDENT: &str = "party_set_ai_patrol_radius";

impl Operation for PartySetAiPatrolRadiusOp {
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
                make_param_doc("<party_id>", ""),
                make_param_doc("<radius_in_km>", ""),
            ],
        }
    }
}
