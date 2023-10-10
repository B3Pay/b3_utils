import Balance from "./Balance"
import { useReActor } from "./actor"

const App = () => {
  const { actorState, loading, initialized } = useReActor()

  console.log("actorState", actorState)
  console.log("loadingState", loading)

  return (
    <div>
      <div
        style={{
          display: "flex",
          flexDirection: "column",
          width: "100%",
          justifyContent: "space-between"
        }}
      >
        <h1>Test Actor</h1>
        {initialized
          ? "Initialized"
          : loading
          ? "Loading..."
          : "Not initialized"}
        <Balance />
      </div>
    </div>
  )
}

export default App
