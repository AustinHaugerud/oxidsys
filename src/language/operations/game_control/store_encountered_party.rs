use language::operations::Operation;

pub struct StoreEncounteredPartyOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2202;

pub const IDENT: &str = "store_encountered_party";

impl Operation for StoreEncounteredPartyOp {
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
