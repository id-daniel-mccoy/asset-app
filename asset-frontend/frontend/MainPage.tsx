import React, { useEffect, useState } from "react"

export function MainPage() {

  const filler = "This is content!";

  return (
    <div className="app">
      <div className="header">
        <h1>Welcome!</h1>
      </div>
      <div className="content">
        <p>{filler}</p>
        <button onClick={() => {alert("Hello World!");}}>Say Hello!</button>
      </div>
    </div>
  )
}