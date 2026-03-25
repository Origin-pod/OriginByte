import React, { lazy, Suspense, useState, useEffect } from 'react'

const SplashCursor = lazy(() => import('./SplashCursor'))

function isMobile() {
  if (typeof window === 'undefined') return false
  return window.matchMedia('(max-width: 768px)').matches
    || 'ontouchstart' in window
}

export default function LandingPage() {
  const [showSplash, setShowSplash] = useState(false)

  useEffect(() => {
    if (isMobile()) return

    const enable = () => setShowSplash(true)
    window.addEventListener('mousemove', enable, { once: true })
    const id = typeof requestIdleCallback !== 'undefined'
      ? requestIdleCallback(() => enable(), { timeout: 3000 })
      : setTimeout(enable, 3000)
    return () => {
      window.removeEventListener('mousemove', enable)
      typeof requestIdleCallback !== 'undefined'
        ? cancelIdleCallback(id as number)
        : clearTimeout(id as ReturnType<typeof setTimeout>)
    }
  }, [])

  return (
    <div className="aboutme-container">
      {showSplash && (
        <Suspense fallback={null}>
          <SplashCursor />
        </Suspense>
      )}

      <div>
        <img
          src="/profile.jpg"
          className="aboutme-profile-img"
          alt="Profile"
          width="150"
          height="150"
        />
      </div>
      <div>
        <p className="aboutme-title">Sheetal Singh 👋</p>
        <p className="aboutme-location">@sheetalssr · Bengaluru, Karnataka</p>
        <div className="aboutme-section">
          <h4>Welcome to my world.</h4>
          <h4>Living my best life !!</h4>
          <h4>
            <a
              href="https://github.com/happycoder0011"
              target="_blank"
              rel="noopener noreferrer"
              className="hovereffect"
            >
              Code for fun
            </a>
          </h4>
          <h4>
            <a
              href="https://www.goodreads.com/review/list/166403141?shelf=%23ALL%23"
              target="_blank"
              rel="noopener noreferrer"
              className="hovereffect"
            >
              Read books
            </a>
          </h4>
          <h4>
            <a
              href="https://www.instagram.com/pondertub/"
              target="_blank"
              rel="noopener noreferrer"
              className="hovereffect"
            >
              Capture moments
            </a>
          </h4>
          <h4>Create . Cherish . Code . </h4>
          <br />
          <div>
            I'm open to partnering on{' '}
            <span className="hovereffect">side projects,</span>{' '}
            <span className="hovereffect">brainstorming,</span>{' '}
            <span className="hovereffect">beta testing,</span>{' '}
            <span className="hovereffect">technical writing,</span>{' '}
            <span className="hovereffect">joining a book club.</span>{' '}
          </div>
        </div>
      </div>
    </div>
  )
}
