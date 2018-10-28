use language::operations::Operation;

pub struct StrStoreFactionNameLinkOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2343;

pub const IDENT: &str = "str_store_faction_name_link";

impl Operation for StrStoreFactionNameLinkOp {
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
