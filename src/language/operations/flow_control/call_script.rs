use language::operations::Operation;

pub struct CallScriptOp;

const DOC: &str = r#"
Calls specified script with or without parameters.
Format: call_script <script_id>, [<script_parma>...];
"#;

pub const OP_CODE: u16 = 1;

pub const IDENT: &str = "call_script";

impl Operation for CallScriptOp {
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
