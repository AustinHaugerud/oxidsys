use language::operations::Operation;

pub struct StoreRandomTroopToCaptureOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2252;

pub const IDENT: &str = "store_random_troop_to_capture";

impl Operation for StoreRandomTroopToCaptureOp {
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
