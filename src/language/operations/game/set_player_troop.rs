use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct SetPlayerTroopOp;

const DOC : &str = "Changes the troop player controls. Generally used in quick-battle scenarios to give player a predefined character.";

pub const OP_CODE: u32 = 47;

pub const IDENT: &str = "set_player_troop";

impl Operation for SetPlayerTroopOp {
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
            param_docs: vec![make_param_doc("<troop_id>", "")],
        }
    }
}
