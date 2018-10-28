use language::operations::Operation;

pub struct ProfileSetBannerIdOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 351;

pub const IDENT: &str = "profile_set_banner_id";

impl Operation for ProfileSetBannerIdOp {
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
