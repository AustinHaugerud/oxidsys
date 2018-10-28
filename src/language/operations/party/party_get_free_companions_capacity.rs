use language::operations::Operation;

pub struct PartyGetFreeCompanionsCapacityOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1633;

pub const IDENT: &str = "party_get_free_companions_capacity";

impl Operation for PartyGetFreeCompanionsCapacityOp {
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
