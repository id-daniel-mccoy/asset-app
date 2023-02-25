import React, { useEffect, useState } from "react"

export function CreateCanister() {

  const [hasLoggedIn, setHasLoggedIn] = useState(false);
  const [user, setUser] = useState("Not Connected");

  const receiveFromChild = async(user: string) => {
    setUser(user);
    setHasLoggedIn(true);
  }

  return (
    <div className="app">
      <h3>Create A New Canister</h3>
      <button onClick={() => alert("Still Testing!")}>Create Assets Canister</button>
    </div>
  )
}