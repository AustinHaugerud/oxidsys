use language::operations::Operation;

pub struct StoreRandomPartyOfTemplateOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2311;

pub const IDENT: &str = "store_random_party_of_template";

impl Operation for StoreRandomPartyOfTemplateOp {
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
