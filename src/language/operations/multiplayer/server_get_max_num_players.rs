use language::operations::Operation;

pub struct ServerGetMaxNumPlayersOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 490;

pub const IDENT: &str = "server_get_max_num_players";

impl Operation for ServerGetMaxNumPlayersOp {
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
