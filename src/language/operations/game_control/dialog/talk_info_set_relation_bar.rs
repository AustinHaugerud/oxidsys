use language::operations::Operation;

pub struct TalkInfoSetRelationBarOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2021;

pub const IDENT: &str = "talk_info_set_relation_bar";

impl Operation for TalkInfoSetRelationBarOp {
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
