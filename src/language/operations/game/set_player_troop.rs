use language::operations::Operation;

pub struct SetPlayerTroopOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 47;

pub const IDENT: &str = "set_player_troop";

impl Operation for SetPlayerTroopOp {
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
