use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

use crate::support::cli;

#[test]
fn test_convert_help() {
    cli()
        .arg("convert")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Convert JSON Schema to HTML documentation",
        ))
        .stdout(predicate::str::contains("--input"))
        .stdout(predicate::str::contains("--output"))
        .stdout(predicate::str::contains("--no-minify"));
}

#[test]
fn test_simple_json_schema_stdin() {
    cli()
        .arg("convert")
        .write_stdin(r#"{"type":"object","properties":{"name":{"type":"string"}}}"#)
        .assert()
        .success()
        .stdout(predicate::str::contains("schema-container"))
        .stdout(predicate::str::contains("Properties"))
        .stdout(predicate::str::contains("name"))
        .stdout(predicate::str::contains("string"));
}

#[test]
fn test_simple_json_schema_file() {
    let temp_dir = TempDir::new().unwrap();
    let input_path = temp_dir.path().join("schema.json");

    fs::write(
        &input_path,
        r#"{"type":"object","properties":{"name":{"type":"string"}}}"#,
    )
    .unwrap();

    cli()
        .arg("convert")
        .arg("-i")
        .arg(&input_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("schema-container"))
        .stdout(predicate::str::contains("Properties"))
        .stdout(predicate::str::contains("name"));
}

#[test]
fn test_output_to_file() {
    let temp_dir = TempDir::new().unwrap();
    let input_path = temp_dir.path().join("schema.json");
    let output_path = temp_dir.path().join("output.html");

    fs::write(
        &input_path,
        r#"{"type":"object","properties":{"test":{"type":"string"}}}"#,
    )
    .unwrap();

    cli()
        .arg("convert")
        .arg("-i")
        .arg(&input_path)
        .arg("-o")
        .arg(&output_path)
        .assert()
        .success();

    let output = fs::read_to_string(&output_path).unwrap();
    assert!(output.contains("schema-container"));
    assert!(output.contains("test"));
}

#[test]
fn test_json_schema_with_description() {
    cli()
        .arg("convert")
        .write_stdin(r#"{"description":"This is an API schema","type":"object"}"#)
        .assert()
        .success()
        .stdout(predicate::str::contains("This is an API schema"))
        .stdout(predicate::str::contains("schema-description"));
}

#[test]
fn test_json_schema_with_required_fields() {
    cli()
        .arg("convert")
        .write_stdin(
            r#"{
            "type":"object",
            "required":["id","name"],
            "properties":{
                "id":{"type":"integer"},
                "name":{"type":"string"},
                "optional":{"type":"boolean"}
            }
        }"#,
        )
        .assert()
        .success()
        .stdout(predicate::str::contains("required-badge"))
        .stdout(predicate::str::contains("id"))
        .stdout(predicate::str::contains("name"));
}

#[test]
fn test_json_schema_with_constraints() {
    cli()
        .arg("convert")
        .write_stdin(r#"{
            "type":"object",
            "properties":{
                "age":{"type":"integer","minimum":0,"maximum":120},
                "email":{"type":"string","format":"email"},
                "username":{"type":"string","minLength":3,"maxLength":20,"pattern":"^[a-zA-Z0-9_]+$"}
            }
        }"#)
        .assert()
        .success()
        .stdout(predicate::str::contains("min: 0"))
        .stdout(predicate::str::contains("max: 120"))
        .stdout(predicate::str::contains("format: email"))
        .stdout(predicate::str::contains("minLength: 3"))
        .stdout(predicate::str::contains("maxLength: 20"))
        .stdout(predicate::str::contains("pattern:"));
}

#[test]
fn test_json_schema_with_enum() {
    cli()
        .arg("convert")
        .write_stdin(
            r#"{
            "type":"object",
            "properties":{
                "status":{"type":"string","enum":["active","inactive","pending"]}
            }
        }"#,
        )
        .assert()
        .success()
        .stdout(predicate::str::contains("Possible values"))
        .stdout(predicate::str::contains("enum-value"))
        .stdout(predicate::str::contains("active"))
        .stdout(predicate::str::contains("inactive"))
        .stdout(predicate::str::contains("pending"));
}

#[test]
fn test_json_schema_with_default() {
    cli()
        .arg("convert")
        .write_stdin(
            r#"{
            "type":"object",
            "properties":{
                "enabled":{"type":"boolean","default":true},
                "count":{"type":"integer","default":0}
            }
        }"#,
        )
        .assert()
        .success()
        .stdout(predicate::str::contains("Default:"))
        .stdout(predicate::str::contains("default-value"))
        .stdout(predicate::str::contains("true"))
        .stdout(predicate::str::contains("0"));
}

#[test]
fn test_json_schema_with_examples() {
    cli()
        .arg("convert")
        .write_stdin(r#"{
            "type":"object",
            "properties":{
                "uuid":{"type":"string","format":"uuid","examples":["123e4567-e89b-12d3-a456-426614174000"]}
            }
        }"#)
        .assert()
        .success()
        .stdout(predicate::str::contains("Examples:"))
        .stdout(predicate::str::contains("examples-label"))
        .stdout(predicate::str::contains("123e4567-e89b-12d3-a456-426614174000"));
}

#[test]
fn test_json_schema_with_nested_objects() {
    cli()
        .arg("convert")
        .write_stdin(
            r#"{
            "type":"object",
            "properties":{
                "user":{
                    "type":"object",
                    "properties":{
                        "profile":{
                            "type":"object",
                            "properties":{
                                "name":{"type":"string"}
                            }
                        }
                    }
                }
            }
        }"#,
        )
        .assert()
        .success()
        .stdout(predicate::str::contains("user"))
        .stdout(predicate::str::contains("profile"))
        .stdout(predicate::str::contains("name"))
        .stdout(predicate::str::contains("nested-properties"));
}

#[test]
fn test_json_schema_with_array() {
    cli()
        .arg("convert")
        .write_stdin(
            r#"{
            "type":"array",
            "items":{
                "type":"object",
                "properties":{
                    "id":{"type":"integer"},
                    "value":{"type":"string"}
                }
            }
        }"#,
        )
        .assert()
        .success()
        .stdout(predicate::str::contains("Array Items"))
        .stdout(predicate::str::contains("array-section"))
        .stdout(predicate::str::contains("id"))
        .stdout(predicate::str::contains("value"));
}

#[test]
fn test_json_schema_with_definitions() {
    cli()
        .arg("convert")
        .write_stdin(
            r#"{
            "type":"object",
            "definitions":{
                "address":{
                    "type":"object",
                    "properties":{
                        "street":{"type":"string"},
                        "city":{"type":"string"}
                    }
                }
            }
        }"#,
        )
        .assert()
        .success()
        .stdout(predicate::str::contains("Definitions"))
        .stdout(predicate::str::contains("definitions-section"))
        .stdout(predicate::str::contains("address"))
        .stdout(predicate::str::contains("street"))
        .stdout(predicate::str::contains("city"));
}

#[test]
fn test_json_schema_with_oneof() {
    cli()
        .arg("convert")
        .write_stdin(
            r#"{
            "oneOf":[
                {"type":"string"},
                {"type":"number"}
            ]
        }"#,
        )
        .assert()
        .success()
        .stdout(predicate::str::contains("One Of"))
        .stdout(predicate::str::contains("compound-schema"))
        .stdout(predicate::str::contains("Option 1"))
        .stdout(predicate::str::contains("Option 2"));
}

#[test]
fn test_invalid_json() {
    cli()
        .arg("convert")
        .write_stdin("not valid json")
        .assert()
        .failure()
        .stderr(predicate::str::contains("JSON error"));
}

#[test]
fn test_nonexistent_file() {
    cli()
        .arg("convert")
        .arg("-i")
        .arg("/nonexistent/path/to/file.json")
        .assert()
        .failure()
        .stderr(predicate::str::contains("[ERROR]"));
}

#[test]
fn test_html_escaping_in_descriptions() {
    cli()
        .arg("convert")
        .write_stdin(r#"{"type":"object","properties":{"test":{"type":"string","description":"<script>alert('xss')</script>"}}}"#)
        .assert()
        .success()
        .stdout(predicate::str::contains("&lt;script&gt;"))
        .stdout(predicate::str::contains("&lt;/script&gt;"));
}

#[test]
fn test_minification_default() {
    // Minification is now enabled by default
    cli()
        .arg("convert")
        .write_stdin(r#"{"type":"object","properties":{"test":{"type":"string"}}}"#)
        .assert()
        .success()
        .stdout(predicate::str::contains(
            r#"<div class="schema-container"><div class="schema-header">"#,
        ));
}

#[test]
fn test_no_minification() {
    // Test explicitly disabling minification
    cli()
        .arg("convert")
        .arg("--no-minify")
        .write_stdin(r#"{"type":"object","properties":{"test":{"type":"string"}}}"#)
        .assert()
        .success()
        // Non-minified output should have proper formatting/spacing
        .stdout(predicate::str::contains("<div class=\"schema-container\">"));
}

#[test]
fn test_multiple_types() {
    cli()
        .arg("convert")
        .write_stdin(r#"{"type":"object","properties":{"flexible":{"type":["string","null"]}}}"#)
        .assert()
        .success()
        .stdout(predicate::str::contains("string | null"));
}

#[test]
fn test_no_full_html_document() {
    cli()
        .arg("convert")
        .write_stdin(r#"{"type":"object"}"#)
        .assert()
        .success()
        .stdout(predicate::str::contains("schema-container"))
        .stdout(predicate::str::contains("<!DOCTYPE").not())
        .stdout(predicate::str::contains("<html").not())
        .stdout(predicate::str::contains("<head").not())
        .stdout(predicate::str::contains("<body").not())
        .stdout(predicate::str::contains("<style").not())
        .stdout(predicate::str::contains("<script").not());
}

#[test]
fn test_css_classes_present() {
    cli()
        .arg("convert")
        .write_stdin(
            r#"{
            "type":"object",
            "description":"Test schema",
            "required":["id"],
            "properties":{
                "id":{"type":"integer","minimum":1},
                "status":{"type":"string","enum":["active","inactive"],"default":"active"}
            }
        }"#,
        )
        .assert()
        .success()
        .stdout(predicate::str::contains("schema-container"))
        .stdout(predicate::str::contains("schema-header"))
        .stdout(predicate::str::contains("schema-description"))
        .stdout(predicate::str::contains("properties-section"))
        .stdout(predicate::str::contains("property-header"))
        .stdout(predicate::str::contains("property-name"))
        .stdout(predicate::str::contains("type-badge"))
        .stdout(predicate::str::contains("required-badge"))
        .stdout(predicate::str::contains("constraints"))
        .stdout(predicate::str::contains("constraint"))
        .stdout(predicate::str::contains("enum-values"))
        .stdout(predicate::str::contains("enum-value"))
        .stdout(predicate::str::contains("default-value"));
}

#[test]
fn test_depth_classes() {
    cli()
        .arg("convert")
        .write_stdin(
            r#"{
            "type":"object",
            "properties":{
                "level1":{
                    "type":"object",
                    "properties":{
                        "level2":{
                            "type":"object",
                            "properties":{
                                "level3":{"type":"string"}
                            }
                        }
                    }
                }
            }
        }"#,
        )
        .assert()
        .success()
        .stdout(predicate::str::contains("depth-0"))
        .stdout(predicate::str::contains("depth-1"))
        .stdout(predicate::str::contains("depth-2"));
}

#[test]
fn test_convert_with_verbose_logging() {
    cli()
        .arg("-vv")
        .arg("convert")
        .write_stdin(r#"{"type":"object"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("INFO"))
        .stderr(predicate::str::contains("Processing compilation to HTML"));
}

#[test]
fn test_convert_with_debug_logging() {
    cli()
        .arg("-L")
        .arg("debug")
        .arg("convert")
        .write_stdin(r#"{"type":"object"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("DEBUG"));
}
