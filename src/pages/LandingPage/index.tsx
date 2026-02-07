import React, { lazy, Suspense, useState, useEffect } from 'react'
import profile from '../../../static/img/profile.jpg'
import './landingPage.css'

const SplashCursor = lazy(() => import('./SplashCursor'))

const CopilotIcon = () => (
  <svg stroke="currentColor" fill="currentColor" strokeWidth="0" viewBox="0 0 24 24" height="1em" width="1em">
    <path d="M9.75 14a.75.75 0 0 1 .75.75v2.5a.75.75 0 0 1-1.5 0v-2.5a.75.75 0 0 1 .75-.75Zm4.5 0a.75.75 0 0 1 .75.75v2.5a.75.75 0 0 1-1.5 0v-2.5a.75.75 0 0 1 .75-.75Z" />
    <path d="M12 2c2.214 0 4.248.657 5.747 1.756.136.099.268.204.397.312.584.235 1.077.546 1.474.952.85.869 1.132 2.037 1.132 3.368 0 .368-.014.733-.052 1.086l.633 1.478.043.022A4.75 4.75 0 0 1 24 15.222v1.028c0 .529-.309.987-.565 1.293-.28.336-.636.653-.966.918a13.84 13.84 0 0 1-1.299.911l-.024.015-.006.004-.039.025c-.223.135-.45.264-.68.386-.46.245-1.122.571-1.941.895C16.845 21.344 14.561 22 12 22c-2.561 0-4.845-.656-6.479-1.303a19.046 19.046 0 0 1-1.942-.894 14.081 14.081 0 0 1-.535-.3l-.144-.087-.04-.025-.006-.004-.024-.015a13.16 13.16 0 0 1-1.299-.911 6.913 6.913 0 0 1-.967-.918C.31 17.237 0 16.779 0 16.25v-1.028a4.75 4.75 0 0 1 2.626-4.248l.043-.022.633-1.478a10.195 10.195 0 0 1-.052-1.086c0-1.331.282-2.498 1.132-3.368.397-.406.89-.717 1.474-.952.129-.108.261-.213.397-.312C7.752 2.657 9.786 2 12 2Zm-8 9.654v6.669a17.59 17.59 0 0 0 2.073.98C7.595 19.906 9.686 20.5 12 20.5c2.314 0 4.405-.594 5.927-1.197a17.59 17.59 0 0 0 2.073-.98v-6.669l-.038-.09c-.046.061-.095.12-.145.177-.793.9-2.057 1.259-3.782 1.259-1.59 0-2.738-.544-3.508-1.492a4.323 4.323 0 0 1-.355-.508h-.344a4.323 4.323 0 0 1-.355.508C10.704 12.456 9.555 13 7.965 13c-1.725 0-2.989-.359-3.782-1.259a3.026 3.026 0 0 1-.145-.177Zm6.309-1.092c.445-.547.708-1.334.851-2.301.057-.357.087-.718.09-1.079v-.031c-.001-.762-.166-1.26-.43-1.568l-.008-.01c-.341-.391-1.046-.689-2.533-.529-1.505.163-2.347.537-2.824 1.024-.462.473-.705 1.18-.705 2.32 0 .605.044 1.087.135 1.472.092.384.231.672.423.89.365.413 1.084.75 2.657.75.91 0 1.527-.223 1.964-.564.14-.11.268-.235.38-.374Zm2.504-2.497c.136 1.057.403 1.913.878 2.497.442.545 1.134.938 2.344.938 1.573 0 2.292-.337 2.657-.751.384-.435.558-1.151.558-2.361 0-1.14-.243-1.847-.705-2.319-.477-.488-1.318-.862-2.824-1.025-1.487-.161-2.192.139-2.533.529-.268.308-.437.808-.438 1.578v.02c.002.299.023.598.063.894Z" />
  </svg>
)

const MountainIcon = () => (
  <svg stroke="currentColor" fill="currentColor" strokeWidth="0" viewBox="0 0 512 512" height="1em" width="1em">
    <path d="M149 24.32L123 124.4l27.8 44.8 15.9-29.4 37.5 30.9 17.5-55.5zm169.7 43.7l-32.2 48.38 10.7 25.5 21.9-21.7 18.1 42.8 10.3-21.7zm-83.4 64.18l-21.9 69.3-41.3-33.9L152 205l-35.2-56.7-88.38 339.4H154.9c45.6-17 135.3-56.7 137.2-106.3 1.7-47.8-108.9-43.4-110.8-91.2-1.5-39.1 84.5-81.5 84.5-81.5s-54.1 43.3-47.4 71c12.9 53 125.6 27.6 143.2 79.3 13 38.2-33.5 104.3-52.2 128.7h174.2L356.5 164.3 336.4 207l-23.5-55.4-22.1 21.9-16.3-39.1-17 25.5z" />
  </svg>
)

const CatIcon = () => (
  <svg stroke="currentColor" fill="currentColor" strokeWidth="0" viewBox="0 0 512 512" height="1em" width="1em">
    <path d="M290.59 192c-20.18 0-106.82 1.98-162.59 85.95V192c0-52.94-43.06-96-96-96-17.67 0-32 14.33-32 32s14.33 32 32 32c17.64 0 32 14.36 32 32v256c0 35.3 28.7 64 64 64h176c8.84 0 16-7.16 16-16v-16c0-17.67-14.33-32-32-32h-32l128-96v144c0 8.84 7.16 16 16 16h32c8.84 0 16-7.16 16-16V289.86c-10.29 2.67-20.89 4.54-32 4.54-61.81 0-113.52-44.05-125.41-102.4zM448 96h-64l-64-64v134.4c0 53.02 42.98 96 96 96s96-42.98 96-96V32l-64 64zm-72 80c-8.84 0-16-7.16-16-16s7.16-16 16-16 16 7.16 16 16-7.16 16-16 16zm80 0c-8.84 0-16-7.16-16-16s7.16-16 16-16 16 7.16 16 16-7.16 16-16 16z" />
  </svg>
)

const MoonIcon = () => (
  <svg stroke="currentColor" fill="currentColor" strokeWidth="0" viewBox="0 0 512 512" height="1em" width="1em">
    <path d="M283.211 512c78.962 0 151.079-35.925 198.857-94.792 7.068-8.708-.639-21.43-11.562-19.35-124.203 23.654-238.262-71.576-238.262-196.954 0-72.222 38.662-138.635 101.498-174.394 9.686-5.512 7.25-20.197-3.756-22.23A258.156 258.156 0 0 0 283.211 0c-141.309 0-256 114.511-256 256 0 141.309 114.511 256 256 256z" />
  </svg>
)

const CookIcon = () => (
  <svg stroke="currentColor" fill="currentColor" strokeWidth="0" viewBox="0 0 512 512" height="1em" width="1em">
    <path d="M255 22.31c-21.2 0-40.5 10.78-50.5 27.63l-4.4 7.42-7.6-4.08c-7.1-3.85-15.2-5.89-23.3-5.89-28.3 0-51.3 23.57-51.3 53.21 0 26.5 18.6 48.6 43.5 52.6l7.6 1.2v53.4c57.9-11.1 116-11.6 174 0v-69.7l4.8-2.6c15.7-8.3 26.1-26.1 26.1-46.15 0-28.8-20.7-51.12-45.4-51.12-6.5 0-13 1.61-18.9 4.72l-6.2 3.2-4.6-5.09C288 29.29 272 22.31 255 22.31zm1 194.79c-25.6 0-51.1 2.4-76.7 7-.9 6-1.3 12.2-1.3 18.6 0 29 9.2 55.2 23.6 73.7 14.4 18.5 33.5 29.3 54.4 29.3s40-10.8 54.4-29.3c14.4-18.5 23.6-44.7 23.6-73.7 0-6.4-.5-12.7-1.3-18.7-25.6-4.7-51.1-7-76.7-6.9zM208.7 348l-89.2 29.7-27.98 112H321.6L307.1 388l17.8-2.6 14.9 104.3h80.7l-28-112-89.2-29.7c-13.8 9.9-30 15.7-47.3 15.7-17.3 0-33.5-5.8-47.3-15.7zm69.3 52.7a9.999 9.999 0 0 1 10 10 9.999 9.999 0 0 1-10 10 9.999 9.999 0 0 1-10-10 9.999 9.999 0 0 1 10-10zm7 46a10 10 0 0 1 10 10 10 10 0 0 1-10 10 10 10 0 0 1-10-10 10 10 0 0 1 10-10z" />
  </svg>
)

const badges = [
  { label: 'Software Engineer', Icon: CopilotIcon },
  { label: 'Mountain Lover', Icon: MountainIcon },
  { label: 'Cat Mom', Icon: CatIcon },
  { label: 'Night Owl', Icon: MoonIcon },
  { label: 'Amateur Chef', Icon: CookIcon },
]

const Badge = ({ Icon, label }) => (
  <div className='badge'>
    <Icon />
    {label}
  </div>
)

function LandingPage() {
  const [showSplash, setShowSplash] = useState(false)

  useEffect(() => {
    const enable = () => setShowSplash(true)
    window.addEventListener('mousemove', enable, { once: true })
    window.addEventListener('touchstart', enable, { once: true })
    const id = typeof requestIdleCallback !== 'undefined'
      ? requestIdleCallback(() => enable(), { timeout: 3000 })
      : setTimeout(enable, 3000)
    return () => {
      window.removeEventListener('mousemove', enable)
      window.removeEventListener('touchstart', enable)
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
          src={profile}
          className="aboutme-profile-img"
          alt="Profile"
          width="150"
          height="150"
          fetchPriority="high"
        />
      </div>
      <div>
        <p className="aboutme-title">Sheetal Singh 👋</p>
        <p className="aboutme-location">@sheetalssr · Bengaluru, Karnataka</p>
        <div className="aboutme-badges">
          {badges.map(({ Icon, label }) => (
            <Badge key={label} Icon={Icon} label={label} />
          ))}
        </div>
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

export default LandingPage
