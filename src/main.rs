use std::process::Command;
use std::fs;
use std::io::Write;
use std::path::Path;

fn main() {
    println!("🌐 Archiving Origin Repositories");fetch_repositories();
    println!("🌐 Archiving Origin Gists");fetch_gists();
}

fn fetch_repositories() {
    let stdout = Command::new("gh")
        .args(["repo", "ls"])
        .output()
        .expect("Failed to execute command");
    let stdout_str = String::from_utf8_lossy(&stdout.stdout);

    for line in stdout_str.lines() {
        let line = line.trim();
        let columns: Vec<&str> = line.split('\t').collect();
        let repo_name = columns[0];

        print!(
            "{name}...    ",
            name = repo_name
        );       

        std::io::stdout()
            .flush()
            .expect("flushing current archive target");

        let _stdout = Command::new("gh")
            .args(["repo", "clone", repo_name])
            .output()
            .expect("Failed to execute command");
        print!("done ⭐\n");
    }
}

fn fetch_gists() {
     let stdout = Command::new("gh")
        .args(["api", "user", "--jq", ".login"])
        .output()
        .expect("Failed to execute command");
    let mut gh_username = String::from_utf8_lossy(&stdout.stdout).to_string();
    gh_username.pop();

    if !Path::new("gists/").exists() {
        fs::create_dir("gists/").unwrap();
    }

    let stdout = Command::new("gh")
        .args(["gist", "ls"])
        .output()
        .expect("Failed to execute command");
    let stdout_str = String::from_utf8_lossy(&stdout.stdout);

    for line in stdout_str.lines() {
        let line = line.trim();
        let columns: Vec<&str> = line.split('\t').collect();
        let (gist_id, gist_name) = (columns[0], columns[1]);

        print!(
            "{username}/gists/{name}...    ",
            username = gh_username,
            name = gist_name
        );

        std::io::stdout()
            .flush()
            .expect("flushing current archive target");

        let gist_filename = format!("gists/{}", gist_name);

        if !Path::new(gist_filename.as_str()).exists() {
            let _stdout = Command::new("gh")
                .args([
                    "gist",
                    "clone",
                    gist_id,
                    format!("gists/{}", gist_filename).as_str(),
                ])
                .output()
                .expect("Failed to execute command");
            print!("done ⭐\n");
        } else {
            print!("already installed ✅\n")
        }
    }
}
