use language::operations::Operation;

pub struct ServerGetAntiCheatOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 499;

pub const IDENT: &str = "server_get_anti_cheat";

impl Operation for ServerGetAntiCheatOp {
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
