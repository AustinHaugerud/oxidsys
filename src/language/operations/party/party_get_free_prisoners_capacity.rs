use language::operations::Operation;

pub struct PartyGetFreePrisonersCapacityOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1634;

pub const IDENT: &str = "party_get_free_prisoners_capacity";

impl Operation for PartyGetFreePrisonersCapacityOp {
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
