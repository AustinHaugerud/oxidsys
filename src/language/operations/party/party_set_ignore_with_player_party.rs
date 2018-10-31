use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PartySetIgnoreWithPlayerPartyOp;

const DOC: &str = "Version 1.161+. Effects uncertain. 4research";

pub const OP_CODE: u32 = 1648;

pub const IDENT: &str = "party_set_ignore_with_player_party";

impl Operation for PartySetIgnoreWithPlayerPartyOp {
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
                make_param_doc("<value>", ""),
            ],
        }
    }
}
