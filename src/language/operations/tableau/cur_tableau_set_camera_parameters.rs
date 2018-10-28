use language::operations::Operation;

pub struct CurTableauSetCameraParametersOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1989;

pub const IDENT: &str = "cur_tableau_set_camera_parameters";

impl Operation for CurTableauSetCameraParametersOp {
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
