use language::operations::Operation;

pub struct GetStartupGroundAmbientLightOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2396;

pub const IDENT: &str = "get_startup_ground_ambient_light";

impl Operation for GetStartupGroundAmbientLightOp {
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
