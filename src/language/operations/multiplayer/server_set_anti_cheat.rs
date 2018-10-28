use language::operations::Operation;

pub struct ServerSetAntiCheatOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 477;

pub const IDENT: &str = "server_set_anti_cheat";

impl Operation for ServerSetAntiCheatOp {
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
