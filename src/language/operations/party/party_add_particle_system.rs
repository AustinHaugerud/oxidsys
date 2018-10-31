use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PartyAddParticleSystemOp;

const DOC : &str = "Appends some special visual effects to the party on the map. Used in Native to add fire and smoke over villages.";

pub const OP_CODE: u32 = 1678;

pub const IDENT: &str = "party_add_particle_system";

impl Operation for PartyAddParticleSystemOp {
    fn op_code(&self) -> u32 {
        OP_CODE
    }

    fn documentation(&self) -> &'static str {
        DOC
    }

    fn identifier(&self) -> &'static str {
        IDENT
    }

    fn param_info(&self) -> ParamInfo {
        ParamInfo {
            num_required: 2,
            num_optional: 0,
            param_docs: vec![
                make_param_doc("<party_id>", ""),
                make_param_doc("<particle_system_id>", ""),
            ],
        }
    }
}
