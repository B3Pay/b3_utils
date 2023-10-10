import { createReActor } from "@re-actor/core"
import { canisterId, createActor } from "../../declarations/payment_backend"

export const {
  ReActorProvider,
  callActor,
  initialize,
  useReActor,
  useActorState
} = createReActor(() => createActor(canisterId))
