use language::operations::Operation;

pub struct InflictCasualtiesToPartyGroupOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1697;

pub const IDENT: &str = "inflict_casualties_to_party_group";

impl Operation for InflictCasualtiesToPartyGroupOp {
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
