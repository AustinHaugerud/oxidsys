use language::operations::Operation;

pub struct StrStoreQuestNameOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2336;

pub const IDENT: &str = "str_store_quest_name";

impl Operation for StrStoreQuestNameOp {
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
