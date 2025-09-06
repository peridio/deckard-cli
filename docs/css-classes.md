# CSS Classes Reference

This document describes the CSS classes used in the generated schema markup. The output is designed to be easily styled according to your needs.

## Container classes

### `.schema-container`
The root container element that wraps all schema documentation.

### `.schema-header`
Contains the schema's top-level description and metadata.

### `.schema-description`
A paragraph element containing the schema's description text.

## Property classes

### `.properties-section`
Container for the properties list in object schemas.

### `.properties-list`
Direct wrapper for all property elements.

### `.property`
Individual property container. Includes depth modifiers:
- `.property.depth-0` - Root level property
- `.property.depth-1` - First level nested property
- `.property.depth-2` - Second level nested property
- And so on for deeper nesting

### `.property-header`
Container for the property name and badges.

### `.property-name`
The property's name/key.

### `.property-description`
Description text for a property.

### `.nested-properties`
Container for nested object properties.

## Type and validation classes

### `.type-badge`
Badge displaying the property's type (string, number, object, etc.).

### `.required-badge`
Badge indicating a property is required.

### `.constraints`
Container for constraint badges.

### `.constraint`
Individual constraint badge (e.g., "minLength: 3", "maximum: 100").

### `.schema-details`
Container for detailed schema information. Includes depth modifiers like `.property`.

## Enum and values classes

### `.enum-values`
Container for enumerated values.

### `.enum-label`
Label text "Possible values:".

### `.enum-value`
Individual enum value badge.

### `.default-value`
Container for default value display.

### `.examples`
Container for example values.

### `.examples-label`
Label text "Examples:".

## Array classes

### `.array-section`
Container for array schema documentation.

### `.array-items`
Container for array item schema.

### `.array-label`
Label text "Items:".

## Compound schema classes

### `.compound-schema`
Container for oneOf/anyOf/allOf schemas.

### `.compound-options`
Container for the list of schema options.

### `.compound-option`
Individual schema option in a compound schema.

## Definition classes

### `.definitions-section`
Container for schema definitions/$defs.

### `.definition`
Individual definition container. Has an `id` attribute formatted as `def-{name}`.

## Element structure

The typical structure of the generated markup:

```html
<div class="schema-container">
  <div class="schema-header">
    <p class="schema-description">...</p>
  </div>
  
  <div class="properties-section">
    <h2>Properties</h2>
    <div class="properties-list">
      <div class="property depth-0" data-property="propertyName">
        <div class="property-header">
          <span class="property-name">propertyName</span>
          <span class="type-badge">string</span>
          <span class="required-badge">required</span>
        </div>
        <div class="property-description">...</div>
        <div class="schema-details depth-1">
          <div class="constraints">
            <span class="constraint">minLength: 1</span>
          </div>
          <div class="enum-values">
            <span class="enum-label">Possible values:</span>
            <span class="enum-value">value1</span>
          </div>
          <div class="default-value">Default: <code>defaultValue</code></div>
        </div>
      </div>
    </div>
  </div>
</div>
```

## Styling tips

1. **Depth modifiers** - Use the `.depth-N` classes to create visual hierarchy through indentation or other styling.

2. **Data attributes** - Properties include `data-property` attributes with the property name for additional styling hooks.

3. **Semantic HTML** - The markup uses semantic elements like `<h2>`, `<h3>`, `<h4>`, `<p>`, `<span>`, and `<code>` that can be styled directly or in combination with classes.

4. **Badge styling** - The `.type-badge`, `.required-badge`, `.constraint`, and `.enum-value` classes are designed to be styled as badges or chips.

5. **Code elements** - Default values and examples are wrapped in `<code>` tags for monospace font styling.

## Example template

For a complete working example of how to style and use the generated markup, see [example-template.html](example-template.html). This template includes:

- Complete CSS styling for all classes
- JavaScript for collapsible nested properties
- Click-to-copy functionality for code blocks
- Responsive design considerations