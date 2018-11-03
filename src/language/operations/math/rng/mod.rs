use language::operations::Operation;
pub mod shuffle_range;
pub mod store_random;
pub mod store_random_in_range;

pub fn load_operands() -> Vec<Box<Operation>> {
    let mut result: Vec<Box<Operation>> = vec![];
    result.push(Box::new(shuffle_range::ShuffleRangeOp {}));
    result.push(Box::new(store_random::StoreRandomOp {}));
    result.push(Box::new(store_random_in_range::StoreRandomInRangeOp {}));
    result
}
