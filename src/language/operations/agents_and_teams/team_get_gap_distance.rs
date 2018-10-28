use language::operations::Operation;

pub struct TeamGetGapDistanceOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1828;

pub const IDENT: &str = "team_get_gap_distance";

impl Operation for TeamGetGapDistanceOp {
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
