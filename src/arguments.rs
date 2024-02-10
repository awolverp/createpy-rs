pub struct ProjectArguments(pub String);

pub struct GitArguments {
    pub user_name: String,
    pub user_email: String,
    pub remote_url: String,
    pub remote_name: String,
    pub branch: String,
}

impl GitArguments {
    fn new() -> GitArguments {
        GitArguments {
            user_name: String::new(),
            user_email: String::new(),
            remote_url: String::new(),
            remote_name: String::new(),
            branch: String::new(),
        }
    }
}

pub struct VirtualEnvironmentArguments {
    pub script: String,
    pub path: String,
}

impl VirtualEnvironmentArguments {
    fn new() -> VirtualEnvironmentArguments {
        VirtualEnvironmentArguments {
            script: String::new(),
            path: String::new(),
        }
    }
}

pub struct OtherArguments {
    pub reinitialize_without_input: bool
}

impl OtherArguments {
    fn new() -> OtherArguments {
        OtherArguments {
            reinitialize_without_input: false,
        }
    }
}

pub struct Arguments(
    pub ProjectArguments,
    pub Option<GitArguments>,
    pub Option<VirtualEnvironmentArguments>,
    pub OtherArguments
);

#[inline(always)]
fn create_commands() -> clap::ArgMatches {
    clap::Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .arg_required_else_help(true)
        .arg(clap::Arg::new("projectname").required(true))
        .next_line_help(true)
        // git arguments
        .next_help_heading("Git")
        .args(
            [
                clap::Arg::new("gitenabled")
                    .short('g')
                    .long("enable-git")
                    .action(clap::ArgAction::SetTrue)
                    .help("Create an empty git repository for project."),
                
                clap::Arg::new("gitname")
                    .short('u')
                    .long("git-name")
                    .value_name("name")
                    .help("Set user name for the created git repository; this option do nothing without '-g'."),

                clap::Arg::new("gitemail")
                    .short('e')
                    .long("git-email")
                    .value_name("email")
                    .help("Set email address for the created git repository; this option do nothing without '-g'."),
                
                clap::Arg::new("gitremoteurl")
                    .short('r')
                    .long("git-remote-url")
                    .value_name("URL")
                    .help("Adding new remote to the created git repository; this option do nothing without '-g'."),
                
                clap::Arg::new("gitremotename")
                    .long("git-remote-name")
                    .default_value("origin")
                    .value_name("name")
                    .help("A shortname that will be used for adding new remote; this option do nothing without '-g' and '-r'."),
                
                clap::Arg::new("gitbranch")
                    .short('b')
                    .long("git-branch")
                    .default_value("main")
                    .value_name("branch")
                    .help("A branch name for the empty git repository; this option do nothing without '-g'.")
            ]
        )
        // virtualenv arguments
        .next_help_heading("Virtual Environment")
        .args(
            [
                clap::Arg::new("venvenabled")
                    .short('v')
                    .long("disable-venv")
                    .action(clap::ArgAction::SetFalse)
                    .help("Disable virtual environment creation for project."),
                
                clap::Arg::new("venvpath")
                    .short('p')
                    .long("venv-path")
                    .default_value("virtualenv")
                    .value_name("PATH")
                    .help("Specify that where virtual environment have to create?"),
                
                clap::Arg::new("venvscript")
                    .short('s')
                    .long("venv-script")
                    .default_value("virtualenv")
                    .value_name("name")
                    .help("Use which script for creating virtual environment? 'virtualenv' and 'venv' are supported."),
            ]
        )
        // Create a custom version and help flag
        .next_help_heading("Other Options")
        .disable_version_flag(true)
        .disable_help_flag(true)
        .args(
            [
                clap::Arg::new("noinput")
                    .short('y')
                    .long("yes")
                    .action(clap::ArgAction::SetTrue)
                    .help("Automatic yes to prompts; assume 'yes' as answer to all prompts."),
                
                clap::Arg::new("version")
                    .long("version")
                    .action(clap::ArgAction::Version)
                    .help("Show version and exit."),
                
                clap::Arg::new("help")
                    .short('h')
                    .long("help")
                    .action(clap::ArgAction::Help)
                    .help("Show this help message and exit."),
            ]
        )
        .get_matches()
}


pub fn parse_args() -> Arguments {
    let matches: clap::ArgMatches = create_commands();
    let mut result = Arguments(
        ProjectArguments(String::new()),
        None,
        None,
        OtherArguments::new()
    );

    if let Some(project_name) = matches.get_one::<String>("projectname") {
        result.0.0 = (*project_name).clone();
    }

    if let Some(git_enabled) = matches.get_one::<bool>("gitenabled") {
        if *git_enabled {
            let mut cfg = GitArguments::new();

            if let Some(gitname) = matches.get_one::<String>("gitname") {
                cfg.user_name = (*gitname).clone();
            }

            if let Some(gitemail) = matches.get_one::<String>("gitemail") {
                cfg.user_email = (*gitemail).clone();
            }

            if let Some(gitremoteurl) = matches.get_one::<String>("gitremoteurl") {
                cfg.remote_url = (*gitremoteurl).clone();
            }

            if let Some(gitremotename) = matches.get_one::<String>("gitremotename") {
                cfg.remote_name = (*gitremotename).clone();
            }

            if let Some(gitbranch) = matches.get_one::<String>("gitbranch") {
                cfg.branch = (*gitbranch).clone();
            }

            result.1 = Some(cfg);
        }
    }

    if let Some(venv_enabled) = matches.get_one::<bool>("venvenabled") {
        if *venv_enabled {
            let mut cfg = VirtualEnvironmentArguments::new();

            if let Some(venvpath) = matches.get_one::<String>("venvpath") {
                cfg.path = (*venvpath).clone();
            }

            if let Some(venvscript) = matches.get_one::<String>("venvscript") {
                cfg.script = (*venvscript).clone();
            }

            result.2 = Some(cfg);
        }
    }

    if let Some(noinput) = matches.get_one::<bool>("noinput") {
        result.3.reinitialize_without_input = (*noinput).clone();
    }

    result
}
