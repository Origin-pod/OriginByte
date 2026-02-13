import React from 'react';
import Layout from '@theme/Layout';
import styles from './styles.module.css';

const featured = {
  title: 'Origin DB Simulator',
  tagline: 'Building a database from scratch. In Rust. Because why not.',
  description: 'A from-the-ground-up database simulator — storage engine, query parsing, indexing, the whole shebang. The kind of project where you wake up at 3am thinking about B-trees.',
  tech: ['Rust'],
  status: 'Active',
  link: 'https://github.com/Origin-pod/Origin-DB-Simulator',
  homepage: 'https://db-simulator.pages.dev',
};

const originProjects = [
  {
    title: 'OriginSublime',
    description: 'AI-powered content curation and action generation. Feed it chaos, get back clarity.',
    tech: ['TypeScript'],
    status: 'Active',
    link: 'https://github.com/Origin-pod/OriginSublime',
    emoji: '\u2728',
  },
  {
    title: 'OriginBrain',
    description: 'All your "will read later" links, intelligently curated so you actually read them. Your future self will thank you.',
    tech: ['Python'],
    status: 'Active',
    link: 'https://github.com/Origin-pod/OriginBrain',
    emoji: '\uD83E\uDDE0',
  },
  {
    title: 'OriginSteward',
    description: 'Jarvis, but for workflows. Automates the boring stuff so you can focus on the fun stuff.',
    tech: ['TypeScript'],
    status: 'Active',
    link: 'https://github.com/Origin-pod/OriginSteward',
    emoji: '\uD83E\uDD16',
  },
  {
    title: 'OriginArt',
    description: 'A melting pot of design systems, 2D/3D experiments, and components. Where pixels go to party.',
    tech: ['TypeScript'],
    status: 'Experimenting',
    link: 'https://github.com/Origin-pod/OriginArt',
    emoji: '\uD83C\uDFA8',
  },
  {
    title: 'OriginSystems',
    description: 'Everything systems. Low-level Rust explorations — the kind of code that talks directly to your hardware.',
    tech: ['Rust'],
    status: 'Active',
    link: 'https://github.com/Origin-pod/OriginSystems',
    emoji: '\u2699\uFE0F',
  },
];

const sideQuests = [
  {
    title: 'Parseltongue',
    description: 'A programming language for NLP and AI — built in Rust because we like our compilers fast and our error messages helpful.',
    tech: ['Rust', 'NLP', 'Compiler Design'],
    status: 'Research',
    link: 'https://github.com/that-in-rust/parseltongue',
  },
  {
    title: 'Vanish',
    description: 'Privacy-focused browser extension. Because your online presence should be a choice, not a default.',
    tech: ['JavaScript', 'Browser Extensions'],
    status: 'Shipped',
    link: 'https://github.com/happycoder0011/Vanish',
  },
  {
    title: 'Twitter Bookmarks 2.0',
    description: 'A smarter way to wrangle your saved tweets. No more "I bookmarked this somewhere..." moments.',
    tech: ['Chrome Extension', 'AI/ML'],
    status: 'In Progress',
    link: 'https://github.com/happycoder0011/Twitter-Bookmark-2.0',
  },
];

const openSource = [
  {
    title: 'Fluent UI — Accessibility Fixes',
    description: 'Contributed accessibility improvements to Microsoft\'s Fluent UI. Making the web usable for everyone, one PR at a time.',
    tech: ['TypeScript', 'React', 'A11y'],
    link: 'https://github.com/microsoft/fluentui/pull/35300',
  },
];

const statusColors: Record<string, string> = {
  'Active': 'statusActive',
  'Shipped': 'statusShipped',
  'In Progress': 'statusWip',
  'Research': 'statusResearch',
  'Experimenting': 'statusExperimenting',
};

const Hacks = () => {
  return (
    <Layout title="Hacks" description="Things I build when I should be sleeping" noFooter>
      <div className={styles.wrapper}>
      <div className={styles.page}>
        {/* Hero */}
        <header className={styles.hero}>
          <h1 className={styles.heroTitle}>The Lab</h1>
          <p className={styles.heroSub}>
            Stuff I build, break, and occasionally ship.
            <br />
            Fueled by curiosity and an unreasonable amount of coffee.
          </p>
        </header>

        {/* Featured Project */}
        <section className={styles.featuredSection}>
          <div className={styles.featuredLabel}>Currently Obsessed With</div>
          <a
            href={featured.link}
            className={styles.featuredCard}
            target="_blank"
            rel="noopener noreferrer"
          >
            <div className={styles.featuredContent}>
              <div className={styles.featuredTop}>
                <h2 className={styles.featuredTitle}>{featured.title}</h2>
                <span className={`${styles.statusBadge} ${styles.statusActive}`}>
                  {featured.status}
                </span>
              </div>
              <p className={styles.featuredTagline}>{featured.tagline}</p>
              <p className={styles.featuredDesc}>{featured.description}</p>
              <div className={styles.featuredFooter}>
                <div className={styles.techRow}>
                  {featured.tech.map((t) => (
                    <span key={t} className={styles.techPill}>{t}</span>
                  ))}
                </div>
                {featured.homepage && (
                  <span className={styles.liveLink}>
                    Live Demo &rarr;
                  </span>
                )}
              </div>
            </div>
            <div className={styles.featuredGlow} />
          </a>
        </section>

        {/* Origin Universe */}
        <section className={styles.section}>
          <h2 className={styles.sectionTitle}>
            <span className={styles.sectionIcon}>&gt;_</span>
            Origin Universe
          </h2>
          <p className={styles.sectionSub}>
            The org. A collection of tools, experiments, and rabbit holes we maintain at{' '}
            <a href="https://github.com/Origin-pod" target="_blank" rel="noopener noreferrer">
              Origin-pod
            </a>.
          </p>
          <div className={styles.projectGrid}>
            {originProjects.map((project) => (
              <a
                key={project.title}
                href={project.link}
                className={styles.projectCard}
                target="_blank"
                rel="noopener noreferrer"
              >
                <div className={styles.cardEmoji}>{project.emoji}</div>
                <div className={styles.cardBody}>
                  <div className={styles.cardTop}>
                    <h3 className={styles.cardTitle}>{project.title}</h3>
                    <span className={`${styles.statusBadge} ${styles[statusColors[project.status] || 'statusActive']}`}>
                      {project.status}
                    </span>
                  </div>
                  <p className={styles.cardDesc}>{project.description}</p>
                  <div className={styles.techRow}>
                    {project.tech.map((t) => (
                      <span key={t} className={styles.techPill}>{t}</span>
                    ))}
                  </div>
                </div>
              </a>
            ))}
          </div>
        </section>

        {/* Side Quests */}
        <section className={styles.section}>
          <h2 className={styles.sectionTitle}>
            <span className={styles.sectionIcon}>~</span>
            Side Quests
          </h2>
          <p className={styles.sectionSub}>
            The "what if I just..." projects. Personal experiments, weekend hacks,
            and things that started as a tweet and became a repo.
          </p>
          <div className={styles.questList}>
            {sideQuests.map((quest) => (
              <a
                key={quest.title}
                href={quest.link}
                className={styles.questCard}
                target="_blank"
                rel="noopener noreferrer"
              >
                <div className={styles.questLeft}>
                  <h3 className={styles.questTitle}>{quest.title}</h3>
                  <p className={styles.questDesc}>{quest.description}</p>
                  <div className={styles.techRow}>
                    {quest.tech.map((t) => (
                      <span key={t} className={styles.techPill}>{t}</span>
                    ))}
                  </div>
                </div>
                <span className={`${styles.statusBadge} ${styles[statusColors[quest.status] || 'statusWip']}`}>
                  {quest.status}
                </span>
              </a>
            ))}
          </div>
        </section>

        {/* Open Source */}
        <section className={styles.section}>
          <h2 className={styles.sectionTitle}>
            <span className={styles.sectionIcon}>$</span>
            Open Source Trail
          </h2>
          <p className={styles.sectionSub}>
            Giving back to the projects that taught me everything.
          </p>
          <div className={styles.questList}>
            {openSource.map((item) => (
              <a
                key={item.title}
                href={item.link}
                className={styles.questCard}
                target="_blank"
                rel="noopener noreferrer"
              >
                <div className={styles.questLeft}>
                  <h3 className={styles.questTitle}>{item.title}</h3>
                  <p className={styles.questDesc}>{item.description}</p>
                  <div className={styles.techRow}>
                    {item.tech.map((t) => (
                      <span key={t} className={styles.techPill}>{t}</span>
                    ))}
                  </div>
                </div>
                <span className={`${styles.statusBadge} ${styles.statusShipped}`}>
                  Merged
                </span>
              </a>
            ))}
          </div>
        </section>

        {/* Footer */}
        <footer className={styles.footerCta}>
          <p>
            Built with love.{' '}
            <a href="https://x.com/sheetalssr" target="_blank" rel="noopener noreferrer">
              Say hi on X @sheetalssr
            </a>
          </p>
        </footer>
      </div>
      </div>
    </Layout>
  );
};

export default Hacks;
