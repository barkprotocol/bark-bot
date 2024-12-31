use anchor_lang::prelude::AccountMeta;
use solana_sdk::pubkey::Pubkey;
use std::collections::HashMap;

pub fn find_unique_account_metas_map(
    account_metas: &Vec<AccountMeta>,
) -> HashMap<Pubkey, AccountMeta> {
    let mut unique_account_metas_map: HashMap<Pubkey, AccountMeta> = HashMap::new();

    for account_meta in account_metas {
        match unique_account_metas_map.get(&account_meta.pubkey) {
            Some(prev_account_meta) => {
                // Skip insertion if we already have the same pubkey and writable flag
                if prev_account_meta.is_writable == account_meta.is_writable {
                    continue;
                }
            }
            None => {} // No existing entry, so proceed with insertion
        }

        // Insert or overwrite with the new AccountMeta
        unique_account_metas_map.insert(account_meta.pubkey, account_meta.clone());
    }

    unique_account_metas_map
}
