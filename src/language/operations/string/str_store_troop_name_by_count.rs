use language::operations::Operation;

pub struct StrStoreTroopNameByCountOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2324;

pub const IDENT: &str = "str_store_troop_name_by_count";

impl Operation for StrStoreTroopNameByCountOp {
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
