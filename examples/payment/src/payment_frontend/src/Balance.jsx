import { useActorMethod } from "./actor"

const Balance = ({}) => {
  const { call, loading, result, error } = useActorMethod("balance")

  return (
    <div>
      {error && <div className="error">{error?.toString()}</div>}
      <button onClick={() => call()} disabled={loading}>
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
