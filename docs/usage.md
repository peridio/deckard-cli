# Usage

## Convert Command

Convert JSON Schema to HTML documentation markup:

```bash
# Read from stdin, write to stdout
cat schema.json | deckard convert > doc.html

# Read from file, write to file
deckard convert -i schema.json -o documentation.html

# Don't minify the output (minification is the default)
deckard convert -i schema.json --no-minify -o doc.html
```

## Output Format

The `convert` command generates semantic HTML markup for JSON schemas, not a complete HTML document. The output:

- Contains only the schema structure markup
- Does not include any CSS styles or JavaScript
- Uses semantic CSS classes for easy styling
- Is designed to be embedded into existing web pages

For documentation on the CSS classes used in the markup, see [docs/css-classes.md](docs/css-classes.md).

Example output structure:
```html
<div class="schema-container">
  <div class="schema-header">
    <p class="schema-description">...</p>
  </div>
  <div class="properties-section">
    <h2>Properties</h2>
    <div class="properties-list">
      <!-- Property definitions -->
    </div>
  </div>
</div>
```
