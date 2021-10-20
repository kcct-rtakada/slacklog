import React from "react"
import "../styles/footer.css"

export default function Footer() {
  return (
    <footer className="footer">
      Powered by
      {' '}
      <a href="https://www.gatsbyjs.com" className="link">Gatsby</a>
    </footer>
  )
}