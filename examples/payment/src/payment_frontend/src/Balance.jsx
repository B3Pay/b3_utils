import { callActor, useActorState } from "./actor"

const Balance = ({}) => {
  const { loading, result, error } = useActorState("balance")

  return (
    <div>
      {error && <div className="error">{error?.toString()}</div>}
      <button onClick={() => callActor("balance")} disabled={loading}>
        Get Balance
      </button>
      {loading ? (
        <div className="loader">loading...</div>
      ) : (
        <div>
          <h1>{result?.toString()}</h1>
        </div>
      )}
    </div>
  )
}

export default Balance
