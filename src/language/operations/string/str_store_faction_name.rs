use language::operations::Operation;

pub struct StrStoreFactionNameOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2335;

pub const IDENT: &str = "str_store_faction_name";

impl Operation for StrStoreFactionNameOp {
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
