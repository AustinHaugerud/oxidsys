use language::operations::Operation;

pub struct ServerGetChangingGameTypeAllowedOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 476;

pub const IDENT: &str = "server_get_changing_game_type_allowed";

impl Operation for ServerGetChangingGameTypeAllowedOp {
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
