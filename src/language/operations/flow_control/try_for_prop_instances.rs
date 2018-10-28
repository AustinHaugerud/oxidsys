use language::operations::Operation;

pub struct TryForPropInstancesOp;

const DOC: &str = r#"
Iterate all prop instances in scene. Optionally specify a prop id.
Format: try_for_prop_instances <iterator> [prop_id];
"#;

pub const OP_CODE: u16 = 16;

pub const IDENT: &str = "try_for_prop_instances";

impl Operation for TryForPropInstancesOp {
    fn op_code(&self) -> u16 {
        OP_CODE
    }

    fn documentation(&self) -> &str {
        DOC
    }

    fn identifier(&self) -> &str {
        IDENT
    }
}
