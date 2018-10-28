use language::operations::Operation;

pub struct PartyLeaveCurBattleOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1666;

pub const IDENT: &str = "party_leave_cur_battle";

impl Operation for PartyLeaveCurBattleOp {
    fn op_code(&self) -> u32 {
        OP_CODE
    }

    fn documentation(&self) -> &'static str {
        DOC
    }

    fn identifier(&self) -> &'static str {
        IDENT
    }
}
