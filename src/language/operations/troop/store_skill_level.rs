use language::operations::Operation;

pub struct StoreSkillLevelOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2170;

pub const IDENT: &str = "store_skill_level";

impl Operation for StoreSkillLevelOp {
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
