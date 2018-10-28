use language::operations::Operation;

pub struct SetSpawnRadiusOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1103;

pub const IDENT: &str = "set_spawn_radius";

impl Operation for SetSpawnRadiusOp {
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
