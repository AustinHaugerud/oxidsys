use language::operations::Operation;

pub struct EndCurrentBattleOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1307;

pub const IDENT: &str = "end_current_battle";

impl Operation for EndCurrentBattleOp {
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
