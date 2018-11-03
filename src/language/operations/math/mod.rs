use language::operations::{Operation};
pub mod assign;
pub mod conditional;
pub mod fixed_point;
pub mod rng;
pub mod store_acos;
pub mod store_add;
pub mod store_and;
pub mod store_asin;
pub mod store_atan;
pub mod store_atan2;
pub mod store_cos;
pub mod store_div;
pub mod store_mod;
pub mod store_mul;
pub mod store_or;
pub mod store_pow;
pub mod store_sin;
pub mod store_sqrt;
pub mod store_sub;
pub mod store_tan;
pub mod val_abs;
pub mod val_add;
pub mod val_and;
pub mod val_clamp;
pub mod val_div;
pub mod val_lshift;
pub mod val_max;
pub mod val_min;
pub mod val_mod;
pub mod val_mul;
pub mod val_or;
pub mod val_rshift;
pub mod val_sub;

pub fn load_operands() -> Vec<Box<Operation>> {
    let mut result: Vec<Box<Operation>> = vec![];
    result.push(Box::new(assign::AssignOp {}));
    result.push(Box::new(store_acos::StoreAcosOp {}));
    result.push(Box::new(store_add::StoreAddOp {}));
    result.push(Box::new(store_and::StoreAndOp {}));
    result.push(Box::new(store_asin::StoreAsinOp {}));
    result.push(Box::new(store_atan::StoreAtanOp {}));
    result.push(Box::new(store_atan2::StoreAtan2Op {}));
    result.push(Box::new(store_cos::StoreCosOp {}));
    result.push(Box::new(store_div::StoreDivOp {}));
    result.push(Box::new(store_mod::StoreModOp {}));
    result.push(Box::new(store_mul::StoreMulOp {}));
    result.push(Box::new(store_or::StoreOrOp {}));
    result.push(Box::new(store_pow::StorePowOp {}));
    result.push(Box::new(store_sin::StoreSinOp {}));
    result.push(Box::new(store_sqrt::StoreSqrtOp {}));
    result.push(Box::new(store_sub::StoreSubOp {}));
    result.push(Box::new(store_tan::StoreTanOp {}));
    result.push(Box::new(val_abs::ValAbsOp {}));
    result.push(Box::new(val_add::ValAddOp {}));
    result.push(Box::new(val_and::ValAndOp {}));
    result.push(Box::new(val_clamp::ValClampOp {}));
    result.push(Box::new(val_div::ValDivOp {}));
    result.push(Box::new(val_lshift::ValLshiftOp {}));
    result.push(Box::new(val_max::ValMaxOp {}));
    result.push(Box::new(val_min::ValMinOp {}));
    result.push(Box::new(val_mod::ValModOp {}));
    result.push(Box::new(val_mul::ValMulOp {}));
    result.push(Box::new(val_or::ValOrOp {}));
    result.push(Box::new(val_rshift::ValRshiftOp {}));
    result.push(Box::new(val_sub::ValSubOp {}));
    result.extend(conditional::load_operands());
    result.extend(fixed_point::load_operands());
    result.extend(rng::load_operands());
    result
}
