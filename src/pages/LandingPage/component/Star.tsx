import React, { CSSProperties } from 'react'
import  '../landingPage.css'

function Stars ({ children }) {
  const wH = window.innerHeight
  const wW = window.innerWidth
  const n = 600
  const stars: React.ReactElement[] = []

  // Calculate safe margins to keep stars within viewport
  const margin = 20 // pixels from edge
  const maxTop = wH - margin
  const maxLeft = wW - margin

  for (let i = 0; i < n; i++) {
    const classname =
      i % 20 === 0 ? 'star star--big' : i % 9 === 0 ? 'star star--medium' : 'star'
    
    // Ensure stars stay within viewport bounds
    const top = Math.min(Math.max(Math.round(Math.random() * wH), margin), maxTop)
    const left = Math.min(Math.max(Math.round(Math.random() * wW), margin), maxLeft)
    
    const mystyle: CSSProperties = {
      position: 'absolute', // Changed to absolute for better positioning
      top: `${top}px`,
      left: `${left}px`,
      animationDuration: `${Math.round(Math.random() * 3000) + 3000}ms`,
      animationDelay: `${Math.round(Math.random() * 3000)}ms`,
      display: 'inline-flex'
    }
    
    stars.push(<div key={i} className={classname} style={mystyle} />)
  }
  return (
    <div className='star-container'>
      {stars}
      {children}
    </div>
  )
}

export default Stars
