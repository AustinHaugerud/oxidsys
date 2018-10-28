use language::operations::Operation;

pub struct PartyGetSkillLevelOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1685;

pub const IDENT: &str = "party_get_skill_level";

impl Operation for PartyGetSkillLevelOp {
    fn op_code(&self) -> u32 {
        OP_CODE
    }

    fn documentation(&self) -> &'static str {
        DOC
    }

    fn identifier(&self) -> &'static str {
        IDENT
    }
}
