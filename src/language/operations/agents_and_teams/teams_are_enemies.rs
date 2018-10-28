use language::operations::Operation;

pub struct TeamsAreEnemiesOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1788;

pub const IDENT: &str = "teams_are_enemies";

impl Operation for TeamsAreEnemiesOp {
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
