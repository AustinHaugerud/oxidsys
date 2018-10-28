use language::operations::Operation;

pub struct ServerGetRenamingServerAllowedOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 475;

pub const IDENT: &str = "server_get_renaming_server_allowed";

impl Operation for ServerGetRenamingServerAllowedOp {
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
