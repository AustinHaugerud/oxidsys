use language::operations::Operation;

pub struct UnlockAchievementOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 372;

pub const IDENT: &str = "unlock_achievement";

impl Operation for UnlockAchievementOp {
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
