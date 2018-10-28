use language::operations::Operation;

pub struct SetStartupSunLightOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2390;

pub const IDENT: &str = "set_startup_sun_light";

impl Operation for SetStartupSunLightOp {
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
