use language::operations::Operation;

pub struct SetPartyBattleModeOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1020;

pub const IDENT: &str = "set_party_battle_mode";

impl Operation for SetPartyBattleModeOp {
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
