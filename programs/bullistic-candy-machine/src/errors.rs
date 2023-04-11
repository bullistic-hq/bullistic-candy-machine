use anchor_lang::prelude::*;

#[error_code]
#[derive(EnumIndex, IndexEnum)]
pub enum CandyError {
    #[msg("Account does not have correct owner!")]
    IncorrectOwner = 2000,
    #[msg("Account is not initialized!")]
    Uninitialized,
    #[msg("Mint Mismatch!")]
    MintMismatch,
    #[msg("Index greater than length!")]
    IndexGreaterThanLength,
    #[msg("Numerical overflow error!")]
    NumericalOverflowError,
    #[msg("Can only provide up to 4 creators to candy machine (because candy machine is one)!")]
    TooManyCreators,
    #[msg("Uuid must be exactly of 6 length.")]
    UuidMustBeExactly6Length,
    #[msg("Not enough tokens to pay for this minting.")]
    NotEnoughTokens,
    #[msg("Not enough SOL to pay for this minting.")]
    NotEnoughSOL,
    #[msg("Token transfer failed.")]
    TokenTransferFailed,
    #[msg("Candy machine is empty!")]
    CandyMachineEmpty,
    #[msg("Candy machine public sale is not live!")]
    CandyMachinePublicSaleNotLive,
    #[msg("Configs that are using hidden uris do not have config lines, they have a single hash representing hashed order.")]
    HiddenSettingsConfigsDoNotHaveConfigLines,
    #[msg("Cannot change number of lines unless is a hidden config.")]
    CannotChangeNumberOfLines,
    #[msg("Public key mismatch.")]
    PublicKeyMismatch,
    #[msg("No SPL allowlist token present.")]
    NoSplAllowlistToken,
    #[msg("Token burn failed.")]
    TokenBurnFailed,
    #[msg("Unable to find an unused config line near your random number index.")]
    CannotFindUsableConfigLine,
    #[msg("Invalid string.")]
    InvalidString,
    #[msg("Suspicious transaction detected.")]
    SuspiciousTransaction,
    #[msg("Cannot Switch to Hidden Settings after items available is greater than 0.")]
    CannotSwitchToHiddenSettings,
    #[msg("Incorrect SlotHashes PubKey.")]
    IncorrectSlotHashesPubkey,
    #[msg("Incorrect collection NFT authority.")]
    IncorrectCollectionAuthority,
    #[msg("Collection PDA address is invalid.")]
    MismatchedCollectionPda,
    #[msg("Provided mint account doesn't match collection PDA mint.")]
    MismatchedCollectionMint,
    #[msg("Slot hashes Sysvar is empty.")]
    SlotHashesEmpty,
    #[msg("The metadata account has data in it, and this must be empty to mint a new NFT.")]
    MetadataAccountMustBeEmpty,
    #[msg("Missing set collection during mint IX for Candy Machine with collection set.")]
    MissingSetCollectionDuringMint,
    #[msg("Can't change collection settings after items have begun to be minted.")]
    NoChangingCollectionDuringMint,
    #[msg(
        "Can't change freeze settings after items have begun to be minted. You can only disable."
    )]
    NoChangingFreezeDuringMint,
    #[msg("Can't change authority while freeze is enabled. Disable freeze first.")]
    NoChangingAuthorityWithFreeze,
    #[msg("Can't change token while freeze is enabled. Disable freeze first.")]
    NoChangingTokenWithFreeze,
    #[msg(
        "Cannot thaw NFT unless all NFTs are minted or Candy Machine authority enables thawing."
    )]
    InvalidThawNft,
    #[msg("The number of remaining accounts passed in doesn't match the Candy Machine settings.")]
    IncorrectRemainingAccountsLen,
    #[msg("FreezePda ATA needs to be passed in if token mint is enabled.")]
    MissingFreezeAta,
    #[msg("Incorrect freeze ATA address.")]
    IncorrectFreezeAta,
    #[msg("FreezePda account doesn't belong to this Candy Machine.")]
    FreezePdaMismatch,
    #[msg("Freeze time can't be longer than MAX_FREEZE_TIME.")]
    EnteredFreezeIsMoreThanMaxFreeze,
    #[msg("Can't withdraw Candy Machine while freeze is active. Disable freeze first.")]
    NoWithdrawWithFreeze,
    #[msg(
        "Can't withdraw Candy Machine while frozen funds need to be redeemed. Unlock funds first."
    )]
    NoWithdrawWithFrozenFunds,
    #[msg("Missing required remaining accounts for remove_freeze with token mint.")]
    MissingRemoveFreezeTokenAccounts,
    #[msg("Can't withdraw SPL Token from freeze PDA into itself.")]
    InvalidFreezeWithdrawTokenAddress,
    #[msg("Can't unlock funds while NFTs are still frozen. Run thaw on all NFTs first.")]
    NoUnlockWithNFTsStillFrozen,
    #[msg("Invalid bot signer authority.")]
    InvalidBotSignerAuthority,
    #[msg("The wallet has already minted the maximum number allowed.")]
    BuyLimitPerAddressExceeded,
    #[msg("The provided merkle allowlist proof is invalid.")]
    InvalidAllowlistProof,
    #[msg("All available allowlist mints have already been claimed by this address.")]
    AllowlistMintsAlreadyUsed,
    #[msg("Maximum roots list count exceeded.")]
    MaximumRootCountExceeded,
    #[msg("Can only provide up to 5 omni mint wallets.")]
    TooManyOmniMintWallets,
    #[msg("Candy machine allowlist sale is not live.")]
    CandyMachineAllowlistSaleNotLive,
    #[msg("Candy machine public sale has ended.")]
    CandyMachinePublicSaleEnded,
    #[msg("Invalid candy machine mint phase times provided.")]
    CandyMachineInvalidMintPhases,
    #[msg("Bot tax collected.")]
    BotTaxCollected,
    #[msg("Invalid mint price provided.")]
    InvalidMintPrice,
    #[msg("Invalid allowlist settings. Can only enable a single allowlist feature at a time.")]
    InvalidAllowlistSettings,
}
