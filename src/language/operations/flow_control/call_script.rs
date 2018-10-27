use language::operations::Operation;

pub struct CallScriptOp;

const DOC : &str = r#"
Calls specified script with or without parameters.
Format: call_script <script_id>, [<script_parma>...];
"#;

pub const OP_CODE : u16 = 1;

impl Operation for CallScriptOp {
    fn op_code(&self) -> u16 {
        OP_CODE
    }

    fn documentation(&self) -> &str {
        DOC
    }
}
