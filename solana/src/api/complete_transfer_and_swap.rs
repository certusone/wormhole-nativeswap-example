use bridge::{
    vaa::ClaimableVAA,
};
use token_bridge::{
    accounts::*,
    types::*,
    messages::{
        PayloadTransferWithPayload,
    }
};
use solana_program::{
    account_info::AccountInfo,
    msg,
    program::invoke_signed,
    // program_error::ProgramError,
    pubkey::Pubkey,
};
use solitaire::{
    processors::seeded::{
    //     invoke_seeded,
        Seeded,
    },
    CreationLamports::Exempt,
    *,
};
// use spl_token::state::{
//     Account,
//     Mint,
// };
// use std::ops::{
//     Deref,
//     DerefMut,
// };

// pub type ConfigAccount<'b, const State: AccountState> =
//     Derive<Data<'b, Config, { State }>, "config">;

pub type TokenBridgeConfigAccount<'b, const STATE: AccountState> = Data<'b, Config, { STATE }>;

pub struct TokenBridgeConfigAccountDerivationData {
    pub token_bridge: Pubkey,
}

impl<'b, const STATE: AccountState> Seeded<&TokenBridgeConfigAccountDerivationData>
    for TokenBridgeConfigAccount<'b, { STATE }>
{
    fn seeds(accs: &TokenBridgeConfigAccountDerivationData) -> Vec<Vec<u8>> {
        vec![
            String::from("config").as_bytes().to_vec(),
            accs.token_bridge.to_bytes().to_vec()
        ]
    }
}

pub type TokenBridgeMintSigner<'b> = Info<'b>;

pub struct TokenBridgeMintSignerDerivationData {
    pub token_bridge: Pubkey,
}

impl<'b> Seeded<&TokenBridgeMintSignerDerivationData>
    for TokenBridgeMintSigner<'b>
{
    fn seeds(accs: &TokenBridgeMintSignerDerivationData) -> Vec<Vec<u8>> {
        vec![
            String::from("mint_signer").as_bytes().to_vec(),
            accs.token_bridge.to_bytes().to_vec()
        ]
    }
}

#[derive(FromAccounts)]
pub struct CompleteTransferAndSwap<'b> {
    pub payer: Mut<Signer<AccountInfo<'b>>>,
    pub config: TokenBridgeConfigAccount<'b, { AccountState::Initialized }>,

    // Signed message for the transfer
    pub vaa: ClaimableVAA<'b, PayloadTransferWithPayload>,
    // Above includes claim account

    pub chain_registration: Endpoint<'b, { AccountState::Initialized }>,

    pub custody: Mut<CustodyAccount<'b, { AccountState::MaybeInitialized }>>,
    pub custody_signer: CustodySigner<'b>,
    pub to_fees: Mut<Data<'b, SplAccount, { AccountState::MaybeInitialized }>>,
    pub mint: Mut<WrappedMint<'b, { AccountState::Initialized }>>,
    pub wrapped_meta: WrappedTokenMeta<'b, { AccountState::Initialized }>,

    pub mint_authority: TokenBridgeMintSigner<'b>,
    pub token_bridge: Info<'b>,
}

impl<'a> From<&CompleteTransferAndSwap<'a>> for EndpointDerivationData {
    fn from(accs: &CompleteTransferAndSwap<'a>) -> Self {
        EndpointDerivationData {
            emitter_chain: accs.vaa.meta().emitter_chain,
            emitter_address: accs.vaa.meta().emitter_address,
        }
    }
}

impl<'a> From<&CompleteTransferAndSwap<'a>> for WrappedDerivationData {
    fn from(accs: &CompleteTransferAndSwap<'a>) -> Self {
        WrappedDerivationData {
            token_chain: accs.vaa.token_chain,
            token_address: accs.vaa.token_address,
        }
    }
}

impl<'a> From<&CompleteTransferAndSwap<'a>> for CustodyAccountDerivationData {
    fn from(accs: &CompleteTransferAndSwap<'a>) -> Self {
        CustodyAccountDerivationData {
            mint: *accs.mint.info().key,
        }
    }
}

impl<'b> InstructionContext<'b> for CompleteTransferAndSwap<'b> {
}

#[derive(BorshDeserialize, BorshSerialize, Default)]
pub struct CompleteTransferAndSwapData {}

pub fn complete_transfer_and_swap(
    ctx: &ExecutionContext,
    accs: &mut CompleteTransferAndSwap,
    _data: CompleteTransferAndSwapData,
) -> Result<()> {
    // Verify that the custody account is derived correctly
    let derivation_data: CustodyAccountDerivationData = (&*accs).into();
    accs.custody
        .verify_derivation(ctx.program_id, &derivation_data)?;
    
    if !accs.custody.is_initialized() {
        accs.custody
            .create(&(&*accs).into(), ctx, accs.payer.key, Exempt)?;

        let init_ix = spl_token::instruction::initialize_account(
            &spl_token::id(),
            accs.custody.info().key,
            accs.mint.info().key,
            accs.custody_signer.key,
        )?;
        invoke_signed(&init_ix, ctx.accounts, &[])?;
    }
    
    Ok(())
}
