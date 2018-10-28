use language::operations::Operation;

pub struct FinishPartyBattleModeOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1019;

pub const IDENT: &str = "finish_party_battle_mode";

impl Operation for FinishPartyBattleModeOp {
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
