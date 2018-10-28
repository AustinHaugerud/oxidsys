use language::operations::Operation;

pub struct TryForPartiesOp;

const DOC: &str = r#"
Iterate over all parties on the map.
Format: try_for_parties <iterator>;
"#;

pub const OP_CODE: u16 = 11;

pub const IDENT: &str = "try_for_parties";

impl Operation for TryForPartiesOp {
    fn op_code(&self) -> u16 {
        unimplemented!()
    }

    fn documentation(&self) -> &'static str {
        DOC
    }

    fn identifier(&self) -> &'static str {
        IDENT
    }
}
