use language::operations::Operation;

pub struct GetStartupAmbientLightOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2395;

pub const IDENT: &str = "get_startup_ambient_light";

impl Operation for GetStartupAmbientLightOp {
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
