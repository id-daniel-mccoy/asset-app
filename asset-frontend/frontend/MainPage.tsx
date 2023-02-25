import React, { useEffect, useState } from "react"
import { ICWalletList } from "./components/login/ICWalletList";
import { CreateCanister } from "./components/canister-ops/CreateCanister";

export function MainPage() {

  const [hasLoggedIn, setHasLoggedIn] = useState(false);
  const [user, setUser] = useState("Not Connected");

  const receiveFromChild = async(user: string) => {
    setUser(user);
    setHasLoggedIn(true);
  }

  const filler = "This is content!";

  return (
    <div className="app">
      <h1>The Asset App</h1>
      {!hasLoggedIn ? <ICWalletList giveToParent={receiveFromChild}/> : <CreateCanister />}
    </div>
  )
}