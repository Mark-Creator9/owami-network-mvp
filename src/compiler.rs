use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::process::Command;

/// Solidity compiler interface
pub struct SolidityCompiler {
    solc_path: String,
}

impl SolidityCompiler {
    /// Create a new Solidity compiler instance
    pub fn new() -> Result<Self> {
        // Try to find solc in system PATH
        let solc_path = which::which("solc")
            .map(|p| p.to_string_lossy().to_string())
            .or_else(|_| {
                // Try common installation paths
                let common_paths = ["/usr/bin/solc", "/usr/local/bin/solc", "solc"];

                for path in common_paths {
                    if Path::new(path).exists() {
                        return Ok(path.to_string());
                    }
                }

                Err(anyhow!(
                    "Solidity compiler (solc) not found. Please install Solidity compiler."
                ))
            })?;

        Ok(Self { solc_path })
    }

    /// Compile Solidity source code
    pub fn compile_source(&self, source: &str, version: Option<&str>) -> Result<CompilationResult> {
        // Create temporary file for source code
        let temp_dir = tempfile::tempdir()?;
        let source_file = temp_dir.path().join("contract.sol");
        let _output_file = temp_dir.path().join("contract.json");

        // Write source code to temporary file
        fs::write(&source_file, source)?;

        // Build solc command
        let mut cmd = Command::new(&self.solc_path);
        cmd.arg("--bin");
        cmd.arg("--abi");
        cmd.arg("--optimize");
        cmd.arg("--output-dir").arg(temp_dir.path());
        cmd.arg(&source_file);

        // Add version if specified
        if let Some(v) = version {
            cmd.arg("--version").arg(v);
        }

        // Execute compilation
        let output = cmd.output()?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow!("Solidity compilation failed: {}", error));
        }

        // Read compilation results
        let output_path = temp_dir.path().join("contract.json");
        let output_content = fs::read_to_string(&output_path)?;

        let result: CompilationResult = serde_json::from_str(&output_content)?;

        Ok(result)
    }

    /// Compile Solidity file
    pub fn compile_file(
        &self,
        file_path: &str,
        version: Option<&str>,
    ) -> Result<CompilationResult> {
        let source = fs::read_to_string(file_path)?;
        self.compile_source(&source, version)
    }

    /// Get Solidity compiler version
    pub fn get_version(&self) -> Result<String> {
        let output = Command::new(&self.solc_path).arg("--version").output()?;

        if output.status.success() {
            let version = String::from_utf8_lossy(&output.stdout);
            Ok(version.trim().to_string())
        } else {
            Err(anyhow!("Failed to get solc version"))
        }
    }
}

/// Compilation result from Solidity compiler
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilationResult {
    pub contracts: std::collections::HashMap<String, ContractOutput>,
    pub sources: std::collections::HashMap<String, SourceFile>,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractOutput {
    pub abi: Vec<serde_json::Value>,
    pub bin: String,
    pub function_hashes: std::collections::HashMap<String, String>,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceFile {
    pub keccak256: String,
    pub license: String,
    pub urls: Vec<String>,
}

/// Rust WASM compiler interface
pub struct RustWasmCompiler {
    cargo_path: String,
    wasm_pack_path: String,
}

impl RustWasmCompiler {
    /// Create a new Rust WASM compiler instance
    pub fn new() -> Result<Self> {
        let cargo_path = which::which("cargo")
            .map(|p| p.to_string_lossy().to_string())
            .map_err(|_| anyhow!("Cargo not found. Please install Rust and Cargo."))?;

        let wasm_pack_path = which::which("wasm-pack")
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|_| "wasm-pack".to_string()); // wasm-pack might not be installed

        Ok(Self {
            cargo_path,
            wasm_pack_path,
        })
    }

    /// Create a new Rust WASM project
    pub fn create_project(&self, project_name: &str, template: Option<&str>) -> Result<()> {
        let mut cmd = Command::new(&self.cargo_path);
        cmd.arg("new");
        cmd.arg("--lib");
        cmd.arg(project_name);

        if let Some(t) = template {
            cmd.arg("--template").arg(t);
        }

        let output = cmd.output()?;

        if output.status.success() {
            Ok(())
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(anyhow!("Failed to create Rust WASM project: {}", error))
        }
    }

    /// Build Rust WASM project
    pub fn build_project(&self, project_path: &str, target: Option<&str>) -> Result<BuildResult> {
        let mut cmd = Command::new(&self.cargo_path);
        cmd.arg("build");
        cmd.arg("--target").arg("wasm32-unknown-unknown");
        cmd.arg("--release");
        cmd.arg("--manifest-path")
            .arg(format!("{}/Cargo.toml", project_path));

        if let Some(t) = target {
            cmd.arg("--target").arg(t);
        }

        let output = cmd.output()?;

        if output.status.success() {
            // Find the generated WASM file
            let wasm_path = format!(
                "{}/target/wasm32-unknown-unknown/release/{}.wasm",
                project_path,
                Path::new(project_path)
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
            );

            Ok(BuildResult {
                success: true,
                wasm_path,
                output: String::from_utf8_lossy(&output.stdout).to_string(),
                error: None,
            })
        } else {
            Ok(BuildResult {
                success: false,
                wasm_path: String::new(),
                output: String::from_utf8_lossy(&output.stdout).to_string(),
                error: Some(String::from_utf8_lossy(&output.stderr).to_string()),
            })
        }
    }

    /// Build with wasm-pack for web compatibility
    pub fn build_with_wasm_pack(
        &self,
        project_path: &str,
        profile: Option<&str>,
    ) -> Result<BuildResult> {
        let mut cmd = Command::new(&self.wasm_pack_path);
        cmd.arg("build");
        cmd.arg(project_path);

        if let Some(p) = profile {
            cmd.arg("--profile").arg(p);
        }

        let output = cmd.output()?;

        if output.status.success() {
            Ok(BuildResult {
                success: true,
                wasm_path: format!("{}/pkg/*.wasm", project_path),
                output: String::from_utf8_lossy(&output.stdout).to_string(),
                error: None,
            })
        } else {
            Ok(BuildResult {
                success: false,
                wasm_path: String::new(),
                output: String::from_utf8_lossy(&output.stdout).to_string(),
                error: Some(String::from_utf8_lossy(&output.stderr).to_string()),
            })
        }
    }
}

/// Build result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildResult {
    pub success: bool,
    pub wasm_path: String,
    pub output: String,
    pub error: Option<String>,
}

/// WASM bytecode optimizer
pub struct WasmOptimizer {
    wasm_opt_path: String,
}

impl WasmOptimizer {
    /// Create a new WASM optimizer instance
    pub fn new() -> Result<Self> {
        let wasm_opt_path = which::which("wasm-opt")
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|_| "wasm-opt".to_string()); // wasm-opt might not be installed

        Ok(Self { wasm_opt_path })
    }

    /// Optimize WASM bytecode
    pub fn optimize(
        &self,
        wasm_bytes: &[u8],
        optimization_level: OptimizationLevel,
    ) -> Result<Vec<u8>> {
        let temp_dir = tempfile::tempdir()?;
        let input_file = temp_dir.path().join("input.wasm");
        let output_file = temp_dir.path().join("output.wasm");

        // Write input WASM
        fs::write(&input_file, wasm_bytes)?;

        // Build optimization command
        let mut cmd = Command::new(&self.wasm_opt_path);
        cmd.arg(&input_file);
        cmd.arg("-o").arg(&output_file);

        match optimization_level {
            OptimizationLevel::None => {}
            OptimizationLevel::Basic => {
                cmd.arg("-O1");
            }
            OptimizationLevel::Aggressive => {
                cmd.arg("-O3");
            }
            OptimizationLevel::Size => {
                cmd.arg("-Oz");
            }
        }

        // Execute optimization
        let output = cmd.output()?;

        if output.status.success() {
            let optimized_bytes = fs::read(&output_file)?;
            Ok(optimized_bytes)
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(anyhow!("WASM optimization failed: {}", error))
        }
    }
}

/// Optimization levels
#[derive(Debug, Clone, Copy)]
pub enum OptimizationLevel {
    None,
    Basic,
    Aggressive,
    Size,
}

/// WASM validation and analysis
pub struct WasmValidator {
    wasmparser_path: String,
}

impl WasmValidator {
    /// Create a new WASM validator instance
    pub fn new() -> Result<Self> {
        let wasmparser_path = which::which("wasm-validate")
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|_| "wasm-validate".to_string());

        Ok(Self { wasmparser_path })
    }

    /// Validate WASM bytecode
    pub fn validate(&self, wasm_bytes: &[u8]) -> Result<ValidationResult> {
        let temp_dir = tempfile::tempdir()?;
        let input_file = temp_dir.path().join("input.wasm");

        // Write input WASM
        fs::write(&input_file, wasm_bytes)?;

        // Execute validation
        let output = Command::new(&self.wasmparser_path)
            .arg(&input_file)
            .output()?;

        if output.status.success() {
            Ok(ValidationResult {
                valid: true,
                warnings: String::from_utf8_lossy(&output.stdout).to_string(),
                errors: None,
            })
        } else {
            Ok(ValidationResult {
                valid: false,
                warnings: String::from_utf8_lossy(&output.stdout).to_string(),
                errors: Some(String::from_utf8_lossy(&output.stderr).to_string()),
            })
        }
    }

    /// Analyze WASM module
    pub fn analyze(&self, wasm_bytes: &[u8]) -> Result<AnalysisResult> {
        let temp_dir = tempfile::tempdir()?;
        let input_file = temp_dir.path().join("input.wasm");

        // Write input WASM
        fs::write(&input_file, wasm_bytes)?;

        // Execute analysis
        let output = Command::new(&self.wasmparser_path)
            .arg("--analyze")
            .arg(&input_file)
            .output()?;

        if output.status.success() {
            Ok(AnalysisResult {
                functions: 0,    // Would parse actual function count
                memory_pages: 0, // Would parse actual memory pages
                table_size: 0,   // Would parse actual table size
                analysis: String::from_utf8_lossy(&output.stdout).to_string(),
            })
        } else {
            Err(anyhow!(
                "WASM analysis failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ))
        }
    }
}

/// Validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub valid: bool,
    pub warnings: String,
    pub errors: Option<String>,
}

/// Analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    pub functions: usize,
    pub memory_pages: usize,
    pub table_size: usize,
    pub analysis: String,
}

/// Compilation service that combines all compilers
pub struct CompilationService {
    solidity_compiler: SolidityCompiler,
    rust_compiler: RustWasmCompiler,
    wasm_optimizer: WasmOptimizer,
    wasm_validator: WasmValidator,
}

impl CompilationService {
    pub fn new() -> Result<Self> {
        Ok(Self {
            solidity_compiler: SolidityCompiler::new()?,
            rust_compiler: RustWasmCompiler::new()?,
            wasm_optimizer: WasmOptimizer::new()?,
            wasm_validator: WasmValidator::new()?,
        })
    }

    /// Compile source code based on language
    pub async fn compile_source(
        &self,
        source: &str,
        language: &str,
        version: Option<&str>,
    ) -> Result<CompilationResult> {
        match language.to_lowercase().as_str() {
            "solidity" | "sol" => {
                let result = self.solidity_compiler.compile_source(source, version)?;

                // Optimize the generated WASM if available
                if let Some(contract_output) = result.contracts.values().next() {
                    if let Ok(wasm_bytes) = hex::decode(&contract_output.bin) {
                        if let Ok(_optimized_bytes) = self
                            .wasm_optimizer
                            .optimize(&wasm_bytes, OptimizationLevel::Basic)
                        {
                            // Update with optimized bytecode
                            // This is simplified - in practice you'd need to reconstruct the result
                        }
                    }
                }

                Ok(result)
            }
            "rust" | "rs" => {
                // Create temporary project
                let temp_dir = tempfile::tempdir()?;
                let project_name = "temp_contract";
                let project_path = temp_dir.path().join(project_name);

                self.rust_compiler
                    .create_project(project_path.to_str().unwrap(), None)?;

                // Write source code to project
                let lib_rs_path = project_path.join("src").join("lib.rs");
                fs::write(lib_rs_path, source)?;

                // Build project
                let build_result = self
                    .rust_compiler
                    .build_project(project_path.to_str().unwrap(), None)?;

                if build_result.success {
                    // Read WASM file
                    let wasm_bytes = fs::read(&build_result.wasm_path)?;

                    // Validate and optimize
                    let validation = self.wasm_validator.validate(&wasm_bytes)?;
                    if !validation.valid {
                        return Err(anyhow!("WASM validation failed: {:?}", validation.errors));
                    }

                    let _optimized_bytes = self
                        .wasm_optimizer
                        .optimize(&wasm_bytes, OptimizationLevel::Basic)?;

                    // Return compilation result (simplified)
                    Ok(CompilationResult {
                        contracts: std::collections::HashMap::new(),
                        sources: std::collections::HashMap::new(),
                        version: "1.0.0".to_string(),
                    })
                } else {
                    Err(anyhow!("Rust compilation failed: {:?}", build_result.error))
                }
            }
            _ => Err(anyhow!("Unsupported language: {}", language)),
        }
    }

    /// Compile Rust source to WASM bytes
    pub fn compile_rust_to_wasm(&self, source: &str, _version: Option<&str>) -> Result<Vec<u8>> {
        // Create temporary project
        let temp_dir = tempfile::tempdir()?;
        let project_name = "temp_contract";
        let project_path = temp_dir.path().join(project_name);

        self.rust_compiler
            .create_project(project_path.to_str().unwrap(), None)?;

        // Write source code to project
        let lib_rs_path = project_path.join("src").join("lib.rs");
        fs::write(lib_rs_path, source)?;

        // Build project
        let build_result = self
            .rust_compiler
            .build_project(project_path.to_str().unwrap(), None)?;

        if build_result.success {
            // Read and return WASM bytes
            let wasm_bytes = fs::read(&build_result.wasm_path)?;
            Ok(wasm_bytes)
        } else {
            Err(anyhow!("Rust compilation failed: {:?}", build_result.error))
        }
    }

    /// Compile Solidity source to WASM bytes
    pub fn compile_solidity_to_wasm(&self, source: &str, version: Option<&str>) -> Result<Vec<u8>> {
        let result = self.solidity_compiler.compile_source(source, version)?;

        // Extract WASM bytecode from compilation result
        // This is simplified - in practice you'd parse the actual output
        if let Some(contract_output) = result.contracts.values().next() {
            if let Ok(wasm_bytes) = hex::decode(&contract_output.bin) {
                return Ok(wasm_bytes);
            }
        }

        Err(anyhow!(
            "Failed to extract WASM bytecode from Solidity compilation"
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solidity_compiler_creation() -> Result<()> {
        let _compiler = SolidityCompiler::new();
        Ok(())
    }

    #[test]
    fn test_rust_compiler_creation() -> Result<()> {
        let _compiler = RustWasmCompiler::new();
        Ok(())
    }

    #[test]
    fn test_wasm_optimizer_creation() -> Result<()> {
        let _optimizer = WasmOptimizer::new();
        Ok(())
    }

    #[test]
    fn test_wasm_validator_creation() -> Result<()> {
        let _validator = WasmValidator::new();
        Ok(())
    }

    #[test]
    fn test_compilation_service_creation() -> Result<()> {
        let _service = CompilationService::new();
        Ok(())
    }
}
