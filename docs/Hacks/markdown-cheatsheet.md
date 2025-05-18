---
sidebar_position: 2
---

# Markdown Cheatsheet

This guide provides a quick reference for all the essential Markdown syntax you'll need while working with documentation.

## Basic Syntax

### Headings

```markdown
# Heading 1

## Heading 2

### Heading 3

#### Heading 4

##### Heading 5

###### Heading 6
```

### Text Formatting

```markdown
_Italic text_ or _Italic text_
**Bold text** or **Bold text**
**_Bold and italic text_**
~~Strikethrough text~~
```

### Lists

#### Unordered Lists

```markdown
- Item 1
- Item 2
  - Subitem 2.1
  - Subitem 2.2

* Alternative bullet

- Another alternative
```

#### Ordered Lists

```markdown
1. First item
2. Second item
3. Third item
   1. Subitem 3.1
   2. Subitem 3.2
```

### Links

```markdown
[Link text](https://www.example.com)
[Link with title](https://www.example.com "Title text")
```

### Images

```markdown
![Alt text](image.jpg)
![Alt text](image.jpg "Image title")
```

### Code

#### Inline Code

```markdown
`inline code`
```

#### Code Blocks

````markdown
```javascript
const greeting = "Hello, World!";
console.log(greeting);
```
````

### Blockquotes

```markdown
> This is a blockquote
>
> > This is a nested blockquote
```

### Horizontal Rules

```markdown
---

---

---
```

## Advanced Syntax

### Tables

```markdown
| Header 1 | Header 2 | Header 3 |
| -------- | -------- | -------- |
| Cell 1   | Cell 2   | Cell 3   |
| Cell 4   | Cell 5   | Cell 6   |
```

### Task Lists

```markdown
- [x] Completed task
- [ ] Pending task
- [ ] Another task
```

### Footnotes

```markdown
Here's a sentence with a footnote. [^1]

[^1]: This is the footnote.
```

### Escaping Characters

```markdown
\* Asterisk
\` Backtick
\# Hash
```

## Docusaurus-Specific Features

### Frontmatter

```markdown
---
sidebar_position: 1
title: Page Title
---

Content starts here...
```

### Admonitions

```markdown
:::tip
This is a tip!
:::

:::note
This is a note!
:::

:::warning
This is a warning!
:::

:::danger
This is a danger message!
:::
```

### Tabs

```markdown
import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';

<Tabs>
  <TabItem value="first" label="First Tab">
    Content for first tab
  </TabItem>
  <TabItem value="second" label="Second Tab">
    Content for second tab
  </TabItem>
</Tabs>
```

## Best Practices

1. **Consistency**: Use consistent formatting throughout your documentation
2. **Spacing**: Add blank lines between different elements
3. **Lists**: Use proper indentation for nested lists
4. **Links**: Use descriptive link text
5. **Images**: Always include alt text for accessibility
6. **Code**: Specify the language for code blocks for proper syntax highlighting

## Common Issues and Solutions

### Line Breaks

To create a line break, end a line with two spaces or use `<br>` tag.

### Lists with Code

When including code in lists, indent the code block with 8 spaces or 2 tabs.

### Tables Alignment

You can align table columns using colons:

```markdown
| Left-aligned | Center-aligned | Right-aligned |
| :----------- | :------------: | ------------: |
```

Remember: Markdown is designed to be readable even in its raw form. When in doubt, keep it simple and clear!
