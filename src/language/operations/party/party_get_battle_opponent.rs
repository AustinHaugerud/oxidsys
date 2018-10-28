use language::operations::Operation;

pub struct PartyGetBattleOpponentOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1680;

pub const IDENT: &str = "party_get_battle_opponent";

impl Operation for PartyGetBattleOpponentOp {
    fn op_code(&self) -> u16 {
        OP_CODE
    }

    fn documentation(&self) -> &'static str {
        DOC
    }

    fn identifier(&self) -> &'static str {
        IDENT
    }
}
