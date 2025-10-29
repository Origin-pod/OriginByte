import React from 'react';
import Layout from '@theme/Layout';
import styles from './styles.module.css';

const projects = {
  openSourceContributions: [
    {
      title: 'Fluent UI - Accessibility Improvements',
      description: 'Contributed accessibility fixes to Microsoft\'s Fluent UI component library',
      technologies: ['TypeScript', 'React', 'Accessibility'],
      date: '2025',
      link: 'https://github.com/microsoft/fluentui/pull/35300',
      type: 'Open Source Contribution'
    }
  ],
  researchProjects: [
    {
      title: 'Parseltongue',
      description: 'A Rust-based programming language for natural language processing and AI applications',
      technologies: ['Rust', 'NLP', 'Compiler Design'],
      date: '2025',
      link: 'https://github.com/that-in-rust/parseltongue',
      type: 'Research Project'
    }
  ],
  personalHacks: [
    {
      title: 'Vanish',
      description: 'A privacy-focused web extension that helps manage and protect your online presence',
      technologies: ['JavaScript', 'Browser Extensions', 'Privacy'],
      date: '2025',
      link: 'https://github.com/happycoder0011/Vanish',
      type: 'Personal Project'
    },
    {
      title: 'Twitter Bookmarks 2.0',
      description: 'A smarter way to organize and retrieve your saved tweets and resources',
      technologies: ['Chrome Extension', 'AI/ML', 'Browser APIs'],
      date: '2025',
      status: 'In Progress',
      link: 'https://github.com/happycoder0011/Twitter-Bookmark-2.0',
      type: 'Personal Project'
    }
  ]
};

const Hacks = () => {
  return (
    <Layout title="Hacks" description="My experiments, contributions, and projects">
      <div className={styles.hacksContainer}>
        <h1>Projects & Contributions</h1>
        <p className={styles.subtitle}>A collection of my open source work, research, and personal projects</p>
        
        {Object.entries(projects).map(([category, items]) => (
          <section key={category} className={styles.categorySection}>
            <h2 className={styles.categoryTitle}>
              {category.split(/(?=[A-Z])/).join(' ')}
            </h2>
            <div className={styles.hacksGrid}>
              {items.map((item, index) => (
                <a 
                  key={index} 
                  href={item.link}
                  className={styles.hackCard}
                  target="_blank"
                  rel="noopener noreferrer"
                >
                  <div className={styles.hackHeader}>
                    <h3>
                      {item.title}
                      <span className={`${styles.status} ${styles[item.status?.toLowerCase() || 'inprogress']}`}>
                        {item.status || 'In Progress'}
                      </span>
                    </h3>
                  </div>
                  <p>{item.description}</p>
                  <div className={styles.detailsRow}>
                    <div className={styles.technologies}>
                      {item.technologies.map((tech, i) => (
                        <span key={i} className={styles.techTag}>{tech}</span>
                      ))}
                    </div>
                    <div className={styles.date}>{item.date}</div>
                  </div>
                </a>
              ))}
            </div>
          </section>
        ))}
      </div>
    </Layout>
  );
};

export default Hacks;
