use language::operations::Operation;

pub struct StoreScriptParam2Op;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 22;

pub const IDENT: &str = "store_script_param_2";

impl Operation for StoreScriptParam2Op {
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
