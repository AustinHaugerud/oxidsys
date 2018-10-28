use language::operations::Operation;

pub struct PropInstanceRefillHitPointsOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1870;

pub const IDENT: &str = "prop_instance_refill_hit_points";

impl Operation for PropInstanceRefillHitPointsOp {
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
