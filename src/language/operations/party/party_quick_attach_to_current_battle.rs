use language::operations::Operation;

pub struct PartyQuickAttachToCurrentBattleOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1663;

pub const IDENT: &str = "party_quick_attach_to_current_battle";

impl Operation for PartyQuickAttachToCurrentBattleOp {
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
