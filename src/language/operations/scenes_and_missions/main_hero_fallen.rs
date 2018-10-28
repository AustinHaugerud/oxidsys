use language::operations::Operation;

pub struct MainHeroFallenOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1006;

pub const IDENT: &str = "main_hero_fallen";

impl Operation for MainHeroFallenOp {
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
