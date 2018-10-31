use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PartySetAiTargetPositionOp;

const DOC : &str = "Sets a specific world map position as the object for current AI behavior (travel to that point).";

pub const OP_CODE: u32 = 1642;

pub const IDENT: &str = "party_set_ai_target_position";

impl Operation for PartySetAiTargetPositionOp {
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
                make_param_doc("<position>", ""),
            ],
        }
    }
}
