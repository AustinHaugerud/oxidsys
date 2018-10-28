use language::operations::Operation;

pub struct StoreQuestNumberOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2261;

pub const IDENT: &str = "store_quest_number";

impl Operation for StoreQuestNumberOp {
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
