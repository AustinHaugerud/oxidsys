use language::operations::Operation;

pub struct LeaveEncounterOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1301;

pub const IDENT: &str = "leave_encounter";

impl Operation for LeaveEncounterOp {
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
