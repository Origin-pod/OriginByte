---
slug: from-html-to-pixel
title: (1/n) From HTML to Pixel
authors: [Sheetal]
tags: [frontend, UI]
---

# From HTML to Pixel: A Deep Dive into Browser Rendering

I am very excited about this exploration! We all have been writing HTML/CSS/JavaScript since we were kids, but I wonder if we ever questioned how the computer understands these languages and renders each pixel on the screen. Let's do a deep dive into each of the steps involved in taking HTML to pixel.

## The Rendering Pipeline

### 1. Parsing the HTML → DOM

When the browser gets the HTML file, it parses it from top to bottom.

As it parses, it builds the **DOM (Document Object Model)** — a tree-like structure that represents all the HTML elements and their relationships.

#### Example DOM Tree Structure

Here's how a simple HTML structure gets converted into a DOM tree:

```html
<!DOCTYPE html>
<html>
  <head>
    <title>My Page</title>
  </head>
  <body>
    <div class="container">
      <h1>Hello World</h1>
      <p>This is a paragraph.</p>
    </div>
  </body>
</html>
```

**DOM Tree Representation:**

```
document
├── html
│   ├── head
│   │   └── title
│   │       └── "My Page"
│   └── body
│       └── div (class="container")
│           ├── h1
│           │   └── "Hello World"
│           └── p
│               └── "This is a paragraph."
```

Each node in this tree represents an HTML element, and the browser can traverse this structure to understand the document's hierarchy and relationships between elements.

### 2. Parsing CSS → CSSOM

Meanwhile, the browser loads and parses CSS (from `<style>` tags or external stylesheets).

It builds the **CSSOM (CSS Object Model)** — this maps out how styles apply to DOM elements.

#### Example CSSOM Structure

Here's how CSS rules get organized into a CSSOM tree:

```css
body {
  font-family: Arial, sans-serif;
  margin: 0;
  padding: 20px;
}

.container {
  background-color: #f0f0f0;
  border: 1px solid #ccc;
}

h1 {
  color: #333;
  font-size: 24px;
}

p {
  color: #666;
  line-height: 1.6;
}
```

**CSSOM Tree Representation:**

```
CSSOM
├── body
│   ├── font-family: Arial, sans-serif
│   ├── margin: 0
│   └── padding: 20px
├── .container
│   ├── background-color: #f0f0f0
│   └── border: 1px solid #ccc
├── h1
│   ├── color: #333
│   └── font-size: 24px
└── p
    ├── color: #666
    └── line-height: 1.6
```

The CSSOM organizes CSS rules by selector and computes the final computed values for each property.

### 3. Construct the Render Tree

The browser combines the DOM and the CSSOM to create the **Render Tree**.

This tree only includes elements that will actually appear visually (e.g., hidden elements like `display: none` don't appear here).

#### Example Render Tree Structure

The Render Tree combines the DOM and CSSOM, excluding non-visual elements:

**Original HTML with CSS:**

```html
<div class="container">
  <h1>Hello World</h1>
  <p>This is a paragraph.</p>
  <div style="display: none;">Hidden content</div>
</div>
```

```css
.container {
  background-color: #f0f0f0;
  padding: 20px;
}
h1 {
  color: #333;
}
p {
  color: #666;
}
```

**Render Tree (combines DOM + CSSOM):**

```
Render Tree
└── div (container)
    ├── computed styles:
    │   ├── background-color: #f0f0f0
    │   └── padding: 20px
    ├── h1
    │   ├── text: "Hello World"
    │   └── computed styles:
    │       └── color: #333
    └── p
        ├── text: "This is a paragraph."
        └── computed styles:
            └── color: #666
```

**Note:** The `<div style="display: none;">` element is excluded from the Render Tree because it won't be displayed visually.

### 4. Layout (Reflow)

The browser traverses the Render Tree to compute geometry:

- How big is each box?
- Where does each box go on the page?

This step is called **Layout** or **Reflow**.

It outputs a set of coordinates for each visible node.

The browser calculates the exact position and size of each element:

```
Layout Boxes:
┌─────────────────────────────────────┐
│ body (0,0) → (800,600)            │
│ ┌─────────────────────────────────┐ │
│ │ header (0,0) → (800,80)        │ │
│ │ └─ h1 (20,20) → (760,40)      │ │
│ └─────────────────────────────────┘ │
│ ┌─────────────────────────────────┐ │
│ │ content (20,100) → (760,480)   │ │
│ │ └─ p (40,120) → (740,40)      │ │
│ └─────────────────────────────────┘ │
└─────────────────────────────────────┘
```

### 5. Painting

Now the browser goes through the layout info and paints each box:

- Fills backgrounds
- Draws text
- Draws borders, shadows, images, etc.

This happens in layers.

The browser paints each element in layers:

1. **Background Layer**: Paint body background (#f5f5f5)
2. **Header Layer**: Paint header background (#333) and text (white)
3. **Content Layer**: Paint content background (white) and text (black)

### 6. Compositing

Modern browsers break the page into layers for efficient rendering.

The GPU composites these layers:

- Puts them together in the correct stacking order
- Applies transformations (like opacity and transform)

The GPU combines these layers:

- Background layer (body)
- Header layer (positioned at top)
- Content layer (positioned below header)

### 7. Rasterization → Pixels

The browser rasterizes these layers into bitmaps.

The GPU converts vector graphics and text into actual pixel data.

Finally, these pixels are sent to the screen's framebuffer.

```
Final Pixel Output:
┌─────────────────────────────────────┐
│ ████████████████████████████████████│ ← Header (dark)
│ ████ My Blog ███████████████████████│
│ ████████████████████████████████████│
│                                     │
│  ██████████████████████████████████ │ ← Content (white)
│  ████ Welcome to my blog! ██████████ │
│  ██████████████████████████████████ │
│                                     │
│  ██████████████████████████████████ │
│  ██████████████████████████████████ │
└─────────────────────────────────────┘
```

## What Happens When You Interact?

- **Scrolling**: Only compositing step repeats (GPU moves layers)
- **Hovering**: Paint + composite steps repeat (new colors)
- **JavaScript changes**: May trigger layout + paint + composite

This entire process happens in milliseconds, and the browser optimizes each step for performance!

## Key Performance Bottlenecks

- **Layout/reflow** is expensive if a lot of nodes shift
- **Paint** is expensive for complex visuals
- **Compositing** is GPU-friendly but layering too many elements can be costly

## Rendering Pipeline Summary

| Stage           | Input        | Output                       |
| --------------- | ------------ | ---------------------------- |
| **Parse HTML**  | HTML         | DOM                          |
| **Parse CSS**   | CSS          | CSSOM                        |
| **Render Tree** | DOM + CSSOM  | Render Tree                  |
| **Layout**      | Render Tree  | Geometry (boxes & positions) |
| **Paint**       | Layout boxes | Raster layers                |
| **Composite**   | Layers       | Final image on screen        |

#### Curious ?

Now the question arises, how we used to do it without GPUs ?

Let's explore that in the next article HTML to Pixel without GPUs...

Stay tuned /
