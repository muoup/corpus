# Corpus Documentation

This directory contains the MkDocs site for the Corpus project.

## Structure

- `index.md` - Landing page and quick start guide
- `infrastructure/` - Stable reference documentation (hard-text)
- `research/` - Experimental findings and explorations (soft-text)
- `usage/` - How-to guides and examples
- `mkdocs.yml` - MkDocs configuration

## Building the Documentation

### Install MkDocs

```bash
# Using pip
pip install mkdocs mkdocs-material

# Or using pip3
pip3 install mkdocs mkdocs-material
```

### Preview Locally

```bash
cd docs
mkdocs serve
```

Then open `http://127.0.0.1:8000` in your browser.

The site will auto-reload as you edit markdown files.

### Build Static Site

```bash
cd docs
mkdocs build
```

Output will be in `site/` directory.

### Deploy

The `site/` directory can be deployed to:
- GitHub Pages
- Netlify
- Any static web host

## Content Philosophy

### Infrastructure (Hard-text)
Stable, reference documentation that changes infrequently:
- Architecture documentation
- API references
- Crate documentation
- Design principles

### Research (Soft-text)
Exploratory, evolving documentation:
- Stream-of-consciousness notes
- Experimental findings
- Conceptual explorations
- Non-linear idea development

### Usage (How-to)
Practical guides for using the system:
- Getting started
- Logging and debugging
- Examples and tutorials

## Editing Guidelines

1. **Don't change existing text** in research files (per instruction)
2. **Add new sections** to document new findings
3. **Cross-reference** between infrastructure and research
4. **Use markdown** for all content (no HTML needed)
5. **Preview changes** with `mkdocs serve` before committing

## See Also

- [Project README](../README.md): Project overview at repository root
- [CRATES.md](infrastructure/crates.md): Original crate documentation
