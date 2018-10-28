use language::operations::Operation;

pub struct NumActiveTeamsLeOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1005;

pub const IDENT: &str = "num_active_teams_le";

impl Operation for NumActiveTeamsLeOp {
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
