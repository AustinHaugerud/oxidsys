use language::operations::Operation;

pub struct StoreCurrentDayOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2272;

pub const IDENT: &str = "store_current_day";

impl Operation for StoreCurrentDayOp {
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
