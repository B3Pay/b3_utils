export { idlFactory } from "../../../declarations/vetkd_notes"
export type {
  EncryptedNote,
  _SERVICE
} from "../../../declarations/vetkd_notes/vetkd_notes.did"
export const ENCRYPTED_NOTES_CANISTER_ID =
  process.env.ENCRYPTED_NOTES_RUST_CANISTER_ID
