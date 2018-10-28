use language::operations::Operation;

pub struct GetAchievementStatOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 370;

pub const IDENT: &str = "get_achievement_stat";

impl Operation for GetAchievementStatOp {
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
