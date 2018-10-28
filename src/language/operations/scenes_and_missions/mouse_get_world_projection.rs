use language::operations::Operation;

pub struct MouseGetWorldProjectionOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 751;

pub const IDENT: &str = "mouse_get_world_projection";

impl Operation for MouseGetWorldProjectionOp {
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
