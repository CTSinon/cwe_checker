pub mod binary;
pub mod graph_utils;
pub mod log;
pub mod symbol_utils;

use crate::prelude::*;

/// Get the contents of a configuration file.
pub fn read_config_file(filename: &str) -> serde_json::Value {
    let project_dirs = directories::ProjectDirs::from("", "", "cwe_checker")
        .expect("Could not discern location of configuration files.");
    let config_dir = project_dirs.config_dir();
    let config_path = config_dir.join(filename);
    println!("{}", config_path.to_str().unwrap());
    let config_file =
        std::fs::read_to_string(config_path).expect("Could not read configuration file");
    serde_json::from_str(&config_file).unwrap()
}

/// Get the base address for the image of a binary when loaded into memory.
pub fn get_binary_base_address(binary: &[u8]) -> Result<u64, Error> {
    use goblin::Object;
    match Object::parse(binary)? {
        Object::Elf(elf_file) => {
            for header in elf_file.program_headers.iter() {
                let vm_range = header.vm_range();
                if !vm_range.is_empty() && header.p_type == goblin::elf::program_header::PT_LOAD {
                    // The loadable segments have to occur in order in the program header table.
                    // So the start address of the first loadable segment is the base offset of the binary.
                    return Ok(vm_range.start as u64);
                }
            }
            Err(anyhow!("No loadable segment bounds found."))
        }
        _ => Err(anyhow!("Binary type not yet supported")),
    }
}
