use language::operations::Operation;

pub struct StoreDistanceToPartyFromPartyOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2281;

pub const IDENT: &str = "store_distance_to_party_from_party";

impl Operation for StoreDistanceToPartyFromPartyOp {
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
