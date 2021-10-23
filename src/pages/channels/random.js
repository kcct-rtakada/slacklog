import React from "react"
import "../../styles/index.css"
import { Link } from "gatsby"
import Layout from "../../components/layout"

export default function random() {
  return (
      <Layout>
        <h4 className={"channels"}>#random</h4>
        <nav className={"nav"}>
          <Link to={`/`} className={"nav-items"}>2021年10月</Link>
          <Link to={`/`} className={"nav-items"}>2021年9月</Link>
          <Link to={`/`} className={"nav-items"}>2021年8月</Link>
        </nav>
      </Layout>
  )
}