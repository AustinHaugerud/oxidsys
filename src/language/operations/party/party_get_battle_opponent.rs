use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PartyGetBattleOpponentOp;

const DOC : &str = "When a party is engaged in battle with another party, returns it's opponent party. Otherwise returns -1.";

pub const OP_CODE: u32 = 1680;

pub const IDENT: &str = "party_get_battle_opponent";

impl Operation for PartyGetBattleOpponentOp {
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
                make_param_doc("<destination>", ""),
                make_param_doc("<party_id>", ""),
            ],
        }
    }
}
