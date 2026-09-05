---
title: GitHub Pages Documentation
description: Deploying documentation to GitHub Pages
---

# GitHub Pages Documentation

This project uses Jekyll with the Minima theme for documentation, deployed to GitHub Pages.

## Site URL

- **agentic-dns**: https://dedsecorg.github.io/agentic-dns/
- **agentic-route**: https://dedsecorg.github.io/agentic-route/

## Features

- Jekyll + Minima theme
- Context7 chat widget on all pages
- Markdown/MDX content from docs/ folder
- Automatic deployment on push to main

## Configuration

- `_config.yml` — Jekyll configuration
- `Gemfile` — Ruby dependencies (github-pages, jekyll-feed, jekyll-seo-tag)
- `_layouts/default.html` — Custom layout with Context7 widget
- `.github/workflows/pages.yml` — Deployment workflow

## Context7 Widget

Added to `_layouts/default.html`:

```html
<script src="https://context7.com/widget.js" data-library="/dedsecorg/agentic-dns" data-color="#059669" data-position="bottom-right"></script>
```

## Local Development

```bash
bundle install
bundle exec jekyll serve
```

Visit http://localhost:4000

EOF 2>&1
