use std::fs;
use std::path::{Path, PathBuf};

use amber_codegen::generate_program;
use amber_parser::build_ast;
use clap::Parser;
use miette::{Context, IntoDiagnostic, Result};

fn main() -> Result<()> {
    let cli = Cli::parse();
    let plan = CompilationPlan::from_cli(cli)?;
    let compiler = AmberCompiler::default();
    run_compilation(&compiler, plan)
}

#[derive(Parser, Debug)]
#[command(name = "amber", version, about = "Amber language CLI")]
struct Cli {
    /// Source file containing Amber code (e.g. main.amb)
    #[arg(value_name = "INPUT")]
    input: PathBuf,

    /// Optional destination for the generated C file
    #[arg(short, long, value_name = "OUTPUT")]
    output: Option<PathBuf>,
}

#[derive(Debug)]
struct CompilationPlan {
    input: PathBuf,
    output: PathBuf,
}

impl CompilationPlan {
    fn from_cli(cli: Cli) -> Result<Self> {
        let input = cli.input;
        if !input.exists() {
            return Err(miette::miette!(
                "input file '{}' does not exist",
                input.display()
            ));
        }
        if !input.is_file() {
            return Err(miette::miette!(
                "input path '{}' is not a file",
                input.display()
            ));
        }
        let output = cli.output.unwrap_or_else(|| default_output_path(&input));
        Ok(Self { input, output })
    }
}

#[derive(Default)]
struct AmberCompiler;

impl AmberCompiler {
    fn compile_from_file(&self, plan: &CompilationPlan) -> Result<String> {
        let source = fs::read_to_string(&plan.input)
            .into_diagnostic()
            .with_context(|| format!("failed to read '{}'", plan.input.display()))?;
        self.compile_source(&source, &plan.input)
    }

    fn compile_source(&self, source: &str, origin: &Path) -> Result<String> {
        let program = build_ast(source)
            .map_err(|err| miette::miette!("failed to parse '{}': {}", origin.display(), err))?;
        generate_program(&program).map_err(|err| {
            miette::miette!("failed to generate C for '{}': {}", origin.display(), err)
        })
    }
}

fn run_compilation(compiler: &AmberCompiler, plan: CompilationPlan) -> Result<()> {
    let c_code = compiler.compile_from_file(&plan)?;
    persist_output(&plan.output, &c_code)?;
    println!("Generated {}", plan.output.display());
    Ok(())
}

fn persist_output(path: &Path, contents: &str) -> Result<()> {
    if let Some(dir) = path.parent().filter(|dir| !dir.as_os_str().is_empty()) {
        fs::create_dir_all(dir)
            .into_diagnostic()
            .with_context(|| format!("failed to create directory '{}'", dir.display()))?;
    }
    fs::write(path, contents)
        .into_diagnostic()
        .with_context(|| format!("failed to write '{}'", path.display()))?;
    Ok(())
}

fn default_output_path(input: &Path) -> PathBuf {
    let mut derived = input.to_path_buf();
    derived.set_extension("c");
    derived
}
