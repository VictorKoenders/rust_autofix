use suggestions::AppliedFixes;
pub enum RunMode {
    Unknown,
    Check,
    Build,
    Run,
    KillAndRun,
}

impl RunMode {
    pub fn to_string(&self) -> &'static str {
        match *self {
            RunMode::Unknown => "unknown",
            RunMode::Check => "check",
            RunMode::Build => "build",
            RunMode::Run => "run",
            RunMode::KillAndRun => "run",
        }
    }
}

pub struct State {
    pub working_directory: String,
    pub mode: RunMode,
    pub applied_suggestions: Vec<AppliedFixes>,
    pub output_compile_log: bool,
}

impl State {
    pub fn new() -> State {
        State {
            working_directory: String::from("."),
            mode: RunMode::Unknown,
            applied_suggestions: Vec::new(),
            output_compile_log: false,
        }
    }

    pub fn from_args<T: Iterator<Item = String>>(args: &mut T) -> Option<State> {
        let mut state = State {
            working_directory: String::from("."),
            mode: RunMode::Unknown,
            applied_suggestions: Vec::new(),
            output_compile_log: false,
        };

        while let Some(item) = args.next() {
            if item == "build" {
                state.mode = RunMode::Build;
            }
            if item == "check" {
                state.mode = RunMode::Check;
            }
            if item == "run" {
                state.mode = RunMode::Run;
            }
            if item == "kill-and-run" {
                state.mode = RunMode::KillAndRun;
            }

            if item == "--dir" {
                match args.next() {
                    Some(dir) => state.working_directory = dir.clone(),
                    None => {
                        print_usage();
                        return None;
                    }
                }
            }

            if item == "--output-log" {
                state.output_compile_log = true;
            }

            if item == "--help" || item == "-h" {
                print_usage();
                return None;
            }
        }

        Some(state)
    }
}

fn print_usage() {
    println!("cargo autofix -- --help");
    println!("cargo autofix -- -h       print this help");
    println!("cargo autofix -- <mode> [--dir <dir>] [--output-log]");
    println!("    mode        The mode that autofix will run cargo at");
    println!("                Can be either of: build, check, run, kill-and-run");
    println!("                kill-and-run will kill any previous processes before building");
    println!("    dir         The current working directory");
    println!("    output-log  Output a log \"compile.log\" at the working directory with the compile log");
}
