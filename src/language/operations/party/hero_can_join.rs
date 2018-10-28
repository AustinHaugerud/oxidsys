use language::operations::Operation;

pub struct HeroCanJoinOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 101;

pub const IDENT: &str = "hero_can_join";

impl Operation for HeroCanJoinOp {
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
