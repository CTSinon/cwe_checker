use cwe_checker_lib::{intermediate_representation::Project, utils::log::LogMessage};
use cwe_checker_lib::pcode::Project as PcodeProject;

pub fn get_ir_project(mut pcode_project: PcodeProject, binary: &[u8], quiet_flag: bool) -> Project{
    pcode_project.normalize();
    let project: Project = match cwe_checker_lib::utils::get_binary_base_address(binary) {
        Ok(binary_base_address) => pcode_project.into_ir_project(binary_base_address),
        Err(_err) => {
            if !quiet_flag {
                let log = LogMessage::new_info("Could not determine binary base address. Using base address of Ghidra output as fallback.");
                println!("{}", log);
            }
            let mut project = pcode_project.into_ir_project(0);
            // Setting the address_base_offset to zero is a hack, which worked for the tested PE files.
            // But this hack will probably not work in general!
            project.program.term.address_base_offset = 0;
            project
        }
    };
    project
}