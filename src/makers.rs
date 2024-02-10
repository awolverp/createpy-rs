use colored::Colorize;

use crate::arguments;
use std::io::Write;
use std::path::PathBuf;

/// ReInitializer returning type
pub enum ReInitializerResult {
    /// This means everything is ok
    Ok,

    /// This means reinitializing is required
    Warning(String),

    /// This means an error occurred
    Err(String),
}

pub trait ReInitializer: Sized {
    /// Try to perform and initialize
    fn initialize(&self) -> ReInitializerResult;

    /// Perform and initialize forcely
    fn reinitialize(&self) -> ReInitializerResult;
}

/// Project maker, controls project directory and anything related
#[derive(Debug)]
pub struct Project {
    name: String,
    base: std::path::PathBuf,
}

impl TryFrom<arguments::ProjectArguments> for Project {
    type Error = String;

    fn try_from(value: arguments::ProjectArguments) -> Result<Self, Self::Error> {
        let mut pieces = value.0.rsplitn(2, std::path::MAIN_SEPARATOR);

        let name = String::from(pieces.next().unwrap());

        if (name.is_empty()) || (name == ".") {
            return Err(String::from("project directory cannot be empty or '.'"));
        }

        let mut base: PathBuf = PathBuf::from(".");

        if let Some(last) = pieces.next() {
            base = PathBuf::from(last);
        }

        if base.canonicalize().is_err() {
            return Err(format!("no such file or directory: {}", base.display()));
        }

        if !base.is_dir() {
            return Err(format!(
                "directory expected (for project), not file: {}",
                base.display()
            ));
        }

        Ok(Project { name, base })
    }
}

impl Project {
    /// Shorthand for `self.base.join(self.name.clone())`
    fn full_path(&self) -> PathBuf {
        self.base.join(self.name.clone())
    }
}

impl ReInitializer for Project {
    fn initialize(&self) -> ReInitializerResult {
        let target = self.full_path();

        match target.try_exists() {
            Ok(exists) => {
                if exists {
                    return ReInitializerResult::Warning(String::from(
                        "project directory is already exists",
                    ));
                }
            }
            Err(e) => {
                return ReInitializerResult::Err(e.to_string());
            }
        }

        println!("│   Creating directory: {}", target.display());
        std::fs::create_dir(target).unwrap();
        ReInitializerResult::Ok
    }

    fn reinitialize(&self) -> ReInitializerResult {
        let target = self.full_path();

        println!("│   Removing directory: {}", target.display());
        std::fs::remove_dir_all(&target).unwrap();

        println!("│   Creating directory: {}", target.display());
        std::fs::create_dir(target).unwrap();
        ReInitializerResult::Ok
    }
}

/// Git maker, controls project git configuration
#[derive(Debug)]
pub struct Git {
    user_name: String,
    user_email: String,
    remote_url: String,
    remote_name: String,
    branch: String,

    /// **Note:** this value have to be setted manually
    projectpath: PathBuf,
}

impl TryFrom<arguments::GitArguments> for Git {
    type Error = String;

    fn try_from(value: arguments::GitArguments) -> Result<Self, Self::Error> {
        Ok(Git {
            user_name: value.user_name,
            user_email: value.user_email,
            remote_url: value.remote_url,
            remote_name: if !value.remote_name.is_empty() {
                value.remote_name
            } else {
                String::from("origin")
            },
            branch: if !value.branch.is_empty() {
                value.branch
            } else {
                String::from("main")
            },
            projectpath: PathBuf::new(),
        })
    }
}

impl Git {
    /// perform `git init -b <branch>` command
    fn init(&self) -> Result<(), String> {
        println!("│   {} - git init -b {}", "Executing".purple(), self.branch);

        let mut command = std::process::Command::new("git");
        command.stdout(std::process::Stdio::null());
        command.stderr(std::process::Stdio::piped());
        command.args(["init", "-b", self.branch.as_str()]);
        command.current_dir(&self.projectpath);

        let child = match command.spawn() {
            Ok(o) => o,
            Err(_) => {
                return Err(String::from("command not found: 'git'"));
            }
        };

        let status = child.wait_with_output().unwrap();

        if !status.status.success() {
            return Err(format!(
                "{} [exit with {}]",
                String::from_utf8_lossy(&status.stderr),
                status.status.code().unwrap_or(256)
            ));
        }

        Ok(())
    }

    /// perform `git config user.name <user_name>` command
    fn config_name(&self) -> Result<(), String> {
        if self.user_name.is_empty() {
            return Ok(());
        }

        println!("│   {} - git config user.name {}", "Executing".purple(), self.user_name);

        let mut command = std::process::Command::new("git");
        command.stdout(std::process::Stdio::null());
        command.stderr(std::process::Stdio::piped());
        command.args(["config", "user.name", self.user_name.as_str()]);
        command.current_dir(&self.projectpath);

        let child = match command.spawn() {
            Ok(o) => o,
            Err(_) => {
                return Err(String::from("command not found: 'git'"));
            }
        };

        let status = child.wait_with_output().unwrap();

        if !status.status.success() {
            return Err(format!(
                "{} [exit with {}]",
                String::from_utf8_lossy(&status.stderr),
                status.status.code().unwrap_or(256)
            ));
        }

        Ok(())
    }

    /// perform `git config user.email <user_email>` command
    fn config_email(&self) -> Result<(), String> {
        if self.user_email.is_empty() {
            return Ok(());
        }

        println!("│   {} - git config user.email {}", "Executing".purple(), self.user_email);

        let mut command = std::process::Command::new("git");
        command.stdout(std::process::Stdio::null());
        command.stderr(std::process::Stdio::piped());
        command.args(["config", "user.email", self.user_email.as_str()]);
        command.current_dir(&self.projectpath);

        let child = match command.spawn() {
            Ok(o) => o,
            Err(_) => {
                return Err(String::from("command not found: 'git'"));
            }
        };

        let status = child.wait_with_output().unwrap();

        if !status.status.success() {
            return Err(format!(
                "{} [exit with {}]",
                String::from_utf8_lossy(&status.stderr),
                status.status.code().unwrap_or(256)
            ));
        }

        Ok(())
    }

    /// perform `git remote add <remote_name> <remote_url>` command
    fn config_remote(&self) -> Result<(), String> {
        if self.remote_url.is_empty() {
            return Ok(());
        }

        if self.remote_name.is_empty() {
            panic!("config_remote: self.remote_name is empty!");
        }

        println!("│   {} - git remote add {} {}", "Executing".purple(), self.remote_name, self.remote_url);

        let mut command = std::process::Command::new("git");
        command.stdout(std::process::Stdio::null());
        command.stderr(std::process::Stdio::piped());
        command.args([
            "remote",
            "add",
            self.remote_name.as_str(),
            self.remote_url.as_str(),
        ]);
        command.current_dir(&self.projectpath);

        let child = match command.spawn() {
            Ok(o) => o,
            Err(_) => {
                return Err(String::from("command not found: 'git'"));
            }
        };

        let status = child.wait_with_output().unwrap();

        if !status.status.success() {
            return Err(format!(
                "{} [exit with {}]",
                String::from_utf8_lossy(&status.stderr),
                status.status.code().unwrap_or(256)
            ));
        }

        Ok(())
    }

    /// perform all configs
    fn configure(&self) -> Result<(), String> {
        self.init()?;
        self.config_name()?;
        self.config_email()?;
        self.config_remote()
    }
}

impl ReInitializer for Git {
    fn initialize(&self) -> ReInitializerResult {
        let target = self.projectpath.join(".git");

        match target.try_exists() {
            Ok(exists) => {
                if exists {
                    return ReInitializerResult::Warning(String::from(
                        "git is already initialized",
                    ));
                }
            }
            Err(e) => {
                return ReInitializerResult::Err(e.to_string());
            }
        }

        match self.configure() {
            Ok(_) => ReInitializerResult::Ok,
            Err(e) => ReInitializerResult::Err(e),
        }
    }

    fn reinitialize(&self) -> ReInitializerResult {
        let target = self.projectpath.join(".git");

        println!("│   Removing git directory: {}", target.display());
        std::fs::remove_dir_all(target).unwrap();

        match self.configure() {
            Ok(_) => ReInitializerResult::Ok,
            Err(e) => ReInitializerResult::Err(e),
        }
    }
}

/// Virtual environment maker
#[derive(Debug)]
pub struct VirtualEnvironment {
    script: String,
    path: PathBuf,
}

impl TryFrom<arguments::VirtualEnvironmentArguments> for VirtualEnvironment {
    type Error = String;

    fn try_from(value: arguments::VirtualEnvironmentArguments) -> Result<Self, Self::Error> {
        if value.script != "venv" && value.script != "virtualenv" {
            return Err(format!(
                "not supported script for creating virtual environment: {}",
                value.script
            ));
        }

        Ok(VirtualEnvironment {
            script: value.script,
            path: if value.path.is_empty() {
                PathBuf::from(".")
            } else {
                PathBuf::from(value.path)
            },
        })
    }
}

impl VirtualEnvironment {
    /// create the virtual environment with `venv` script
    fn venv(&self) -> Result<(), String> {
        println!("│   {} - python3 -m venv {}", "Executing".purple(), self.path.display());

        let mut command = std::process::Command::new("python3");
        command.stdout(std::process::Stdio::null());
        command.stderr(std::process::Stdio::piped());
        command.args(["-m", "venv"]);
        command.arg(self.path.clone());

        let child = match command.spawn() {
            Ok(o) => o,
            Err(_) => {
                return Err(String::from("command not found: 'python3'"));
            }
        };

        let status = child.wait_with_output().unwrap();

        if !status.status.success() {
            return Err(format!(
                "{} [exit with {}]",
                String::from_utf8_lossy(&status.stderr),
                status.status.code().unwrap_or(256)
            ));
        }

        Ok(())
    }

    /// create the virtual environment with `virtualenv` script
    fn virtualenv(&self) -> Result<(), String> {
        println!("│   {} - virtualenv --no-vcs-ignore {}", "Executing".purple(), self.path.display());

        let mut command = std::process::Command::new("virtualenv");
        command.stdout(std::process::Stdio::null());
        command.stderr(std::process::Stdio::piped());
        command.args(["--no-vcs-ignore"]);
        command.arg(self.path.clone());

        let child = match command.spawn() {
            Ok(o) => o,
            Err(_) => {
                return Err(String::from("command not found: 'virtualenv'"));
            }
        };

        let status = child.wait_with_output().unwrap();

        if !status.status.success() {
            return Err(format!(
                "{} [exit with {}]",
                String::from_utf8_lossy(&status.stderr),
                status.status.code().unwrap_or(256)
            ));
        }

        Ok(())
    }

    fn create(&self) -> Result<(), String> {
        if self.script == "venv" {
            self.venv()
        } else if self.script == "virtualenv" {
            self.virtualenv()
        } else {
            Err(format!("not supported script: '{}'", self.script))
        }
    }
}

impl ReInitializer for VirtualEnvironment {
    fn initialize(&self) -> ReInitializerResult {
        let to_check = self.path.join("pyvenv.cfg");

        match to_check.try_exists() {
            Ok(exists) => {
                if exists {
                    return ReInitializerResult::Warning(String::from(
                        "virtual environment is already exists",
                    ));
                }
            }
            Err(e) => {
                return ReInitializerResult::Err(e.to_string());
            }
        }

        match self.create() {
            Ok(_) => ReInitializerResult::Ok,
            Err(e) => ReInitializerResult::Err(e),
        }
    }

    fn reinitialize(&self) -> ReInitializerResult {
        match self.create() {
            Ok(_) => ReInitializerResult::Ok,
            Err(e) => ReInitializerResult::Err(e),
        }
    }
}

pub struct MakersStructure {
    pub project: Project,
    pub git: Option<Git>,
    pub venv: Option<VirtualEnvironment>,
}

impl TryFrom<arguments::Arguments> for MakersStructure {
    type Error = String;

    fn try_from(value: arguments::Arguments) -> Result<Self, Self::Error> {
        let project: Project = match Project::try_from(value.0) {
            Ok(o) => o,
            Err(e) => {
                return Err(e);
            }
        };

        let mut result = MakersStructure {
            project,
            git: None,
            venv: None,
        };

        if let Some(g_args) = value.1 {
            match Git::try_from(g_args) {
                Ok(mut o) => {
                    o.projectpath = result.project.full_path();
                    result.git = Some(o);
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }

        if let Some(v_args) = value.2 {
            match VirtualEnvironment::try_from(v_args) {
                Ok(o) => {
                    result.venv = Some(o);
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }

        Ok(result)
    }
}

#[inline(always)]
fn confirm() -> bool {
    std::io::stdout().flush().unwrap();

    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    buf = buf.trim().to_lowercase().to_string();

    buf == "y" || buf == "yes"
}

pub fn call_reinitializer(re_t: &impl ReInitializer, no_input: bool) -> Result<(), String> {
    let mut status = re_t.initialize();

    if let ReInitializerResult::Warning(warn) = status {
        if no_input {
            println!("│   {} {}\n│   do you to create it again {}? yes", "warning:".yellow(), warn, "(y/n)".bold());
        } else {
            print!("│   {} {}\n│   do you to create it again {}? ", "warning:".yellow(), warn, "(y/n)".bold());
        }
        
        if !no_input && !confirm() {
            return Ok(());
        }

        status = re_t.reinitialize();
    }

    if let ReInitializerResult::Err(e) = status {
        return Err(e);
    }

    Ok(())
}
