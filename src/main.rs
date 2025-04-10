use clap::{Arg, ArgAction, Command, crate_name, crate_version, value_parser};
use futures::{StreamExt, stream::FuturesUnordered};
use ghcl::github::{self, Auth, Repository};
use indicatif::{HumanBytes, MultiProgress, ProgressBar, ProgressStyle};
use std::{fs, sync::Arc};

const ARG_OWNER: &str = "owner";
const ARG_TOKEN: &str = "token";
const ARG_CONCURRENT: &str = "concurrent";

#[tokio::main]
async fn main() {
    // Command::new("run")
    //     .about("Parse and execute a Brainfuck program from a file")
    //     .arg(
    //         Arg::new(ARG_INPUT_FILE)
    //             .help("Path to the Brainfuck source file")
    //             .index(1)
    //             .required(true),
    //     )
    //     .arg(
    //         Arg::new(ARG_MEMORY_SIZE)
    //             .help("Number of memory cells")
    //             .long(ARG_MEMORY_SIZE)
    //             .action(ArgAction::Set)
    //             .default_value(DEFAULT_MEMORY_SIZE)
    //             .value_parser(value_parser!(usize)),
    //     )
    //     .arg(
    //         Arg::new(ARG_PROFILE)
    //             .help("Collect and print program metrics")
    //             .long_help("Collect and print program metrics. Substantially increases execution time and memory usage.")
    //             .long(ARG_PROFILE)
    //             .action(ArgAction::SetTrue),
    //     )
    //     .arg(
    //         Arg::new(ARG_TIME)
    //             .help("Print parsing and execution time")
    //             .long(ARG_TIME)
    //             .action(ArgAction::SetTrue),
    //     )

    let matches = Command::new(crate_name!())
        .version(crate_version!())
        .about("GitHub Cloning Tool")
        .arg_required_else_help(true)
        .arg(
            Arg::new(ARG_OWNER)
                .help("Owner from which to clone from")
                .long(ARG_OWNER)
                .required(true),
        )
        .arg(
            Arg::new(ARG_TOKEN)
                .help("Github token")
                .long(ARG_TOKEN)
                .required(true),
        )
        .arg(
            Arg::new(ARG_CONCURRENT)
                .help("Maximum number of concurrent clones")
                .long(ARG_CONCURRENT)
                .default_value("5")
                .value_parser(value_parser!(usize)),
        )
        .get_matches();

    let owner = matches.get_one::<String>(ARG_OWNER).unwrap();
    let token = matches.get_one::<String>(ARG_TOKEN).unwrap();
    let max_concurrent = *matches.get_one::<usize>(ARG_CONCURRENT).unwrap();

    let auth = github::auth(owner.to_string(), token.to_string());
    // let repos = github::list_repos(&auth).unwrap();
    let repos: Vec<Repository> =
        serde_json::from_str(&fs::read_to_string("repos.json").unwrap()).unwrap();

    let total_bytes: u64 = repos.iter().map(|repo| repo.size_bytes).sum();
    println!("Total size: {}", HumanBytes(total_bytes));

    let auth = Arc::new(auth);
    let mut futures = FuturesUnordered::new();

    let mp = Arc::new(MultiProgress::new());

    let progress_style =
        ProgressStyle::with_template("{spinner:.cyan} {elapsed_precise:.green} {wide_msg}")
            .unwrap()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]);
    for repo in repos {
        if futures.len() >= max_concurrent {
            futures.next().await;
        }

        let auth = auth.clone();
        let pb = mp.add(ProgressBar::new_spinner());
        pb.set_style(progress_style.clone());
        pb.set_message(repo.full_name());
        pb.enable_steady_tick(std::time::Duration::from_millis(100));
        futures.push(tokio::spawn(clone(pb, auth, repo)));
    }
}

async fn clone(pb: ProgressBar, auth: Arc<Auth>, repository: Repository) {
    for _ in 0..=100 {
        tokio::time::sleep(tokio::time::Duration::from_millis(rand::random_range(
            10..40,
        )))
        .await;
    }
    if rand::random_bool(1.0 / 3.0) {
        pb.finish_with_message(format!("❌ {}", repository.full_name()));
    } else {
        pb.finish_with_message(format!("✔️ {}", repository.full_name()));
    }

    // match github::clone_repo(&auth, &repository) {
    //     Ok(_) => format!("✔️ {}", repository.full_name()),
    //     Err(_) => format!("❌ {}", repository.full_name()),
    // };
}
