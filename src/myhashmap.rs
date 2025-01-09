use std::collections::HashMap;

pub fn test_hashmap_basic() {
    let mut stock_ticker:HashMap<String, u32>= HashMap::new();
    
    println!("{}", stock_ticker.len());
    println!("{}", stock_ticker.is_empty());

    stock_ticker.insert("Nvidia".to_string(), 1345);
    stock_ticker.insert("Aapl".to_string(), 234);

    println!("{:#?}", stock_ticker)
}