mod backend;

use cwe_checker_lib::analysis::graph;
use cwe_checker_lib::utils::binary::RuntimeMemoryImage;
use cwe_checker_lib::utils::log::print_all_messages;
use cwe_checker_lib::utils::read_config_file;
use cwe_checker_lib::AnalysisResults;
use std::collections::HashSet;
use std::path::PathBuf;
use structopt::StructOpt;
use backend::{get_project_from_file, get_project_from_ghidra};
use cwe_checker_lib::intermediate_representation::Project;


#[derive(Debug, StructOpt)]
/// Find vulnerable patterns in binary executables
struct CmdlineArgs {
    /// The path to the binary.
    #[structopt(required_unless("module-versions"),  validator(check_file_existence))]
    binary: Option<String>,

    /// Path to a custom configuration file to use instead of the standard one.
    #[structopt(long, short, validator(check_file_existence))]
    config: Option<String>,

    /// Write the results to a file.
    #[structopt(long, short)]
    out: Option<String>,

    /// path to a file generated by ghidra script
    #[structopt(long)]
    project: Option<String>,

    /// Specify a specific set of checks to be run as a comma separated list, e.g. 'CWE332,CWE476,CWE782'.
    ///
    /// Use the "--module-names" command line option to get a list of all valid check names.
    #[structopt(long, short)]
    partial: Option<String>,

    /// Generate JSON output.
    #[structopt(long, short)]
    json: bool,

    /// Do not print log messages. This prevents polluting STDOUT for json output.
    #[structopt(long, short)]
    quiet: bool,

    /// Prints out the version numbers of all known modules.
    #[structopt(long)]
    module_versions: bool,

    /// Output for debugging purposes.
    /// The current behavior of this flag is unstable and subject to change.
    #[structopt(long, hidden = true)]
    debug: bool,
}

fn main() {
    let cmdline_args = CmdlineArgs::from_args();

    run_with_ghidra(cmdline_args);
}

/// Check the existence of a file
fn check_file_existence(file_path: String) -> Result<(), String> {
    if std::fs::metadata(&file_path)
        .map_err(|err| format!("{}", err))?
        .is_file()
    {
        Ok(())
    } else {
        Err(format!("{} is not a file.", file_path))
    }
}

/// Run the cwe_checker with Ghidra as its backend.
fn run_with_ghidra(args: CmdlineArgs) {
    let mut modules = cwe_checker_lib::get_modules();
    if args.module_versions {
        // Only print the module versions and then quit.
        println!("[cwe_checker] module_versions:");
        for module in modules.iter() {
            println!("{}", module);
        }
        return;
    }

    // Get the configuration file
    let config: serde_json::Value = if let Some(config_path) = args.config {
        let file = std::io::BufReader::new(std::fs::File::open(config_path).unwrap());
        serde_json::from_reader(file).expect("Parsing of the configuration file failed")
    } else {
        read_config_file("config.json")
    };

    // Filter the modules to be executed if the `--partial` parameter is set.
    if let Some(ref partial_module_list) = args.partial {
        filter_modules_for_partial_run(&mut modules, partial_module_list);
    } else {
        // TODO: CWE78 is disabled on a standard run for now,
        // because it uses up huge amounts of RAM and computation time on some binaries.
        modules = modules
            .into_iter()
            .filter(|module| module.name != "CWE78")
            .collect();
    }

    let binary_file_path = PathBuf::from(args.binary.unwrap());
    let binary: Vec<u8> = std::fs::read(&binary_file_path).unwrap_or_else(|_| {
        panic!(
            "Error: Could not read from file path {}",
            binary_file_path.display()
        )
    });

    let mut project: Project;

    if let Some(project_file_path) = args.project {
        let project_file_path = PathBuf::from(project_file_path);
        project = get_project_from_file(&project_file_path, &binary[..], args.quiet);
    } else {
        project = get_project_from_ghidra(&binary_file_path, &binary[..], args.quiet);
    }
    // Normalize the project and gather log messages generated from it.
    let mut all_logs = project.normalize();

    // Generate the representation of the runtime memory image of the binary
    let mut runtime_memory_image = RuntimeMemoryImage::new(&binary).unwrap_or_else(|err| {
        panic!("Error while generating runtime memory image: {}", err);
    });
    if project.program.term.address_base_offset != 0 {
        // We adjust the memory addresses once globally
        // so that other analyses do not have to adjust their addresses.
        runtime_memory_image.add_global_memory_offset(project.program.term.address_base_offset);
    }
    // Generate the control flow graph of the program
    let extern_sub_tids = project
        .program
        .term
        .extern_symbols
        .iter()
        .map(|symbol| symbol.tid.clone())
        .collect();
    let control_flow_graph = graph::get_program_cfg(&project.program, extern_sub_tids);

    let analysis_results = AnalysisResults::new(
        &binary,
        &runtime_memory_image,
        &control_flow_graph,
        &project,
    );

    let modules_depending_on_pointer_inference = vec!["CWE78", "CWE476", "Memory"];
    let pointer_inference_results = if modules
        .iter()
        .any(|module| modules_depending_on_pointer_inference.contains(&module.name))
    {
        Some(analysis_results.compute_pointer_inference(&config["Memory"]))
    } else {
        None
    };
    let analysis_results =
        analysis_results.set_pointer_inference(pointer_inference_results.as_ref());

    // Print debug and then return.
    // Right now there is only one debug printing function.
    // When more debug printing modes exist, this behaviour will change!
    if args.debug {
        cwe_checker_lib::analysis::pointer_inference::run(
            &project,
            &runtime_memory_image,
            &control_flow_graph,
            serde_json::from_value(config["Memory"].clone()).unwrap(),
            true,
        );
        return;
    }

    // Execute the modules and collect their logs and CWE-warnings.
    let mut all_cwes = Vec::new();
    for module in modules {
        let (mut logs, mut cwes) = (module.run)(&analysis_results, &config[&module.name]);
        all_logs.append(&mut logs);
        all_cwes.append(&mut cwes);
    }

    // Print the results of the modules.
    if args.quiet {
        all_logs = Vec::new(); // Suppress all log messages since the `--quiet` flag is set.
    }
    print_all_messages(all_logs, all_cwes, args.out.as_deref(), args.json);
}

/// Only keep the modules specified by the `--partial` parameter in the `modules` list.
/// The parameter is a comma-separated list of module names, e.g. 'CWE332,CWE476,CWE782'.
fn filter_modules_for_partial_run(
    modules: &mut Vec<&cwe_checker_lib::CweModule>,
    partial_param: &str,
) {
    let module_names: HashSet<&str> = partial_param.split(',').collect();
    *modules = module_names
        .into_iter()
        .filter_map(|module_name| {
            if let Some(module) = modules.iter().find(|module| module.name == module_name) {
                Some(*module)
            } else if module_name.is_empty() {
                None
            } else {
                panic!("Error: {} is not a valid module name.", module_name)
            }
        })
        .collect();
}




