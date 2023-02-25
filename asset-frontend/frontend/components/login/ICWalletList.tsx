import React from "react"
import { StoicWallet as Stoic } from "./utils/Stoic";
import { PlugWallet as Plug} from "./utils/Plug";
import { InfinityWallet as Infinity} from "./utils/Infinity";
import { InternetIdentity } from "./utils/InternetIdentity";
import { NFID } from "./utils/NFID";
import "./index.css";

export function ICWalletList(props: any) {

  const giveToParent = props.giveToParent;

  const changeUserAuth = async(user: string) => {
    giveToParent(user);
  }

  return (
    <div className="walletList">
      <Stoic changeProvider={changeUserAuth}/>
      <Plug changeProvider={changeUserAuth}/>
      <Infinity changeProvider={changeUserAuth}/>
      <NFID changeProvider={changeUserAuth}/>
      <InternetIdentity changeProvider={changeUserAuth}/>
    </div>
  )
}