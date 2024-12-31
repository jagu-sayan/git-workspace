use anyhow::Context;
use git_workspace::commands::{
    add_provider_to_config, archive, execute_cmd, fetch, list, lock, pull_all_repositories, update,
};
use git_workspace::config::ProviderSource;
use git_workspace::display::DisplayType;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "git-workspace", author, about)]
struct Args {
    #[structopt(
        short = "w",
        long = "workspace",
        parse(from_os_str),
        env = "GIT_WORKSPACE"
    )]
    workspace: PathBuf,
    #[structopt(subcommand)]
    command: Command,
}

#[derive(StructOpt)]
enum Command {
    /// Update the workspace, removing and adding any repositories as needed.
    Update {
        #[structopt(short = "t", long = "threads", default_value = "8")]
        threads: usize,
        #[structopt(short, long, default_value = "progress")]
        /// Choose between three different outputs: progress, table, simple
        display: DisplayType,
    },
    /// Fetch new commits for all repositories in the workspace
    Fetch {
        #[structopt(short = "t", long = "threads", default_value = "8")]
        threads: usize,
        #[structopt(short, long, default_value = "progress")]
        /// Choose between three different outputs: progress, table, simple
        display: DisplayType,
    },
    /// Fetch all repositories from configured providers and write the lockfile
    Lock {},
    /// Pull new commits on the primary branch for all repositories in the workspace
    SwitchAndPull {
        #[structopt(short = "t", long = "threads", default_value = "8")]
        threads: usize,
        #[structopt(short, long, default_value = "progress")]
        /// Choose between three different outputs: progress, table, simple
        display: DisplayType,
    },
    /// List all repositories in the workspace
    ///
    /// This command will output the names of all known repositories in the workspace.
    /// Passing --full will output absolute paths.
    List {
        #[structopt(long = "full")]
        full: bool,
    },
    /// Archive repositories that don't exist in the workspace anymore.
    Archive {
        /// Disable confirmation prompt
        #[structopt(long = "force")]
        force: bool,
    },
    /// Run a git command in all repositories
    ///
    /// This command executes the "command" in all git workspace repositories.
    /// The program will receive the given "args", and have it's working directory
    /// set to the repository directory.
    Run {
        #[structopt(short = "t", long = "threads", default_value = "8")]
        threads: usize,
        #[structopt(required = true)]
        command: String,
        #[structopt(short, long, default_value = "progress")]
        /// Choose between three different outputs: progress, table, simple
        display: DisplayType,
        args: Vec<String>,
    },
    /// Add a provider to the configuration
    Add {
        #[structopt(short = "file", long = "file", default_value = "workspace.toml")]
        file: PathBuf,
        #[structopt(subcommand)]
        command: ProviderSource,
    },
}

fn main() -> anyhow::Result<()> {
    // Parse our arguments to Args using structopt.
    let args = Args::from_args();
    handle_main(args)
}

fn handle_main(args: Args) -> anyhow::Result<()> {
    // Convert our workspace path to a PathBuf. We cannot use the value given directly as
    // it could contain a tilde, so we run `expanduser` on it _if_ we are on a Unix platform.
    // On Windows this isn't supported.
    let expanded_workspace_path;
    #[cfg(not(unix))]
    {
        expanded_workspace_path = PathBuf::from(args.workspace);
    }
    #[cfg(unix)]
    {
        expanded_workspace_path = expanduser::expanduser(args.workspace.to_string_lossy())
            .with_context(|| "Error expanding git workspace path")?;
    }

    // If our workspace path doesn't exist then we need to create it, and call `canonicalize`
    // on the result. This fails if the path does not exist.
    let workspace_path = (if expanded_workspace_path.exists() {
        &expanded_workspace_path
    } else {
        fs_extra::dir::create_all(&expanded_workspace_path, false).with_context(|| {
            format!(
                "Error creating workspace directory {}",
                &expanded_workspace_path.display()
            )
        })?;
        println!(
            "Created {} as it did not exist",
            &expanded_workspace_path.display()
        );

        &expanded_workspace_path
    })
    .canonicalize()
    .with_context(|| {
        format!(
            "Error canonicalizing workspace path {}",
            &expanded_workspace_path.display()
        )
    })?;

    // Run our sub command. Pretty self-explanatory.
    match args.command {
        Command::List { full } => list(&workspace_path, full)?,
        Command::Update { threads, display } => {
            lock(&workspace_path)?;
            update(&workspace_path, display, threads)?
        }
        Command::Lock {} => {
            lock(&workspace_path)?;
        }
        Command::Archive { force } => archive(&workspace_path, force)?,
        Command::Fetch { threads, display } => fetch(&workspace_path, display, threads)?,
        Command::Add { file, command } => add_provider_to_config(&workspace_path, command, &file)?,
        Command::Run {
            threads,
            command,
            display,
            args,
        } => execute_cmd(&workspace_path, display, threads, command, args)?,
        Command::SwitchAndPull { threads, display } => {
            pull_all_repositories(&workspace_path, display, threads)?
        }
    };
    Ok(())
}
