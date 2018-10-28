use language::operations::Operation;

pub struct StoreProficiencyLevelOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2176;

pub const IDENT: &str = "store_proficiency_level";

impl Operation for StoreProficiencyLevelOp {
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
