import React from 'react'
import profile from '../../../static/img/profile.jpg'
import './landingPage.css'
import { GiMountainRoad } from "react-icons/gi";
import { GoCopilot } from "react-icons/go";
import { FaCat } from "react-icons/fa";
import { FaMoon } from "react-icons/fa";
import { GiCook } from "react-icons/gi";

 const data = {
    title:"Sheetal Singh",
    badges: [
        {
            label: 'Software Engineer',
            icon: GoCopilot,
            color: "purple"
        },
        {
            label: 'Mountain Lover',
            icon: GiMountainRoad,
            color: "purple"
         },
         {
            label: 'Cat Mom',
            icon: FaCat,
            color: "purple"
        },
        {
            label: 'Night Owl',
            icon: FaMoon,
            color: "purple"
        },
        {
            label: 'Amateur Chef',
            icon: GiCook,
            color: "purple"
        },

    ],
}

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
          <h4>& capturing moments</h4>
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
