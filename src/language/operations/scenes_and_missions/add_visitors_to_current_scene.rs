use language::operations::Operation;

pub struct AddVisitorsToCurrentSceneOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1265;

pub const IDENT: &str = "add_visitors_to_current_scene";

impl Operation for AddVisitorsToCurrentSceneOp {
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
