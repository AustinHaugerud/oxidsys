use language::operations::Operation;

pub struct StrStoreInfoPageNameLinkOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2345;

pub const IDENT: &str = "str_store_info_page_name_link";

impl Operation for StrStoreInfoPageNameLinkOp {
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
