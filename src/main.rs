use futures::{StreamExt, stream::FuturesUnordered};
use ghcl::github::{self, Auth, Repository};
use indicatif::{HumanBytes, MultiProgress, ProgressBar, ProgressStyle};
use std::{fs, sync::Arc};

#[tokio::main]
async fn main() {
    let username = "LimeEng";
    let token = "";
    let max_concurrent = 5;

    let auth = github::auth(username.to_string(), token.to_string());
    // let repos = github::list_repos(&auth).unwrap();
    let repos: Vec<Repository> =
        serde_json::from_str(&fs::read_to_string("repos.json").unwrap()).unwrap();

    let total_bytes: u64 = repos.iter().map(|repo| repo.disk_usage).sum();
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
        let mp = Arc::clone(&mp);
        let pb = mp.add(ProgressBar::new_spinner());
        pb.set_style(progress_style.clone());
        pb.set_message(repo.full_name());
        pb.enable_steady_tick(std::time::Duration::from_millis(100));
        futures.push(tokio::spawn(clone(pb, auth, repo)));
    }
}

async fn clone(pb: ProgressBar, auth: Arc<Auth>, repository: Repository) {
    // Simulate task work
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
