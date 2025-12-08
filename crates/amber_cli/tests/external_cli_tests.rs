use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

use amber_cli::{AmberCompiler, CompilationPlan, run_compilation};

#[test]
fn test_cli_compilation_from_file_success() {
    // Create a temporary directory for our test
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let input_path = temp_dir.path().join("test_input.amb");

    // Write a simple Amber program to the temp file
    let test_program = r#"
fn main() {
    const x: i32 = 42;
}
"#;
    fs::write(&input_path, test_program).expect("Failed to write test file");

    // Create a compilation plan
    let output_path = temp_dir.path().join("output.c");
    let plan = CompilationPlan {
        input: input_path,
        output: output_path.clone(),
    };

    // Run the full compilation pipeline (parse, generate, write file)
    let compiler = AmberCompiler::default();
    let result = run_compilation(&compiler, plan);

    // Print the error if compilation failed
    if result.is_err() {
        eprintln!("Compilation error: {:?}", result.as_ref().err());
    }

    // Verify compilation and file writing succeeded
    assert!(result.is_ok(), "Compilation should succeed: {:?}", result.err());

    // Verify the output file was created and contains expected content
    let output_content = fs::read_to_string(&output_path).expect("Failed to read output file");
    assert!(output_content.contains("const int32_t x = 42;"));
}

#[test]
fn test_cli_compilation_with_variables() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let input_path = temp_dir.path().join("variables_test.amb");

    // Write a program with various variable types to the temp file
    let test_program = r#"
fn main() {
    const x: i32 = 5;
    var y: i32 = 10;
    comptime const z: i32 = 42;
}
"#;
    fs::write(&input_path, test_program).expect("Failed to write test file");

    let output_path = temp_dir.path().join("variables_output.c");
    let plan = CompilationPlan {
        input: input_path,
        output: output_path.clone(),
    };

    let compiler = AmberCompiler::default();
    let result = run_compilation(&compiler, plan);

    // Print the error if compilation failed
    if result.is_err() {
        eprintln!("Compilation error: {:?}", result.as_ref().err());
    }

    assert!(result.is_ok(), "Compilation should succeed: {:?}", result.err());

    let output_content = fs::read_to_string(&output_path).expect("Failed to read output file");
    assert!(output_content.contains("const int32_t x = 5;"));
    assert!(output_content.contains("int32_t y = 10;"));
    assert!(output_content.contains("const int32_t z = 42;"));
}

#[test]
fn test_cli_compilation_with_functions() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let input_path = temp_dir.path().join("functions_test.amb");

    // Write a program with functions to the temp file
    let test_program = r#"
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

extern fn external_func(x: u32);

fn main() {
    const a: i32 = 2;
    const b: i32 = 3;
    const result: i32 = a + b;
}
"#;
    fs::write(&input_path, test_program).expect("Failed to write test file");

    let output_path = temp_dir.path().join("functions_output.c");
    let plan = CompilationPlan {
        input: input_path,
        output: output_path.clone(),
    };

    let compiler = AmberCompiler::default();
    let result = run_compilation(&compiler, plan);

    // Print the error if compilation failed
    if result.is_err() {
        eprintln!("Compilation error: {:?}", result.as_ref().err());
    }

    assert!(result.is_ok(), "Compilation should succeed: {:?}", result.err());

    let output_content = fs::read_to_string(&output_path).expect("Failed to read output file");
    assert!(output_content.contains("int32_t add(int32_t a, int32_t b)"));
    assert!(output_content.contains("extern void external_func(uint32_t x);"));
}

#[test]
fn test_cli_compilation_with_while_loop() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let input_path = temp_dir.path().join("while_test.amb");

    // Write a program with a while loop to the temp file (inside a function)
    let test_program = r#"
fn main() {
    var counter: i32 = 0;
    const limit: i32 = 5;

    while counter < limit {
        counter = counter + 1;
    }
}
"#;
    fs::write(&input_path, test_program).expect("Failed to write test file");

    let output_path = temp_dir.path().join("while_output.c");
    let plan = CompilationPlan {
        input: input_path,
        output: output_path.clone(),
    };

    let compiler = AmberCompiler::default();
    let result = run_compilation(&compiler, plan);

    // Print the error if compilation failed
    if result.is_err() {
        eprintln!("Compilation error: {:?}", result.as_ref().err());
    }

    assert!(result.is_ok(), "Compilation should succeed: {:?}", result.err());

    let output_content = fs::read_to_string(&output_path).expect("Failed to read output file");
    assert!(output_content.contains("while ((counter < limit))"));
}

#[test]
fn test_cli_file_not_found_error() {
    // Create a path to a non-existent file
    let input_path = PathBuf::from("this_file_does_not_exist.amb");
    
    let output_path = PathBuf::from("/tmp/output.c");
    let plan = CompilationPlan {
        input: input_path,
        output: output_path,
    };
    
    let compiler = AmberCompiler::default();
    let result = compiler.compile_from_file(&plan);
    
    // This should fail because the file doesn't exist
    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("failed to read"));
}

#[test]
fn test_cli_invalid_syntax_error() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let input_path = temp_dir.path().join("invalid_syntax.amb");
    
    // Write a program with invalid syntax to the temp file
    let test_program = "const a = 1";  // Missing semicolon
    fs::write(&input_path, test_program).expect("Failed to write test file");
    
    let output_path = temp_dir.path().join("invalid_output.c");
    let plan = CompilationPlan {
        input: input_path,
        output: output_path,
    };
    
    let compiler = AmberCompiler::default();
    let result = compiler.compile_from_file(&plan);
    
    // This should fail because of invalid syntax
    assert!(result.is_err());
}