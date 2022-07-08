use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct PostMessageData {
    /// Unique nonce for this message
    pub nonce: u32,

    /// Message payload
    pub payload: Vec<u8>,

    /// Commitment Level required for an attestation to be produced
    pub consistency_level: ConsistencyLevel,
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub enum ConsistencyLevel {
    Confirmed,
    Finalized,
}

// Wormhole core bridge commands.
#[derive(AnchorDeserialize, AnchorSerialize)]
pub enum CoreBridgeInstruction {
    Initialize,
    PostMessage,
    PostVAA,
    SetFees,
    TransferFees,
    UpgradeContract,
    UpgradeGuardianSet,
    VerifySignatures,
}

// Wormhole token bridge commands.
#[derive(AnchorDeserialize, AnchorSerialize)]
enum TokenBridgeInstruction {
    Initialize,
    AttestToken,
    CompleteNative,
    CompleteWrapped,
    TransferWrapped,
    TransferNative,
    RegisterChain,
    CreateWrapped,
    UpgradeContract,
    CompleteNativeWithPayload,
    CompleteWrappedWithPayload,
    TransferWrappedWithPayload,
    TransferNativeWithPayload,
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct BridgeData {
    /// The current guardian set index, used to decide which signature sets to accept.
    pub guardian_set_index: u32,

    /// Lamports in the collection account
    pub last_lamports: u64,

    /// Bridge configuration, which is set once upon initialization.
    pub config: BridgeConfig,
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct BridgeConfig {
    /// Period for how long a guardian set is valid after it has been replaced by a new one.  This
    /// guarantees that VAAs issued by that set can still be submitted for a certain period.  In
    /// this period we still trust the old guardian set.
    pub guardian_set_expiration_time: u32,

    /// Amount of lamports that needs to be paid to the protocol to post a message
    pub fee: u64,
}

#[derive(Debug)]
#[repr(transparent)]
pub struct PostedMessageData(pub MessageData);

/// All VAAs messages posted on solana have this header.
#[derive(Debug, Default, BorshDeserialize, BorshSerialize)]
pub struct MessageData {
    pub vaa_version: u8,                 // Header of the posted VAA
    pub consistency_level: u8,           // Level of consistency requested by the emitter
    pub vaa_time: u32,                   // Time the vaa was submitted
    pub vaa_signature_account: Pubkey,   // Account where signatures are stored
    pub submission_time: u32,            // Time the posted message was created
    pub nonce: u32,                      // Unique nonce for this message
    pub sequence: u64,                   // Sequence number of this message
    pub emitter_chain: u16,              // Emitter of the message
    pub emitter_address: [u8; 32],       // Emitter of the message
    pub payload: Vec<u8>,                // Message payload
}

impl AnchorDeserialize for PostedMessageData {
    fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        *buf = &buf[3..];
        Ok(PostedMessageData(
            <MessageData as BorshDeserialize>::deserialize(buf)?,
        ))
    }
}

pub fn get_message_data<'info>(vaa_account: &AccountInfo<'info>) -> Result<MessageData> {
    Ok(PostedMessageData::try_from_slice(&vaa_account.data.borrow())?.0)
}
