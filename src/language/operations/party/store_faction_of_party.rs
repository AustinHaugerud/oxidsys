use language::operations::Operation;

pub struct StoreFactionOfPartyOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2204;

pub const IDENT: &str = "store_faction_of_party";

impl Operation for StoreFactionOfPartyOp {
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
