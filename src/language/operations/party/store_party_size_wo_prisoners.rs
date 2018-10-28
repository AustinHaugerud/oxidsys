use language::operations::Operation;

pub struct StorePartySizeWoPrisonersOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2157;

pub const IDENT: &str = "store_party_size_wo_prisoners";

impl Operation for StorePartySizeWoPrisonersOp {
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
