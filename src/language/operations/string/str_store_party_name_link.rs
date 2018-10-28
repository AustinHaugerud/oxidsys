use language::operations::Operation;

pub struct StrStorePartyNameLinkOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2342;

pub const IDENT: &str = "str_store_party_name_link";

impl Operation for StrStorePartyNameLinkOp {
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
