# Front-End Micro-Patterns for Senior Engineers (LLD Prep)

This document covers micro-level front-end design patterns that a senior software engineer should know for low-level design (LLD) and system design interviews.

---

## 🎨 UI Component Patterns

- **Controlled vs. Uncontrolled Components**
  - Controlled: State managed via props (e.g., controlled `<input>`).
  - Uncontrolled: State held in DOM via refs/default values.
- **Container vs. Presentational Components**
  - Container (smart): Handles data, state, side-effects.
  - Presentational (dumb): Purely UI rendering.
- **Compound Components**
  - Parent component provides context to child components (`<Tabs><Tab/><TabPanel/></Tabs>`).
- **Render Props**
  - Passing a function as a prop to control rendering.
- **Higher-Order Components (HOCs)**
  - Wrapping logic around a component (largely replaced by hooks).
- **Slot/Children Pattern**
  - Passing arbitrary content via `children` or slots (Vue/Svelte).
- **Controlled Form Handling**
  - Debounced inputs, validation, dirty-state tracking.
- **Headless Component Pattern**
  - Logic without styles; users provide render structure.
- **Fallback / Skeletons / Loading States**
- **Error Boundaries**
  - Catching render errors and displaying fallback UI.

---

## 🧩 State Management Micro-Patterns

- **Local Component State** (hooks, signals, reactive vars).
- **Derived State** (compute instead of storing redundantly).
- **Lifting State Up** (shared ownership).
- **Context for Cross-Cutting State** (global provider).
- **Reducer Pattern** (`useReducer`, Redux-style).
- **State Machines & Statecharts** (XState).
- **Atomic State** (Recoil/Jotai).
- **Event Emitters / Pub-Sub**.
- **Immutable State Updates**.
- **Undo/Redo Stack** (time travel for UI).

---

## 🔄 Data Fetching & Caching Patterns

- **SWR / Stale-While-Revalidate**.
- **Suspense for Data Fetching**.
- **Infinite Scroll vs. Pagination**.
- **Request Deduplication**.
- **Optimistic UI Updates**.
- **Prefetching Data on Hover/Idle**.
- **Cache Invalidation Strategies**.

---

## 📦 Rendering Optimization Patterns

- **Virtualized Lists** (windowing).
- **Lazy Loading / Code Splitting**.
- **Memoization** (`useMemo`, `useCallback`).
- **Pure Components / `React.memo`**.
- **Skeleton/Placeholder Rendering**.
- **Shallow Rendering / Selectors**.
- **Concurrency Patterns** (React concurrent mode).

---

## 🖇️ Composition & Reuse Micro-Patterns

- **Custom Hooks / Composables**.
- **Decorator Pattern**.
- **Mixins (legacy, but relevant)**.
- **Adapter Pattern** (normalize props/APIs).
- **Facade Pattern** (simplified APIs).

---

## 🌍 Accessibility & Interaction Patterns

- **Keyboard Navigation** (focus traps, arrow key movement).
- **ARIA Roles & Live Regions**.
- **Accessible Modals, Menus, Tooltips**.
- **Skip Links & Focus Management**.
- **Click Outside / Escape Key Handling**.
- **Responsive Layout Adjustments** (breakpoints, adaptive patterns).

---

## ⚡ Event Handling & DOM Patterns

- **Delegated Event Handling**.
- **Throttling & Debouncing Events**.
- **Passive Event Listeners**.
- **Synthetic Events vs. Native Events**.
- **Gesture Handling** (pan, pinch, swipe).
- **IntersectionObserver for Lazy Elements**.

---

## 🛠️ Error Handling & Resilience Patterns

- **Retry with Backoff** (exponential, jitter).
- **Graceful Fallback UI**.
- **Boundary Guards** (prop validation, type narrowing).
- **Circuit Breaker Pattern** (stop retrying broken APIs).

---

## 🧱 Styling & Theming Patterns

- **CSS-in-JS / CSS Modules / Utility Classes**.
- **Atomic CSS / Tailwind-like patterns**.
- **BEM & Scoped Naming Conventions**.
- **Theme Provider Pattern**.
- **Responsive Design Tokens**.
- **Design Tokens with CSS Vars**.
- **Dynamic Styling based on State**.

---

## 🔐 Security-Oriented Micro-Patterns

- **Sanitization & Escaping** (XSS prevention).
- **Strict CSP-Compatible Rendering**.
- **Clickjacking Prevention**.
- **Secure Cookie Handling**.
- **JWT Storage Strategies** (in-memory vs. localStorage).

---

## 🏗️ Testing & Debugging Patterns

- **Test ID / Data Attributes for Selectors**.
- **Mocking API Requests** (MSW, fake servers).
- **Snapshot Testing**.
- **E2E Patterns** (Cypress, Playwright).
- **Visual Regression Testing** (Percy, Chromatic).

---

## 🧮 Algorithmic / Utility Micro-Patterns

- **Diffing Algorithms** (virtual DOM).
- **String/HTML Escaping Utilities**.
- **Deep Clone vs. Shallow Clone**.
- **Debounce & Throttle Implementations**.
- **Binary Search in UI** (autocomplete, search).
- **Priority Queues** (task scheduling).

---

## ✅ Senior-Level Interview Expectations

- Be able to **demonstrate trade-offs**:
  - Optimistic vs. pessimistic updates.
  - Pagination vs. virtualization.
  - Debounce vs. throttle.
  - Context vs. lifting state.
- Show **scalability & maintainability** thinking.
- Highlight **performance, accessibility, and security** concerns.
