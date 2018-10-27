
fn extract_block_contents(input : &str) -> &str {
    &input[1..input.len()-1]
}

#[cfg(test)]
mod tests {

    use super::extract_block_contents;

    #[test]
    fn test_extracts_block() {
        let block_source = "{\ncontent\n}";
        let content = extract_block_contents(block_source);
        assert_eq!(content, "\ncontent\n");
    }
}
