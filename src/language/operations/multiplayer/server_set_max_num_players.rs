use language::operations::Operation;

pub struct ServerSetMaxNumPlayersOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 491;

pub const IDENT: &str = "server_set_max_num_players";

impl Operation for ServerSetMaxNumPlayersOp {
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
