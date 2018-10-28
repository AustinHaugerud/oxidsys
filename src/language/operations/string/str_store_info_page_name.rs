use language::operations::Operation;

pub struct StrStoreInfoPageNameOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2337;

pub const IDENT: &str = "str_store_info_page_name";

impl Operation for StrStoreInfoPageNameOp {
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
