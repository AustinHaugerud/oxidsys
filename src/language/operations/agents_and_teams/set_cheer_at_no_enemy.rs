use language::operations::Operation;

pub struct SetCheerAtNoEnemyOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2379;

pub const IDENT: &str = "set_cheer_at_no_enemy";

impl Operation for SetCheerAtNoEnemyOp {
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
