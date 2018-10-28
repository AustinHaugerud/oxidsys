use language::operations::Operation;

pub struct SetPartyCreationRandomLimitsOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1080;

pub const IDENT: &str = "set_party_creation_random_limits";

impl Operation for SetPartyCreationRandomLimitsOp {
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
