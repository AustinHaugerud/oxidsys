use language::operations::Operation;

pub struct GetStartupSunLightOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2394;

pub const IDENT: &str = "get_startup_sun_light";

impl Operation for GetStartupSunLightOp {
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
