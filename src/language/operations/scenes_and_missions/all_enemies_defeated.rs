use language::operations::Operation;

pub struct AllEnemiesDefeatedOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1003;

pub const IDENT: &str = "all_enemies_defeated";

impl Operation for AllEnemiesDefeatedOp {
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
