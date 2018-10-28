use language::operations::Operation;

pub struct GetAverageGameDifficultyOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 990;

pub const IDENT: &str = "get_average_game_difficulty";

impl Operation for GetAverageGameDifficultyOp {
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
