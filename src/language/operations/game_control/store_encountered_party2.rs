use language::operations::Operation;

pub struct StoreEncounteredParty2Op;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2203;

pub const IDENT: &str = "store_encountered_party2";

impl Operation for StoreEncounteredParty2Op {
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
