use std::fs;

use amber_codegen::generate_program;
use amber_parser::build_ast_with_name;

// Helper function to read test files and generate C code
fn test_amber_file(fixture_name: &str) -> Result<String, String> {
    let fixture_path = format!("../../test_fixtures/{}.amb", fixture_name);
    let source = fs::read_to_string(&fixture_path)
        .map_err(|e| format!("Failed to read fixture file '{}': {}", fixture_path, e))?;

    let program = build_ast_with_name(&source, fixture_path.clone())
        .map_err(|e| format!("Failed to parse '{}': {}", fixture_path, e))?;

    generate_program(&program).map_err(|e| format!("Failed to generate C code: {}", e))
}

#[test]
fn test_variables_codegen() {
    let result = test_amber_file("variables").expect("Variables test should succeed");

    // Check that the generated C code contains expected elements
    assert!(result.contains("const int32_t x = 5;"));
    assert!(result.contains("int32_t y = 10;"));
    assert!(result.contains("const int32_t z = 42;"));
}

#[test]
fn test_functions_codegen() {
    let result = test_amber_file("functions").expect("Functions test should succeed");

    // Check for function declarations
    assert!(result.contains("int32_t add(int32_t a, int32_t b)"));
    assert!(result.contains("void print_hello(void)"));
    assert!(result.contains("extern void external_func(uint32_t x);"));
    assert!(result.contains("void main(void)"));
}

#[test]
fn test_structs_codegen() {
    let result = test_amber_file("structs").expect("Structs test should succeed");

    // Check for struct definition
    assert!(result.contains("typedef struct {"));
    assert!(result.contains("int32_t x;"));
    assert!(result.contains("int32_t y;"));
    assert!(result.contains("} Point;"));

    // Check for method implementations
    assert!(result.contains("int32_t Point_get_x(Point* self, int32_t value)"));
}

#[test]
fn test_control_flow_codegen() {
    let result = test_amber_file("control_flow").expect("Control flow test should succeed");

    // Check for while loop
    assert!(result.contains("while ((counter < limit))"));

    // Check for if statement
    assert!(result.contains("if ((counter > 0))"));
}

#[test]
fn test_complex_example_codegen() {
    let result = test_amber_file("complex_example").expect("Complex example test should succeed");

    // Check for main function
    assert!(result.contains("void main(void)"));

    // Check for while loop
    assert!(result.contains("while ((counter < sum))"));

    // Check for if statement
    assert!(result.contains("if ((counter > 10))"));
}

#[test]
fn test_error_handling_for_nonexistent_file() {
    let result = test_amber_file("nonexistent");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Failed to read fixture file"));
}

#[test]
fn test_invalid_syntax_error_handling() {
    // Test with a file that has invalid syntax
    let source = "const a = 1"; // Missing semicolon
    let program = build_ast_with_name(source, "test.amb".to_string());

    assert!(program.is_err());
}

#[test]
fn test_pointer() {
    let result = test_amber_file("pointer").expect("pointer test should succeed");
    println!("{}", result);
    assert!(result.contains("uint8_t* const p1;"));
    assert!(result.contains("uint8_t* p2;"));
    assert!(result.contains("const uint8_t* p3;"));
}
