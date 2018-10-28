use language::operations::Operation;

pub struct StoreTimeOfDayOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2271;

pub const IDENT: &str = "store_time_of_day";

impl Operation for StoreTimeOfDayOp {
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
