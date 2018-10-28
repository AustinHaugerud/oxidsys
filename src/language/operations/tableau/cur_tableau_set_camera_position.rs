use language::operations::Operation;

pub struct CurTableauSetCameraPositionOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1988;

pub const IDENT: &str = "cur_tableau_set_camera_position";

impl Operation for CurTableauSetCameraPositionOp {
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
