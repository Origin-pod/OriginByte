# Product Requirements Document (PRD): HeadGesture Reader

## 1. Document Information

| **Attribute**        | **Details**                                                                               |
| -------------------- | ----------------------------------------------------------------------------------------- |
| **Product Name**     | HeadGesture Reader                                                                        |
| **Version**          | 1.0                                                                                       |
| **Date**             | November 3, 2025                                                                          |
| **Author**           | Grok (xAI Assistant)                                                                      |
| **Status**           | Draft                                                                                     |
| **Revision History** | Initial draft based on user concept for a hands-free PDF/book reader using head gestures. |

## 2. Problem-First Analysis

### Core Problem
Users with limited hand mobility need reliable ways to navigate digital documents without physical interaction. Current solutions are either:
1. **Expensive hardware**: Eye-tracking systems ($1K+)
2. **Software alternatives**: Voice control (requires speaking, not always practical)
3. **Mobile solutions**: Not ideal for desktop document workflows

**Problem Validation Needed**:
- How many users specifically need document navigation vs general computer control?
- Are current voice/mouse alternatives sufficient for most users?
- Will users accept webcam-based solution vs voice control?

### Solution Hypothesis
Head gesture detection could provide a middle-ground solution - more affordable than eye-tracking, less disruptive than voice commands for quiet environments.

**Key Assumptions to Validate**:
- Users prefer head gestures over voice commands for reading
- Webcam-based detection can achieve sufficient reliability
- Market size justifies development effort

## 3. Target Audience & User Personas

### Primary Users

- **Accessibility-Focused Readers**: Individuals with limited hand mobility (e.g., quadriplegia, arthritis). Pain point: Current tools require expensive hardware.
- **Multitaskers**: Chefs, mechanics, or parents reading recipes/manuals while hands are occupied.
- **Tech Enthusiasts**: Early adopters interested in AI/ML integrations.

### User Personas

| **Persona**                   | **Demographics**                           | **Goals**                                        | **Pain Points**                                          |
| ----------------------------- | ------------------------------------------ | ------------------------------------------------ | -------------------------------------------------------- |
| **Alex (Accessibility User)** | 35, wheelchair user, reads tech docs daily | Seamless page turns without fatigue              | Button mashing causes cramps; eye-tracking is too pricey |
| **Jordan (Multitasker)**      | 28, home cook, reads cookbooks on tablet   | Quick navigation during prep                     | Sticky hands ruin touchscreens                           |
| **Sam (Enthusiast)**          | 22, CS student, hacks personal projects    | Customizable gestures; open-source extensibility | Clunky prototypes lack polish                            |

**Market Size**: ~15M global users with mobility disabilities (WHO 2025 data); $2B accessibility software market growing 12% YoY.

## 4. Problem Statement & Success Metrics

### Problems Solved

- Traditional readers rely on touch/swipes, inaccessible for 10-15% of users.
- Existing gesture tools (e.g., Google Euphonia) are mobile-only or nod-based, ignoring tilt precision.
- No lightweight, webcam-only solution for desktops.

### Riskiest Assumptions & Validation Metrics

| **Risk**                          | **Validation Method**                     | **Success Criteria**                     |
| --------------------------------- | ----------------------------------------- | ---------------------------------------- |
| **Technical Feasibility**         | Build proof-of-concept with MediaPipe     | >70% accuracy in good lighting conditions |
| **User Acceptance**               | User interviews with accessibility users  | 60%+ prefer over voice alternatives      |
| **Market Size**                   | Survey potential users, analyze competitors| 1K+ interested users in first month      |
| **Reliability**                   | Extended testing in varied conditions     | <20% false positives per reading session |

## 5. Features & Prioritization

Features prioritized using MoSCoW method (Must-have, Should-have, Could-have, Won't-have for MVP).

### Core Features

| **Feature**                 | **Description**                                                                                  | **Priority** | **User Story**                                                                     |
| --------------------------- | ------------------------------------------------------------------------------------------------ | ------------ | ---------------------------------------------------------------------------------- |
| **PDF/EPUB Loading**        | Open and render PDFs/EPUBs with zoom/pan.                                                        | Must         | As Alex, I want to load my accessibility PDFs so I can read without setup hassle.  |
| **Head Gesture Navigation** | Detect yaw (left/right tilt) for prev/next pages; debounce to prevent misfires. Threshold: ±20°. | Must         | As Jordan, I want to tilt my head to turn pages so my hands stay free for cooking. |
| **Fallback Controls**       | Keyboard/mouse buttons for manual navigation.                                                    | Must         | As Sam, I want button backups so I can demo without a webcam.                      |
| **Calibration Mode**        | One-time setup to adjust thresholds based on user head size/lighting.                            | Should       | As Alex, I want to calibrate gestures so accuracy adapts to my setup.              |
| **Progress Indicators**     | Visual feedback (e.g., head overlay in mini-webcam view) for gesture confirmation.               | Should       | As Jordan, I want to see if my tilt registered so I avoid frustration.             |

### Advanced Features (Post-MVP)

| **Feature**               | **Description**                                 | **Priority** | **User Story**                                                  |
| ------------------------- | ----------------------------------------------- | ------------ | --------------------------------------------------------------- |
| **Multi-Gesture Support** | Nod (pitch) for zoom; shake for bookmarks.      | Could        | As Sam, I want nod-to-zoom so I can enlarge text hands-free.    |
| **Voice Integration**     | Optional TTS for page summaries.                | Could        | As Alex, I want voice read-aloud so I multitask audio-visually. |
| **Cross-Device Sync**     | Cloud bookmark sync (opt-in).                   | Won't (MVP)  | As Jordan, I want my page progress on phone and desktop.        |
| **Custom Themes**         | Dark mode, font adjustments for dyslexic users. | Could        | As Alex, I want high-contrast themes for eye strain relief.     |

## 6. Technical Requirements

### Functional Specs

- **Input**: Webcam (480p+ resolution); supports 80% of laptops (e.g., via OpenCV).
- **Output**: Rendered pages at 150 DPI; gesture events trigger UI updates.
- **Integrations**: MediaPipe for pose estimation (local ML model, <50MB); PyMuPDF for PDFs; ebooklib for EPUBs.

### Technical Reality Check

| **Component**          | **Claim vs Reality**                                                                 |
| ---------------------- | ------------------------------------------------------------------------------------ |
| **MediaPipe Accuracy** | Claim: 95% → Reality: 70-80% in optimal conditions, 50-60% in poor lighting          |
| **Latency**            | Claim: <100ms → Reality: 150-300ms on consumer hardware                              |
| **False Positives**    | Not addressed → Reality: High during natural head movement while reading              |
| **Lighting Dependency**| Not addressed → Reality: Major limiting factor for real-world usage                   |
| **Resource Usage**     | Claim: <200MB RAM → Reality: 300-500MB with video processing                         |

### Dependencies & Risks

- **Libs**: OpenCV, MediaPipe, Tkinter (built-in), PyMuPDF, ebooklib.
- **Risks**: Lighting variability affecting detection (mitigate: adaptive thresholds). Webcam permissions (mitigate: clear onboarding).

## 7. User Flows

### High-Level Flow

1. Launch app → Onboard (calibrate webcam/gestures).
2. Load document → Display page 1.
3. Read → Tilt head → Visual feedback → Page turns.
4. Exit/Save → Bookmark auto-saves.

**Wireframe Sketch** (Text-Based):

```
[Top Bar: File | Open PDF/EPUB | Calibrate]
[Main Canvas: PDF Page with Scrollbars]
[Mini-Webcam Preview: Head Overlay + Angle Indicator]
[Bottom: Prev Button | Page X/Y | Next Button | Settings]
```

## 8. Validation-First Roadmap

### Phase 0: Problem Validation (2-4 weeks)
**Goal**: Determine if this problem is worth solving before building anything

**Activities**:
- Interview 15-20 accessibility users about current document navigation habits
- Survey 100+ potential users about head gesture vs voice control preferences
- Technical spike: Build minimal MediaPipe proof-of-concept
- Competitive analysis: Test existing free alternatives

**Go/No-Go Criteria**:
- 70%+ of interviewed users express strong interest in head gestures
- Technical spike achieves >65% accuracy in normal lighting
- Clear differentiation from existing solutions identified

### Phase 1: MVP (Only if Phase 0 passes)
**Reduced Scope**: Single gesture (left tilt = next page) + PDF support only
**Timeline**: 6-8 weeks
**Resources**: 1-2 developers

### Phase 2: Expansion (Only if MVP gains traction)
- Additional gestures, EPUB support, premium features

**Total Investment Decision Point**: After Phase 0 validation (~4 weeks, minimal cost)

## 9. Recommendation: DON'T BUILD (Yet)

### Critical Issues Discovered

**Technical Reality**:
- MediaPipe accuracy (70-80%) may not meet user expectations for reliability
- High false positive rate during natural reading movements
- Significant lighting dependency limits real-world usage
- Performance claims in original PRD were overly optimistic

**Market Risks**:
- Strong free alternatives exist (Windows Speech Recognition, mobile accessibility)
- Eye-tracking prices decreasing ($200-300 range for basic models)
- User preference for voice control may be higher than assumed

**Business Model Flaws**:
- $4.99/year premium model unrealistic for niche accessibility tool
- Open-source approach contradicts premium features strategy
- Market size likely much smaller than claimed 15M users

### Alternative Approaches to Consider

1. **Voice-first solution**: Build better voice commands for document navigation
2. **Hybrid approach**: Combine multiple input methods (voice + simple gestures)
3. **Niche focus**: Target specific use cases where voice isn't viable (libraries, quiet offices)
4. **Integration play**: Build plugin for existing PDF readers instead of standalone app

### If You Must Proceed

**Minimum Viable Validation** (2 weeks, <40 hours):
1. Build simple MediaPipe PoC (single gesture detection)
2. Test with 10 accessibility users
3. Compare against voice control alternatives
4. Only proceed if >70% accuracy AND strong user preference

**Expected Outcome**: High probability of "no-go" decision based on technical and user research validation.

This analysis suggests investing time in problem validation before any significant development effort. The original PRD was overly optimistic about both technical feasibility and market opportunity.
