use language::operations::Operation;

pub struct SetFogDistanceOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1798;

pub const IDENT: &str = "set_fog_distance";

impl Operation for SetFogDistanceOp {
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
