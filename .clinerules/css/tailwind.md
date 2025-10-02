---
description: Tailwind CSS with DaisyUI
globs: "**/*.{css,html,heex,jsx,tsx,vue}"
alwaysApply: true
---

- Use Tailwind classes exclusively, no custom CSS
- Use DaisyUI components over custom implementations
- Use theme colors via data-theme attribute, prefer semantic names (primary, secondary, accent)
- Use responsive prefixes (sm:, md:, lg:, xl:) and Tailwind spacing scale

## Examples

```html
<!-- Preferred: theme-based -->
<button class="btn btn-primary">Save</button>
<div class="card bg-base-100 shadow-xl">...</div>

<!-- Avoid: hardcoded colors -->
<button class="bg-blue-500 text-white px-4 py-2">Save</button>
<div class="bg-white shadow-lg p-6">...</div>
```
