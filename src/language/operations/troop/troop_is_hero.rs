use language::operations::Operation;

pub struct TroopIsHeroOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1507;

pub const IDENT: &str = "troop_is_hero";

impl Operation for TroopIsHeroOp {
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
