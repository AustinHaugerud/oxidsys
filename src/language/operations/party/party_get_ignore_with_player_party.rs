use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PartyGetIgnoreWithPlayerPartyOp;

const DOC : &str = "Version 1.161+. Effects uncertain. Documented official syntax is suspicious and probably incorrect. 4research";

pub const OP_CODE: u32 = 1649;

pub const IDENT: &str = "party_get_ignore_with_player_party";

impl Operation for PartyGetIgnoreWithPlayerPartyOp {
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
            param_docs: vec![make_param_doc("<party_id>", "")],
        }
    }
}
