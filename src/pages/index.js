import React from "react"
import "../styles/index.css"
import { Link } from "gatsby"
import Layout from "../components/layout"

export default function Home() {
  return (
    <Layout>
      <h4 className={"channels"}>Channels</h4>
      <nav className={"nav"}>
        <Link to={`/channels/general`} className={"nav-items"}>#general</Link>
        <Link to={'/channels/random'} className={"nav-items"}>#random</Link>
      </nav>
    </Layout>
  )
}
