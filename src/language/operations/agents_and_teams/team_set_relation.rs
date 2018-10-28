use language::operations::Operation;

pub struct TeamSetRelationOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1796;

pub const IDENT: &str = "team_set_relation";

impl Operation for TeamSetRelationOp {
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
