use std::str::from_utf8_unchecked;

use std::result::Result as StandardResult;

use anchor_lang::prelude::*;
use solana_program::{
    account_info::AccountInfo,
    program::{invoke, invoke_signed},
    program_memory::sol_memcmp,
    program_pack::{IsInitialized, Pack},
    pubkey::{Pubkey, PUBKEY_BYTES},
    system_instruction,
};
use spl_associated_token_account::get_associated_token_address;

#[cfg(feature = "use-test-anti-bot-authority")]
use crate::constants::ANTI_BOT_DEV_AUTHORITY;

#[cfg(not(feature = "use-test-anti-bot-authority"))]
use crate::constants::ANTI_BOT_MAINNET_AUTHORITY;

use crate::{BuyerMerkleAllowlistProofData, CandyError, CandyMachine, CandyMachineData, MintPhase};

pub fn assert_initialized<T: Pack + IsInitialized>(account_info: &AccountInfo) -> Result<T> {
    let account: T = T::unpack_unchecked(&account_info.data.borrow())?;
    if !account.is_initialized() {
        Err(CandyError::Uninitialized.into())
    } else {
        Ok(account)
    }
}

pub fn cmp_pubkeys(a: &Pubkey, b: &Pubkey) -> bool {
    sol_memcmp(a.as_ref(), b.as_ref(), PUBKEY_BYTES) == 0
}

pub fn is_omni_minter<'info>(
    buyer: &Signer<'info>,
    candy_machine: &Account<'info, CandyMachine>,
) -> bool {
    candy_machine.data.omni_mint_wallets.contains(&buyer.key())
}

pub fn validate_mint_phase<'info>(
    buyer: &Signer<'info>,
    mint_phase: &MintPhase,
    candy_machine: &Account<'info, CandyMachine>,
    buyer_merkle_allowlist_proof_data: &Option<BuyerMerkleAllowlistProofData>,
) -> StandardResult<(), CandyError> {
    if is_omni_minter(buyer, candy_machine) && mint_phase != &MintPhase::Expired {
        return Ok(());
    }

    match mint_phase {
        MintPhase::Expired => Err(CandyError::CandyMachinePublicSaleEnded),
        MintPhase::Premint => {
            let error = if candy_machine.data.allowlist_sale_start_time.is_some() {
                CandyError::CandyMachineAllowlistSaleNotLive
            } else {
                CandyError::CandyMachinePublicSaleNotLive
            };
            Err(error)
        }
        MintPhase::Allowlist => {
            let allowlist_settings_present = buyer_merkle_allowlist_proof_data.is_some()
                || candy_machine.data.spl_token_allowlist_settings.is_some();

            if !allowlist_settings_present {
                Err(CandyError::CandyMachineAllowlistSaleNotLive)
            } else {
                Ok(())
            }
        }
        MintPhase::Public => Ok(()),
    }
}

pub fn validate_mint_phase_times(candy_machine_data: &CandyMachineData) -> Result<()> {
    let allowlist_sale_start_time = candy_machine_data.allowlist_sale_start_time;
    let public_sale_start_time = candy_machine_data.public_sale_start_time;
    let public_sale_end_time = candy_machine_data.public_sale_end_time;

    if public_sale_start_time >= public_sale_end_time {
        return Err(CandyError::CandyMachineInvalidMintPhases.into());
    }

    if let Some(allowlist_sale_start_time) = allowlist_sale_start_time {
        if allowlist_sale_start_time >= public_sale_start_time {
            return Err(CandyError::CandyMachineInvalidMintPhases.into());
        }
    }

    Ok(())
}

pub fn assert_owned_by(account: &AccountInfo, owner: &Pubkey) -> Result<()> {
    if !cmp_pubkeys(account.owner, owner) {
        Err(CandyError::IncorrectOwner.into())
    } else {
        Ok(())
    }
}
/// TokenTransferParams
pub struct TokenTransferParams<'a: 'b, 'b> {
    /// CHECK: account checked in CPI
    pub source: AccountInfo<'a>,
    /// CHECK: account checked in CPI
    pub destination: AccountInfo<'a>,
    pub amount: u64,
    /// CHECK: account checked in CPI
    pub authority: AccountInfo<'a>,
    /// authority_signer_seeds
    pub authority_signer_seeds: &'b [&'b [u8]],
    /// CHECK: account checked in CPI
    pub token_program: AccountInfo<'a>,
}

#[inline(always)]
pub fn spl_token_transfer(params: TokenTransferParams<'_, '_>) -> Result<()> {
    let TokenTransferParams {
        source,
        destination,
        authority,
        token_program,
        amount,
        authority_signer_seeds,
    } = params;

    let mut signer_seeds = vec![];
    if !authority_signer_seeds.is_empty() {
        signer_seeds.push(authority_signer_seeds)
    }

    let result = invoke_signed(
        &spl_token::instruction::transfer(
            token_program.key,
            source.key,
            destination.key,
            authority.key,
            &[],
            amount,
        )?,
        &[source, destination, authority, token_program],
        &signer_seeds,
    );

    result.map_err(|_| CandyError::TokenTransferFailed.into())
}

pub fn assert_is_ata(
    ata: &AccountInfo,
    wallet: &Pubkey,
    mint: &Pubkey,
) -> core::result::Result<spl_token::state::Account, ProgramError> {
    let ata_account = assert_is_token_account(ata, wallet, mint)?;
    assert_keys_equal(&get_associated_token_address(wallet, mint), ata.key)?;
    Ok(ata_account)
}

pub fn assert_is_token_account(
    token_account: &AccountInfo,
    wallet: &Pubkey,
    mint: &Pubkey,
) -> core::result::Result<spl_token::state::Account, ProgramError> {
    assert_owned_by(token_account, &spl_token::id())?;
    let ata_account: spl_token::state::Account = assert_initialized(token_account)?;
    assert_keys_equal(&ata_account.owner, wallet)?;
    assert_keys_equal(&ata_account.mint, mint)?;
    Ok(ata_account)
}

pub fn assert_keys_equal(key1: &Pubkey, key2: &Pubkey) -> Result<()> {
    if !cmp_pubkeys(key1, key2) {
        err!(CandyError::PublicKeyMismatch)
    } else {
        Ok(())
    }
}

pub fn is_feature_active(uuid: &str, feature_index: usize) -> bool {
    uuid.as_bytes()[feature_index] == b"1"[0]
}

/// TokenBurnParams
pub struct TokenBurnParams<'a: 'b, 'b> {
    /// mint
    /// CHECK: account checked in CPI
    pub mint: AccountInfo<'a>,
    /// source
    /// CHECK: account checked in CPI
    pub source: AccountInfo<'a>,
    /// amount
    pub amount: u64,
    /// authority
    /// CHECK: account checked in CPI
    pub authority: AccountInfo<'a>,
    /// authority_signer_seeds
    pub authority_signer_seeds: Option<&'b [&'b [u8]]>,
    /// token_program
    /// CHECK: account checked in CPI
    pub token_program: AccountInfo<'a>,
}

pub fn spl_token_burn(params: TokenBurnParams<'_, '_>) -> Result<()> {
    let TokenBurnParams {
        mint,
        source,
        authority,
        token_program,
        amount,
        authority_signer_seeds,
    } = params;
    let mut seeds: Vec<&[&[u8]]> = vec![];
    if let Some(seed) = authority_signer_seeds {
        seeds.push(seed);
    }
    let result = invoke_signed(
        &spl_token::instruction::burn(
            token_program.key,
            source.key,
            mint.key,
            authority.key,
            &[],
            amount,
        )?,
        &[source, mint, authority, token_program],
        seeds.as_slice(),
    );
    result.map_err(|_| CandyError::TokenBurnFailed.into())
}

// string is 6 bytes long, can be any valid utf8 char coming in.
// feature_index is between 0 and 5, inclusive. We set it to an array of utf8 "0"s first
pub fn set_feature_flag(uuid: &mut String, feature_index: usize) {
    let mut bytes: [u8; 6] = [b'0'; 6];
    uuid.bytes().enumerate().for_each(|(i, byte)| {
        if i == feature_index || byte == b'1' {
            bytes[i] = b'1';
        }
    });

    // unsafe is fine because we know for a fact that the array will only
    // contain valid UTF8 bytes since we fully ignore user inputted UUID and set
    // it to an array of only valid bytes (b'0') and then only modify the bytes in
    // that valid utf8 byte array to other valid utf8 characters (b'1')
    // This saves a bit of compute from the overhead of using the from_utf8 or
    // other similar methods that need to ensure that the bytes are valid
    unsafe {
        uuid.replace_range(.., from_utf8_unchecked(&bytes));
    }
}

// string is 6 bytes long, can be any valid utf8 char coming in.
// feature_index is between 0 and 5, inclusive. We set it to an array of utf8 "0"s first
pub fn remove_feature_flag(uuid: &mut String, feature_index: usize) {
    let mut bytes: [u8; 6] = [b'0'; 6];
    uuid.bytes().enumerate().for_each(|(i, byte)| {
        if i == feature_index {
            bytes[i] = b'0';
        } else if byte == b'1' {
            bytes[i] = b'1';
        }
    });

    // unsafe is fine because we know for a fact that the array will only
    // contain valid UTF8 bytes since we fully ignore user inputted UUID and set
    // it to an array of only valid bytes (b'0') and then only modify the bytes in
    // that valid utf8 byte array to other valid utf8 characters (b'1')
    // This saves a bit of compute from the overhead of using the from_utf8 or
    // other similar methods that need to ensure that the bytes are valid
    unsafe {
        uuid.replace_range(.., from_utf8_unchecked(&bytes));
    }
}

pub fn punish_bots<'a>(
    error: CandyError,
    bot_account: AccountInfo<'a>,
    payment_account: AccountInfo<'a>,
    system_program: AccountInfo<'a>,
    fee: u64,
) -> Result<()> {
    // Note: This needs to be consistent with auction house punish_bots log for tx parsing.
    let bot_tax_collected_error: Result<()> = Err(CandyError::BotTaxCollected.into());
    msg!(
        "{} {} BullisticCandyMachine botting is taxed at {:?} lamports.",
        bot_tax_collected_error.unwrap_err().to_string(),
        error.to_string(),
        fee
    );

    let final_fee = fee.min(bot_account.lamports());
    invoke(
        &system_instruction::transfer(bot_account.key, payment_account.key, final_fee),
        &[bot_account, payment_account, system_program],
    )?;
    Ok(())
}

// On non-mainnet environments we check against a less secure anti-bot authority
// because we include this keypair into our repos for testing convenience.
#[cfg(feature = "use-test-anti-bot-authority")]
pub fn assert_valid_bot_signer_authority(bot_signer_authority: &Pubkey) -> Result<()> {
    if cmp_pubkeys(bot_signer_authority, &ANTI_BOT_DEV_AUTHORITY) {
        return Ok(());
    }

    Err(CandyError::InvalidBotSignerAuthority.into())
}

#[cfg(not(feature = "use-test-anti-bot-authority"))]
pub fn assert_valid_bot_signer_authority(bot_signer_authority: &Pubkey) -> Result<()> {
    if cmp_pubkeys(bot_signer_authority, &ANTI_BOT_MAINNET_AUTHORITY) {
        return Ok(());
    }

    Err(CandyError::InvalidBotSignerAuthority.into())
}

pub fn make_ata<'a>(
    ata: AccountInfo<'a>,
    wallet: AccountInfo<'a>,
    mint: AccountInfo<'a>,
    fee_payer: AccountInfo<'a>,
    ata_program: AccountInfo<'a>,
    token_program: AccountInfo<'a>,
    system_program: AccountInfo<'a>,
    rent: AccountInfo<'a>,
    fee_payer_seeds: &[&[u8]],
) -> Result<()> {
    let as_arr = [fee_payer_seeds];
    let seeds: &[&[&[u8]]] = if fee_payer_seeds.is_empty() {
        &[]
    } else {
        &as_arr
    };

    invoke_signed(
        &spl_associated_token_account::instruction::create_associated_token_account(
            fee_payer.key,
            wallet.key,
            mint.key,
            token_program.key,
        ),
        &[
            ata,
            wallet,
            mint,
            fee_payer,
            ata_program,
            system_program,
            rent,
            token_program,
        ],
        seeds,
    )?;

    Ok(())
}

pub fn write_anchor_account_discriminator<T: AnchorDeserialize + AccountSerialize>(
    account: &UncheckedAccount,
) -> Result<()> {
    let mut data_ref: &mut [u8] = &mut account.try_borrow_mut_data()?;
    let anchor_account: T = AnchorDeserialize::deserialize(&mut &*data_ref)?;
    anchor_account.try_serialize(&mut data_ref)?;
    Ok(())
}

/// These functions deal with verification of Merkle trees (hash trees).
///
/// The initial implementation for Solidity based Ethereum contracts is here:
/// https://github.com/OpenZeppelin/openzeppelin-contracts/blob/v3.4.0/contracts/cryptography/MerkleProof.sol
///
/// That was then ported to Solana by the Saber Team:
/// https://github.com/saber-hq/merkle-distributor/blob/master/programs/merkle-distributor/src/merkle_proof.rs
///
/// The Saber code was then used directly in the Metaplex Gumdrop implementation
/// which served as the basis for the current bullistic-gumdrop program.
///
/// Returns true if a `leaf` can be proved to be a part of a Merkle tree
/// defined by `root`. For this, a `proof` must be provided, containing
/// sibling hashes on the branch from the leaf to the root of the tree. Each
/// pair of leaves and each pair of pre-images are assumed to be sorted.
pub fn verify_merkle_proof(proof: &[[u8; 32]], root: [u8; 32], leaf: [u8; 32]) -> bool {
    let mut computed_hash = leaf;
    for proof_element in proof.iter() {
        let proof_element = *proof_element;
        if computed_hash <= proof_element {
            // Hash(current computed hash + current element of the proof)
            computed_hash =
                solana_program::keccak::hashv(&[&[0x01], &computed_hash, &proof_element]).0;
        } else {
            // Hash(current element of the proof + current computed hash)
            computed_hash =
                solana_program::keccak::hashv(&[&[0x01], &proof_element, &computed_hash]).0;
        }
    }
    // Check if the computed hash (root) is equal to the provided root
    computed_hash == root
}

// We disallow enabling both allowlist types for a single candy machine.
pub fn validate_candy_machine_allowlist_state(data: &CandyMachineData) -> Result<()> {
    if data.spl_token_allowlist_settings.is_some() && !data.merkle_allowlist_root_list.is_empty() {
        return Err(CandyError::InvalidAllowlistSettings.into());
    }

    Ok(())
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::constants::COLLECTIONS_FEATURE_INDEX;

    #[test]
    fn feature_flag_working() {
        let mut uuid = String::from("ABCDEF");
        println!(
            "Should be 65: {}",
            uuid.as_bytes()[COLLECTIONS_FEATURE_INDEX]
        );

        uuid = String::from("01H333");
        println!("Should be 01H333: {}", uuid);
        set_feature_flag(&mut uuid, COLLECTIONS_FEATURE_INDEX + 1);
        assert!(is_feature_active(&uuid, COLLECTIONS_FEATURE_INDEX + 1));
        println!("Should be 010000: {}", uuid);
        remove_feature_flag(&mut uuid, COLLECTIONS_FEATURE_INDEX + 1);
        assert!(!is_feature_active(&uuid, COLLECTIONS_FEATURE_INDEX + 1));
        println!("Should be 000000: {}", uuid);

        set_feature_flag(&mut uuid, COLLECTIONS_FEATURE_INDEX);
        assert!(is_feature_active(&uuid, COLLECTIONS_FEATURE_INDEX));
        println!("Should be 100000: {}", uuid);
        remove_feature_flag(&mut uuid, COLLECTIONS_FEATURE_INDEX);
        assert!(!is_feature_active(&uuid, COLLECTIONS_FEATURE_INDEX));
        println!("Should be 000000: {}", uuid);
    }

    #[test]
    fn check_keys_equal() {
        let key1 = Pubkey::new_unique();
        assert!(cmp_pubkeys(&key1, &key1));
    }

    #[test]
    fn check_keys_not_equal() {
        let key1 = Pubkey::new_unique();
        let key2 = Pubkey::new_unique();
        assert!(!cmp_pubkeys(&key1, &key2));
    }
}
