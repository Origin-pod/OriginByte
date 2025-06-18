import React from 'react'
import profile from './profile.jpg'
import {data} from './data'
import './landingPage.css'

const Badge = ({Icon,label}) => {
  return (
    <div className='badge'>
        <Icon/>
        {label}
    </div>
  )
}

function LandingPage () {
  return (
<div className="aboutme-container">
      <div>
        <img src={profile} className="aboutme-profile-img" alt="Profile" />
      </div>
      <div>
        <p className="aboutme-title">Sheetal Singh 👋</p>
        <p className="aboutme-location">@sheetalssr · Bengaluru, Karnataka</p>
        <div className="aboutme-badges">
          {data.badges.map(({ icon, label }, idx) => (
            <Badge key={label + idx} Icon={icon} label={label} />
          ))}
        </div>
        <div className="aboutme-section">
          <h4>Welcome to my world.</h4>
          <h4>Living my best life !!</h4>
          <h4>writing code,</h4>
          <h4>
            solving problem,{' '}
            <span className="hovereffect" >
              reading literature
            </span>
            ,
          </h4>
          <h4>writing poetry, travelling places</h4>
          <h4>& experiencing life.</h4>
          <h4>Create . Cherish . Code . </h4>
          <br />
          <div>
            I'm open to partnering on{' '}
            <span className="hovereffect">side projects,</span>{' '}
            <span className="hovereffect">mentoring,</span>{' '}
            <span className="hovereffect">brainstorming,</span>{' '}
            <span className="hovereffect">beta testing new products,</span>{' '}
            <span className="hovereffect">freelance website development,</span>{' '}
            <span className="hovereffect">technical writing,</span>{' '}
            <span className="hovereffect">joining a book club,</span>{' '}
            <span className="hovereffect">reviewing book proposals,</span>{' '}
            <span className="hovereffect">and writing blog posts.</span>{' '}
          </div>
        </div>
      </div>
    </div>
  )
}

export default LandingPage
