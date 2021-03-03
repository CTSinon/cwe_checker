pub mod use_ghidra;
pub mod get_project;
use get_project::get_ir_project;
use use_ghidra::get_ghidra_result;
use cwe_checker_lib::intermediate_representation::Project;
use std::path::Path;

/// Execute the `p_code_extractor` plugin in ghidra and parse its output into the `Project` data structure.
pub fn get_project_from_ghidra(binary_path: &Path, binary: &[u8], quiet_flag: bool) -> Project {
    
    let (subprocess, fifo_path) = get_ghidra_result(binary_path);

    // Open the FIFO
    let file = std::fs::File::open(&fifo_path).expect("Could not open FIFO.");

    let project_pcode: cwe_checker_lib::pcode::Project =
        serde_json::from_reader(std::io::BufReader::new(file)).unwrap();

    subprocess.join().expect("ghidra subprocess error.");
    get_ir_project(project_pcode, binary, quiet_flag) 
}

/// get project from a json file extracted by ghidra script
pub fn get_project_from_file(file_path: &Path, binary: &[u8], quiet_flag: bool) -> Project {
    // Open the FIFO
    let file = std::fs::File::open(&file_path).expect("Could not open FIFO.");

    let project_pcode: cwe_checker_lib::pcode::Project =
        serde_json::from_reader(std::io::BufReader::new(file)).unwrap();

    get_ir_project(project_pcode, binary, quiet_flag) 
}
