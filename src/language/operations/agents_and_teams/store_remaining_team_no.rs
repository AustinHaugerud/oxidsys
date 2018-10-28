use language::operations::Operation;

pub struct StoreRemainingTeamNoOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2360;

pub const IDENT: &str = "store_remaining_team_no";

impl Operation for StoreRemainingTeamNoOp {
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
