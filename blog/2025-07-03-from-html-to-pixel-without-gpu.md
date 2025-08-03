---
slug: from-html-to-pixel-without-gpu
title: From HTML to Pixel without GPU
authors: [Sheetal]
tags: [frontend, UI]
---

# From HTML to Pixel without GPU: The Pre-GPU Era

> **Related Article**: [From HTML to Pixel: A Deep Dive into Browser Rendering](/blog/from-html-to-pixel) - Learn how modern browsers use GPU acceleration to render web pages efficiently.

In our [previous article](/blog/from-html-to-pixel), we explored how modern browsers use GPU acceleration to render web pages efficiently. But what about the time before GPUs were widely available? How did browsers transform HTML into pixels when everything had to be done on the CPU?

## The CPU-Only Rendering Era (1990s - Early 2000s)

Before GPU acceleration became mainstream, browsers relied entirely on CPU-based rendering. This was a fundamentally different approach that shaped how web pages were designed and optimized.

## The CPU-Only Rendering Pipeline

### 1. Parsing the HTML → DOM

**Same as modern browsers** - HTML parsing and DOM construction remained largely unchanged.

### 2. Parsing CSS → CSSOM

**Same as modern browsers** - CSS parsing and CSSOM construction was identical.

### 3. Construct the Render Tree

**Same as modern browsers** - Combining DOM and CSSOM into a render tree.

### 4. Layout (Reflow)

**Same as modern browsers** - Calculating positions and sizes of elements.

### 5. Painting (CPU-Based)

**This is where things got interesting!**

Instead of GPU layers, the CPU painted everything directly to a single bitmap:

```
CPU Painting Process:
┌─────────────────────────────────────┐
│ Single Bitmap Buffer               │
│ ┌─────────────────────────────────┐ │
│ │ Paint background                │ │
│ │ Paint header background         │ │
│ │ Paint header text              │ │
│ │ Paint content background        │ │
│ │ Paint content text             │ │
│ │ Paint borders, shadows, etc.   │ │
│ └─────────────────────────────────┘ │
└─────────────────────────────────────┘
```

**Key Differences:**

- **No Layers**: Everything painted to a single bitmap
- **No GPU**: All rendering done on CPU
- **Sequential Painting**: Elements painted one after another
- **No Compositing**: No separate compositing step

### 6. Direct to Screen

The final bitmap was sent directly to the screen's framebuffer.

## Performance Comparison: CPU vs GPU

### CPU-Only Rendering (1990s-2000s)

**Advantages:**

- **Simplicity**: Single-threaded, straightforward approach
- **Compatibility**: Worked on all hardware
- **No Driver Issues**: No GPU driver dependencies

**Bottlenecks:**

- **Single Thread**: All rendering on main thread
- **CPU Bound**: Limited by CPU performance
- **No Parallelism**: Sequential painting of elements
- **Memory Intensive**: Large bitmaps in system memory
- **Slow Animations**: 15-30 FPS typical
- **Blocking**: Rendering blocked by other CPU tasks

### GPU-Accelerated Rendering (Modern)

**Advantages:**

- **Parallel Processing**: GPU can handle multiple operations simultaneously
- **Hardware Acceleration**: Specialized graphics hardware
- **Layer Compositing**: Independent layers for efficiency
- **High Performance**: 60 FPS smooth animations
- **Non-Blocking**: GPU operations don't block CPU

**Bottlenecks:**

- **Driver Dependencies**: GPU driver issues
- **Memory Transfer**: CPU-GPU memory copying overhead
- **Layer Management**: Too many layers can be expensive

## Real-World Example: CPU vs GPU Rendering

Let's compare how a simple webpage would render in both eras:

### CPU-Only Approach (1990s)

```html
<div class="header">Hello World</div>
<div class="content">Welcome!</div>
```

**Rendering Steps:**

1. Parse HTML → DOM
2. Parse CSS → CSSOM
3. Create Render Tree
4. Calculate Layout
5. **CPU Painting (Sequential):**
   - Paint background color
   - Paint header background
   - Paint header text
   - Paint content background
   - Paint content text
6. Send bitmap to screen

**Performance:**

- **Time**: ~50-100ms per frame
- **FPS**: 10-20 FPS typical
- **Memory**: Large bitmap in system RAM
- **CPU Usage**: 80-100% during rendering

### GPU-Accelerated Approach (Modern)

```html
<div class="header">Hello World</div>
<div class="content">Welcome!</div>
```

**Rendering Steps:**

1. Parse HTML → DOM
2. Parse CSS → CSSOM
3. Create Render Tree
4. Calculate Layout
5. **GPU Painting (Parallel):**
   - Create background layer
   - Create header layer
   - Create content layer
6. **GPU Compositing:**
   - Combine layers efficiently
7. Send to screen

**Performance:**

- **Time**: ~16ms per frame (60 FPS)
- **FPS**: 60 FPS smooth
- **Memory**: Layers in GPU memory
- **CPU Usage**: 10-20% during rendering

## Historical Browser Examples

### Netscape Navigator (1994-2007)

- **Rendering**: CPU-only software rendering
- **Performance**: 15-30 FPS typical
- **Memory**: Large bitmaps in system RAM
- **Animations**: Limited, often choppy

### Internet Explorer 6 (2001-2006)

- **Rendering**: CPU-based with some hardware acceleration
- **Performance**: 20-40 FPS
- **Memory**: System RAM + some GPU memory
- **Animations**: Basic, often stuttering

### Modern Chrome/Firefox (2010+)

- **Rendering**: Full GPU acceleration
- **Performance**: 60 FPS smooth
- **Memory**: GPU memory for layers
- **Animations**: Smooth, hardware-accelerated

## Performance Bottlenecks Comparison

| Aspect                   | CPU-Only (1990s)   | GPU-Accelerated (Modern) |
| ------------------------ | ------------------ | ------------------------ |
| **Frame Rate**           | 15-30 FPS          | 60 FPS                   |
| **Animation Smoothness** | Choppy, stuttering | Smooth, fluid            |
| **Memory Usage**         | High (system RAM)  | Efficient (GPU memory)   |
| **CPU Usage**            | 80-100%            | 10-20%                   |
| **Parallelism**          | None               | High                     |
| **Layer Management**     | Single bitmap      | Multiple layers          |
| **Compositing**          | None               | GPU-accelerated          |

## Why the Shift to GPU?

### 1. **Performance Demands**

- Web applications became more complex
- Animations and transitions became common
- Users expected smooth 60 FPS experiences

### 2. **Hardware Evolution**

- GPUs became standard on all computers
- GPU memory became faster and larger
- GPU drivers became more stable

### 3. **Software Innovation**

- WebGL enabled GPU programming
- CSS transforms and animations
- Canvas and WebGL APIs

## Legacy Impact

The CPU-only era still influences modern web development:

### 1. **Progressive Enhancement**

- Websites still work without GPU acceleration
- Fallbacks for older hardware
- Graceful degradation

### 2. **Performance Best Practices**

- Minimize layout thrashing (still CPU-bound)
- Optimize paint operations
- Use `will-change` for GPU layers

### 3. **Mobile Considerations**

- Mobile GPUs are less powerful
- Battery life considerations
- Thermal throttling

## Modern Hybrid Approach

Today's browsers use a hybrid approach:

```
Modern Browser Rendering:
┌─────────────────────────────────────┐
│ CPU Tasks                          │
│ ├── HTML/CSS Parsing               │
│ ├── JavaScript Execution            │
│ └── Layout Calculations             │
└─────────────────────────────────────┘
                ↓
┌─────────────────────────────────────┐
│ GPU Tasks                          │
│ ├── Layer Creation                 │
│ ├── Painting                       │
│ ├── Compositing                    │
│ └── Rasterization                  │
└─────────────────────────────────────┘
```

## Key Takeaways

1. **CPU-Only Rendering** was simpler but much slower
2. **GPU Acceleration** enabled smooth 60 FPS experiences
3. **Modern browsers** use hybrid CPU/GPU approach
4. **Performance expectations** have evolved dramatically
5. **Legacy considerations** still matter for compatibility

The journey from CPU-only to GPU-accelerated rendering represents one of the most significant performance improvements in web browser history, enabling the rich, interactive web experiences we enjoy today!

Interesting evolution, isn't it ?

#### Curious ?

Now I am interested in learning how game engines evolved over the years and what can be go take away from there that can evolve the web as we experience it today, from developer as well as user perspective.

Stay tuned/
