use clap::clap_app;
use std::env;
use std::fs;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;

const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() {
    let matches = clap_app!(myapp =>
        (version: VERSION.unwrap())
        (author: "Emil Sayahi <limesayahi@gmail.com>")
        (@arg INPUT: "The working directory of the grabber")
    )
    .get_matches();
    env::set_current_dir(matches.value_of("INPUT").unwrap()).unwrap();
    let octocrab;
    match fs::read_to_string("./token.txt") {
        Ok(_) => {
            octocrab = octocrab::OctocrabBuilder::new()
                .personal_token(fs::read_to_string("./token.txt").unwrap())
                .build()
                .unwrap();
        }
        Err(_) => {
            octocrab = octocrab::OctocrabBuilder::new().build().unwrap();
        }
    }

    let repositories = [("Dirout", "Dokkoo")];
    let mut latest_releases = vec![];
    for repository in repositories.iter() {
        let release = octocrab
            .repos(repository.0, repository.1)
            .releases()
            .get_latest()
            .await
            .unwrap();
        latest_releases.push((repository.1, release));
    }

    for release in latest_releases {
        let file_contents = &serde_yaml::to_string(&release.1).unwrap();
        write_file(
            format!("./stable/{}.mokkf", release.0),
            format!(
                "{}collection: \"releases\"\nlayout: main\npermalink: /{{{{ page.name }}}}.html\ndate: {}\ntitle: {}\n---\n{{% raw %}}\n{}\n{{% endraw %}}",
                file_contents, release.1.published_at.to_rfc3339(), release.1.name.unwrap(), release.1.body.unwrap().replace('{', "{‎")
            ),
        )
    }
}

fn write_file(path: String, text_to_write: String) {
    fs::create_dir_all(Path::new(&path[..]).parent().unwrap()).unwrap(); // Create output path, write to file
    let file = File::create(&path).unwrap(); // Create file which we will write to
    let mut buffered_writer = BufWriter::new(file); // Create a buffered writer, allowing us to modify the file we've just created
    write!(buffered_writer, "{}", text_to_write).unwrap(); // Write String to file
    buffered_writer.flush().unwrap(); // Empty out the data in memory after we've written to the file
}
