use language::operations::Operation;

pub struct PositionSetZToGroundLevelOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 791;

pub const IDENT: &str = "position_set_z_to_ground_level";

impl Operation for PositionSetZToGroundLevelOp {
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
