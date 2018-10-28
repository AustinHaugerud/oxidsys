use language::operations::Operation;

pub struct StrStoreQuestNameLinkOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2344;

pub const IDENT: &str = "str_store_quest_name_link";

impl Operation for StrStoreQuestNameLinkOp {
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
