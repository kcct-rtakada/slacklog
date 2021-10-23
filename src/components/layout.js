import React from "react"
import "../styles/layout.css"
import Header from "./header"
import Footer from "./footer"

const Layout = ({ children }) => {
  return (
    <div className="layout">
      <Header />
      <main>
        <div className={"margin-16"}>
          {children}
        </div>
      </main>
      <Footer />
    </div>
  )
}

export default Layout
