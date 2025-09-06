use serde_json::Value;
use std::collections::HashSet;

/// Main function to generate HTML documentation from a JSON Schema
pub fn generate_html(schema: &Value) -> Result<String, crate::error::Error> {
    let mut html = String::new();

    // Main container
    html.push_str("<div class=\"schema-container\">");

    // Schema header
    html.push_str(&generate_header(schema));

    // Generate property documentation
    if schema.get("type").and_then(|v| v.as_str()) == Some("object") {
        if let Some(properties) = schema.get("properties").and_then(|v| v.as_object()) {
            html.push_str("<div class=\"properties-section\">");
            html.push_str("<h2>Properties</h2>");

            let required = get_required_fields(schema);

            html.push_str("<div class=\"properties-list\">");
            for (prop_name, prop_schema) in properties {
                let is_required = required.contains(prop_name.as_str());
                html.push_str(&generate_property_html(
                    prop_name,
                    prop_schema,
                    is_required,
                    0,
                )?);
            }
            html.push_str("</div>");
            html.push_str("</div>");
        }
    }

    // Handle array schemas
    if schema.get("type").and_then(|v| v.as_str()) == Some("array") {
        if let Some(items) = schema.get("items") {
            html.push_str("<div class=\"array-section\">");
            html.push_str("<h2>Array Items</h2>");
            html.push_str(&generate_schema_details(items, 0)?);
            html.push_str("</div>");
        }
    }

    // Handle oneOf, anyOf, allOf
    if let Some(one_of) = schema.get("oneOf").and_then(|v| v.as_array()) {
        html.push_str(&generate_compound_schema("One Of", one_of)?);
    }
    if let Some(any_of) = schema.get("anyOf").and_then(|v| v.as_array()) {
        html.push_str(&generate_compound_schema("Any Of", any_of)?);
    }
    if let Some(all_of) = schema.get("allOf").and_then(|v| v.as_array()) {
        html.push_str(&generate_compound_schema("All Of", all_of)?);
    }

    // Handle definitions/$defs
    if let Some(definitions) = schema
        .get("definitions")
        .or_else(|| schema.get("$defs"))
        .and_then(|v| v.as_object())
    {
        html.push_str("<div class=\"definitions-section\">");
        html.push_str("<h2>Definitions</h2>");
        for (def_name, def_schema) in definitions {
            html.push_str(&format!(
                "<div class=\"definition\" id=\"def-{}\">",
                escape_html(def_name)
            ));
            html.push_str(&format!("<h3>{}</h3>", escape_html(def_name)));
            html.push_str(&generate_schema_details(def_schema, 0)?);
            html.push_str("</div>");
        }
        html.push_str("</div>");
    }

    html.push_str("</div>");

    Ok(html)
}

fn generate_header(schema: &Value) -> String {
    let mut html = String::new();

    html.push_str("<div class=\"schema-header\">");

    // Only include description if present, no title
    if let Some(description) = schema.get("description").and_then(|v| v.as_str()) {
        html.push_str(&format!(
            "<p class=\"schema-description\">{}</p>",
            escape_html(description)
        ));
    }

    html.push_str("</div>");

    html
}

fn generate_property_html(
    name: &str,
    schema: &Value,
    required: bool,
    depth: usize,
) -> Result<String, crate::error::Error> {
    let mut html = String::new();

    html.push_str(&format!(
        "<div class=\"property depth-{}\" data-property=\"{}\">",
        depth,
        escape_html(name)
    ));

    html.push_str("<div class=\"property-header\">");
    html.push_str(&format!(
        "<span class=\"property-name\">{}</span>",
        escape_html(name)
    ));

    if let Some(prop_type) = get_schema_type(schema) {
        html.push_str(&format!(
            " <span class=\"type-badge\">{}</span>",
            escape_html(&prop_type)
        ));
    }

    if required {
        html.push_str(" <span class=\"required-badge\">required</span>");
    }

    html.push_str("</div>");

    if let Some(description) = schema.get("description").and_then(|v| v.as_str()) {
        html.push_str(&format!(
            "<div class=\"property-description\">{}</div>",
            escape_html(description)
        ));
    }

    html.push_str(&generate_schema_details(schema, depth + 1)?);

    html.push_str("</div>");

    Ok(html)
}

fn generate_schema_details(schema: &Value, depth: usize) -> Result<String, crate::error::Error> {
    let mut html = String::new();

    html.push_str(&format!("<div class=\"schema-details depth-{}\">", depth));

    // Constraints
    let constraints = get_constraints(schema);
    if !constraints.is_empty() {
        html.push_str("<div class=\"constraints\">");
        for constraint in constraints {
            html.push_str(&format!(
                "<span class=\"constraint\">{}</span>",
                escape_html(&constraint)
            ));
        }
        html.push_str("</div>");
    }

    // Enum values
    if let Some(enum_values) = schema.get("enum").and_then(|v| v.as_array()) {
        html.push_str("<div class=\"enum-values\">");
        html.push_str("<span class=\"enum-label\">Possible values:</span>");
        for value in enum_values {
            let val_str = format_json_value(value);
            html.push_str(&format!(
                " <span class=\"enum-value\">{}</span>",
                escape_html(&val_str)
            ));
        }
        html.push_str("</div>");
    }

    // Default value
    if let Some(default) = schema.get("default") {
        let default_str = format_json_value(default);
        html.push_str(&format!(
            "<div class=\"default-value\">Default: <code>{}</code></div>",
            escape_html(&default_str)
        ));
    }

    // Examples
    if let Some(examples) = schema.get("examples").and_then(|v| v.as_array()) {
        if !examples.is_empty() {
            html.push_str("<div class=\"examples\">");
            html.push_str("<span class=\"examples-label\">Examples:</span>");
            for example in examples {
                let ex_str = match example {
                    Value::String(s) => s.clone(),
                    v => v.to_string(),
                };
                html.push_str(&format!(" <code>{}</code>", escape_html(&ex_str)));
            }
            html.push_str("</div>");
        }
    }

    // Nested properties for objects
    if schema.get("type").and_then(|v| v.as_str()) == Some("object") {
        if let Some(properties) = schema.get("properties").and_then(|v| v.as_object()) {
            let required = get_required_fields(schema);

            html.push_str("<div class=\"nested-properties\">");
            for (prop_name, prop_schema) in properties {
                let is_required = required.contains(prop_name.as_str());
                html.push_str(&generate_property_html(
                    prop_name,
                    prop_schema,
                    is_required,
                    depth,
                )?);
            }
            html.push_str("</div>");
        }
    }

    // Array items
    if schema.get("type").and_then(|v| v.as_str()) == Some("array") {
        if let Some(items) = schema.get("items") {
            html.push_str("<div class=\"array-items\">");
            html.push_str("<div class=\"array-label\">Items:</div>");
            html.push_str(&generate_schema_details(items, depth)?);
            html.push_str("</div>");
        }
    }

    html.push_str("</div>");

    Ok(html)
}

fn generate_compound_schema(label: &str, schemas: &[Value]) -> Result<String, crate::error::Error> {
    let mut html = String::new();

    html.push_str(&format!(
        "<div class=\"compound-schema\"><h3>{}</h3>",
        label
    ));
    html.push_str("<div class=\"compound-options\">");

    for (i, schema) in schemas.iter().enumerate() {
        html.push_str(&format!(
            "<div class=\"compound-option\"><h4>Option {}</h4>",
            i + 1
        ));
        html.push_str(&generate_schema_details(schema, 0)?);
        html.push_str("</div>");
    }

    html.push_str("</div></div>");

    Ok(html)
}

fn get_schema_type(schema: &Value) -> Option<String> {
    if let Some(type_val) = schema.get("type") {
        if let Some(type_str) = type_val.as_str() {
            return Some(type_str.to_string());
        }
        if let Some(type_arr) = type_val.as_array() {
            let types: Vec<String> = type_arr
                .iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect();
            if !types.is_empty() {
                return Some(types.join(" | "));
            }
        }
    }

    if schema.get("oneOf").is_some() {
        return Some("oneOf".to_string());
    }
    if schema.get("anyOf").is_some() {
        return Some("anyOf".to_string());
    }
    if schema.get("allOf").is_some() {
        return Some("allOf".to_string());
    }
    if schema.get("$ref").is_some() {
        return Some("$ref".to_string());
    }

    None
}

fn get_required_fields(schema: &Value) -> HashSet<&str> {
    schema
        .get("required")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect())
        .unwrap_or_default()
}

fn get_constraints(schema: &Value) -> Vec<String> {
    let mut constraints = Vec::new();

    if let Some(min) = schema.get("minimum").and_then(|v| v.as_f64()) {
        constraints.push(format!("min: {}", min));
    }
    if let Some(max) = schema.get("maximum").and_then(|v| v.as_f64()) {
        constraints.push(format!("max: {}", max));
    }
    if let Some(min_len) = schema.get("minLength").and_then(|v| v.as_u64()) {
        constraints.push(format!("minLength: {}", min_len));
    }
    if let Some(max_len) = schema.get("maxLength").and_then(|v| v.as_u64()) {
        constraints.push(format!("maxLength: {}", max_len));
    }
    if let Some(pattern) = schema.get("pattern").and_then(|v| v.as_str()) {
        constraints.push(format!("pattern: {}", pattern));
    }
    if let Some(format) = schema.get("format").and_then(|v| v.as_str()) {
        constraints.push(format!("format: {}", format));
    }
    if let Some(min_items) = schema.get("minItems").and_then(|v| v.as_u64()) {
        constraints.push(format!("minItems: {}", min_items));
    }
    if let Some(max_items) = schema.get("maxItems").and_then(|v| v.as_u64()) {
        constraints.push(format!("maxItems: {}", max_items));
    }
    if schema.get("uniqueItems").and_then(|v| v.as_bool()) == Some(true) {
        constraints.push("uniqueItems".to_string());
    }
    if schema.get("exclusiveMinimum").is_some() {
        constraints.push("exclusiveMinimum".to_string());
    }
    if schema.get("exclusiveMaximum").is_some() {
        constraints.push("exclusiveMaximum".to_string());
    }

    constraints
}

fn format_json_value(value: &Value) -> String {
    match value {
        Value::String(s) => format!("\"{}\"", s),
        Value::Null => "null".to_string(),
        v => v.to_string(),
    }
}

pub fn escape_html(text: &str) -> String {
    text.chars()
        .map(|c| match c {
            '&' => "&amp;".to_string(),
            '<' => "&lt;".to_string(),
            '>' => "&gt;".to_string(),
            '"' => "&quot;".to_string(),
            '\'' => "&#39;".to_string(),
            _ => c.to_string(),
        })
        .collect()
}

// Styles and scripts are no longer embedded in the output.
// Users should provide their own CSS to style the schema markup.
// See docs/css-classes.md for documentation on available CSS classes.
