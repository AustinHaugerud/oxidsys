use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PartyClearParticleSystemsOp;

const DOC: &str = "Removes all special visual effects from the party on the map.";

pub const OP_CODE: u32 = 1679;

pub const IDENT: &str = "party_clear_particle_systems";

impl Operation for PartyClearParticleSystemsOp {
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
            num_required: 1,
            num_optional: 0,
            param_docs: vec![make_param_doc("<party_id>", "")],
        }
    }
}
