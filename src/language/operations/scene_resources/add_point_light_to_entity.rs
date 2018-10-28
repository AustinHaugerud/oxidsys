use language::operations::Operation;

pub struct AddPointLightToEntityOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1961;

pub const IDENT: &str = "add_point_light_to_entity";

impl Operation for AddPointLightToEntityOp {
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
