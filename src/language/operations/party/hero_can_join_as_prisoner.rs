use language::operations::Operation;

pub struct HeroCanJoinAsPrisonerOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 102;

pub const IDENT: &str = "hero_can_join_as_prisoner";

impl Operation for HeroCanJoinAsPrisonerOp {
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
