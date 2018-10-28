use language::operations::Operation;

pub struct MultiplayerMakeEveryoneEnemyOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 420;

pub const IDENT: &str = "multiplayer_make_everyone_enemy";

impl Operation for MultiplayerMakeEveryoneEnemyOp {
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
