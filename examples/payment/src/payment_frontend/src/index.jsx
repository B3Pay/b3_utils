import React from "react"
import { createRoot } from "react-dom/client"
import App from "./App"
import { ReActorProvider } from "./actor"

const container = document.getElementById("root")
const root = createRoot(container)

root.render(
  <ReActorProvider>
    <App />
  </ReActorProvider>
)
