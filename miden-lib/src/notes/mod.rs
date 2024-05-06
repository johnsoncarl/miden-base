use alloc::vec::Vec;

use miden_objects::{
    accounts::AccountId,
    assets::Asset,
    crypto::rand::FeltRng,
    notes::{
        Note, NoteAssets, NoteExecutionHint, NoteInputs, NoteMetadata, NoteRecipient, NoteTag,
        NoteType,
    },
    NoteError, Word, ZERO,
};

use self::utils::build_note_script;

pub mod utils;

// STANDARDIZED SCRIPTS
// ================================================================================================

/// Generates a P2ID note - pay to id note.
///
/// This script enables the transfer of assets from the `sender` account to the `target` account
/// by specifying the target's account ID.
///
/// The passed-in `rng` is used to generate a serial number for the note. The returned note's tag
/// is set to the target's account ID.
///
/// # Errors
/// Returns an error if deserialization or compilation of the `P2ID` script fails.
pub fn create_p2id_note<R: FeltRng>(
    sender: AccountId,
    target: AccountId,
    assets: Vec<Asset>,
    note_type: NoteType,
    mut rng: R,
) -> Result<Note, NoteError> {
    let bytes = include_bytes!(concat!(env!("OUT_DIR"), "/assets/note_scripts/P2ID.masb"));
    let note_script = build_note_script(bytes)?;

    let inputs = NoteInputs::new(vec![target.into()])?;
    let tag = NoteTag::from_account_id(target, NoteExecutionHint::Local)?;
    let serial_num = rng.draw_word();
    let aux = ZERO;

    let metadata = NoteMetadata::new(sender, note_type, tag, aux)?;
    let vault = NoteAssets::new(assets)?;
    let recipient = NoteRecipient::new(serial_num, note_script, inputs);
    Ok(Note::new(vault, metadata, recipient))
}

/// Generates a P2IDR note - pay to id with recall after a certain block height.
///
/// This script enables the transfer of assets from the sender `sender` account to the `target`
/// account by specifying the target's account ID. Additionally it adds the possibility for the
/// sender to reclaiming the assets if the note has not been consumed by the target within the
/// specified timeframe.
///
/// The passed-in `rng` is used to generate a serial number for the note. The returned note's tag
/// is set to the target's account ID.
///
/// # Errors
/// Returns an error if deserialization or compilation of the `P2IDR` script fails.
pub fn create_p2idr_note<R: FeltRng>(
    sender: AccountId,
    target: AccountId,
    assets: Vec<Asset>,
    note_type: NoteType,
    recall_height: u32,
    mut rng: R,
) -> Result<Note, NoteError> {
    let bytes = include_bytes!(concat!(env!("OUT_DIR"), "/assets/note_scripts/P2IDR.masb"));
    let note_script = build_note_script(bytes)?;

    let inputs = NoteInputs::new(vec![target.into(), recall_height.into()])?;
    let tag = NoteTag::from_account_id(target, NoteExecutionHint::Local)?;
    let serial_num = rng.draw_word();
    let aux = ZERO;

    let vault = NoteAssets::new(assets)?;
    let metadata = NoteMetadata::new(sender, note_type, tag, aux)?;
    let recipient = NoteRecipient::new(serial_num, note_script, inputs);
    Ok(Note::new(vault, metadata, recipient))
}

/// Generates a SWAP note - swap of assets between two accounts.
///
/// This script enables a swap of 2 assets between the `sender` account and any other account that
/// is willing to consume the note. The consumer will receive the `offered_asset` and will create a
/// new P2ID note with `sender` as target, containing the `requested_asset`.
///
/// # Errors
/// Returns an error if deserialization or compilation of the `SWAP` script fails.
pub fn create_swap_note<R: FeltRng>(
    sender: AccountId,
    offered_asset: Asset,
    requested_asset: Asset,
    note_type: NoteType,
    mut rng: R,
) -> Result<(Note, Word), NoteError> {
    let bytes = include_bytes!(concat!(env!("OUT_DIR"), "/assets/note_scripts/SWAP.masb"));
    let note_script = build_note_script(bytes)?;

    let payback_serial_num = rng.draw_word();
    let payback_recipient = utils::build_p2id_recipient(sender, payback_serial_num)?;
    let asset_word: Word = requested_asset.into();
    let payback_tag = NoteTag::from_account_id(sender, NoteExecutionHint::Local)?;

    let inputs = NoteInputs::new(vec![
        payback_recipient[0],
        payback_recipient[1],
        payback_recipient[2],
        payback_recipient[3],
        asset_word[0],
        asset_word[1],
        asset_word[2],
        asset_word[3],
        payback_tag.inner().into(),
    ])?;

    // TODO: build the tag for the SWAP use case
    let tag = 0.into();
    let serial_num = rng.draw_word();
    let aux = ZERO;

    let metadata = NoteMetadata::new(sender, note_type, tag, aux)?;
    let vault = NoteAssets::new(vec![offered_asset])?;
    let recipient = NoteRecipient::new(serial_num, note_script, inputs);
    let note = Note::new(vault, metadata, recipient);

    Ok((note, payback_serial_num))
}
