import React from "react"
import { Link } from "gatsby"
import Layout from "../../components/layout"

export default function random() {
  return (
    <>
      <Layout>
        <h4>#random</h4>
        <nav>
          <Link to={`/`}></Link>
        </nav>
      </Layout>
    </>
  )
}