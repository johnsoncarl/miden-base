use miden_crypto::{
    utils::{ByteReader, ByteWriter, Deserializable, Serializable},
    Word,
};
use vm_processor::DeserializationError;

use crate::{
    accounts::AccountId,
    assembly::{Assembler, AssemblyContext, ProgramAst},
    assets::Asset,
    vm::CodeBlock,
    Digest, Felt, Hasher, NoteError, NOTE_TREE_DEPTH, WORD_SIZE, ZERO,
};

mod assets;
pub use assets::NoteAssets;

mod details;
pub use details::NoteDetails;

mod inputs;
pub use inputs::NoteInputs;

mod metadata;
pub use metadata::NoteMetadata;

mod note_header;
pub use note_header::NoteHeader;

mod note_id;
pub use note_id::NoteId;

mod note_tag;
pub use note_tag::{NoteExecutionHint, NoteTag};

mod note_type;
pub use note_type::NoteType;

mod nullifier;
pub use nullifier::Nullifier;

mod origin;
pub use origin::{NoteInclusionProof, NoteOrigin};

mod recipient;
pub use recipient::NoteRecipient;

mod script;
pub use script::NoteScript;

// CONSTANTS
// ================================================================================================

/// The depth of the leafs in the note Merkle tree used to commit to notes produced in a block.
/// This is equal `NOTE_TREE_DEPTH + 1`. In the kernel we do not authenticate leaf data directly
/// but rather authenticate hash(left_leaf, right_leaf).
pub const NOTE_LEAF_DEPTH: u8 = NOTE_TREE_DEPTH + 1;

// NOTE
// ================================================================================================

/// A note with all the data required for it to be consumed by executing it against the transaction
/// kernel.
///
/// Notes consist of note metadata and details. Note metadata is always public, but details may be
/// either public, encrypted, or private, depending on the note type. Note details consist of note
/// assets, script, inputs, and a serial number, the three latter grouped into a recipient object.
///
/// Note details can be reduced to two unique identifiers: [NoteId] and [Nullifier]. The former is
/// publicly associated with a note, while the latter is known only to entities which have access
/// to full note details.
///
/// Fungible and non-fungible asset transfers are done by moving assets to the note's assets. The
/// note's script determines the conditions required for the note consumption, i.e. the target
/// account of a P2ID or conditions of a SWAP, and the effects of the note. The serial number has
/// a double duty of preventing double spend, and providing unlikability to the consumer of a note.
/// The note's inputs allow for customization of its script.
///
/// To create a note, the kernel does not require all the information above, a user can create a
/// note only with the commitment to the script, inputs, the serial number (i.e., the recipient),
/// and the kernel only verifies the source account has the assets necessary for the note creation.
/// See [NoteRecipient] for more details.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Note {
    header: NoteHeader,
    details: NoteDetails,

    nullifier: Nullifier,
}

impl Note {
    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------

    /// Returns a new [Note] created with the specified parameters.
    pub fn new(assets: NoteAssets, metadata: NoteMetadata, recipient: NoteRecipient) -> Self {
        let details = NoteDetails::new(assets, recipient);
        let header = NoteHeader::new(details.id(), metadata);
        let nullifier = details.nullifier();

        Self { header, details, nullifier }
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns the note's unique identifier.
    ///
    /// This value is both an unique identifier and a commitment to the note.
    pub fn id(&self) -> NoteId {
        self.header.id()
    }

    /// Returns the note's metadata.
    pub fn metadata(&self) -> &NoteMetadata {
        self.header.metadata()
    }

    /// Returns the note's assets.
    pub fn assets(&self) -> &NoteAssets {
        self.details.assets()
    }

    /// Returns the note's recipient serial_num, the secret required to consume the note.
    pub fn serial_num(&self) -> Word {
        self.details.serial_num()
    }

    /// Returns the note's recipient script which locks the assets of this note.
    pub fn script(&self) -> &NoteScript {
        self.details.script()
    }

    /// Returns the note's recipient inputs which customizes the script's behavior.
    pub fn inputs(&self) -> &NoteInputs {
        self.details.inputs()
    }

    /// Returns the note's recipient.
    pub fn recipient(&self) -> &NoteRecipient {
        self.details.recipient()
    }

    /// Returns the note's nullifier.
    ///
    /// This is public data, used to prevent double spend.
    pub fn nullifier(&self) -> Nullifier {
        self.nullifier
    }

    /// Returns the note's authentication hash.
    ///
    /// This value is used authenticate the note's presence in the note tree, it is computed as:
    ///
    /// > hash(note_id, note_metadata)
    ///
    pub fn authentication_hash(&self) -> Digest {
        Hasher::merge(&[self.id().inner(), Word::from(self.metadata()).into()])
    }
}

// CONVERSIONS FROM NOTE
// ================================================================================================

impl From<&Note> for NoteHeader {
    fn from(note: &Note) -> Self {
        note.header
    }
}

impl From<Note> for NoteHeader {
    fn from(note: Note) -> Self {
        note.header
    }
}

impl From<&Note> for NoteDetails {
    fn from(note: &Note) -> Self {
        note.details.clone()
    }
}

impl From<Note> for NoteDetails {
    fn from(note: Note) -> Self {
        note.details
    }
}

// SERIALIZATION
// ================================================================================================

impl Serializable for Note {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        let Self {
            header,
            details,

            // nullifier is not serialized as it can be computed from the rest of the data
            nullifier: _,
        } = self;

        // only metadata is serialized as note ID can be computed from note details
        header.metadata().write_into(target);
        details.write_into(target);
    }
}

impl Deserializable for Note {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        let metadata = NoteMetadata::read_from(source)?;
        let details = NoteDetails::read_from(source)?;
        let (assets, recipient) = details.into_parts();

        Ok(Self::new(assets, metadata, recipient))
    }
}
