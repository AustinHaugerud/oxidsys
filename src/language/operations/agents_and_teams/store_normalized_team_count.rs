use language::operations::Operation;

pub struct StoreNormalizedTeamCountOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2385;

pub const IDENT: &str = "store_normalized_team_count";

impl Operation for StoreNormalizedTeamCountOp {
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
