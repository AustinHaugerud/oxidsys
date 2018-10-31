use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PartySetAiObjectOp;

const DOC: &str = "Sets another party as the object for current AI behavior (follow that party).";

pub const OP_CODE: u32 = 1641;

pub const IDENT: &str = "party_set_ai_object";

impl Operation for PartySetAiObjectOp {
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
                make_param_doc("<object_party_id>", ""),
            ],
        }
    }
}
