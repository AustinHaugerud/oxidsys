use language::operations::Operation;

pub struct StrStoreTroopNamePluralOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2323;

pub const IDENT: &str = "str_store_troop_name_plural";

impl Operation for StrStoreTroopNamePluralOp {
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
