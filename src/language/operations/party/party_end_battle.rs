use language::operations::Operation;

pub struct PartyEndBattleOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 108;

pub const IDENT: &str = "party_end_battle";

impl Operation for PartyEndBattleOp {
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
