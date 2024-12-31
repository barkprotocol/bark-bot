let ix_keys = vec![
    AccountMeta::new(Pubkey::new_unique(), false),
    AccountMeta::new(Pubkey::new_unique(), true),
];
let keys_unique = vec![
    AccountMeta::new(Pubkey::new_unique(), false),
    AccountMeta::new(Pubkey::new_unique(), true),
    AccountMeta::new(Pubkey::new_unique(), false),
];

let indexes = find_key_indexes(&ix_keys, &keys_unique);

for (i, index) in indexes.iter().enumerate() {
    match index {
        Some(idx) => println!("Found index for key {}: {}", i, idx),
        None => println!("Key {} not found", i),
    }
}
