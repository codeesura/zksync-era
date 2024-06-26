use crate::{cmd::Cmd, logger};
use xshell::{cmd, Shell};

const PREREQUISITES: [Prerequisite; 6] = [
    Prerequisite {
        name: "git",
        download_link: "https://git-scm.com/book/en/v2/Getting-Started-Installing-Git",
    },
    Prerequisite {
        name: "docker",
        download_link: "https://docs.docker.com/get-docker/",
    },
    Prerequisite {
        name: "docker-compose",
        download_link: "https://docs.docker.com/compose/install/",
    },
    Prerequisite {
        name: "forge",
        download_link: "https://book.getfoundry.sh/getting-started/installation",
    },
    Prerequisite {
        name: "cargo",
        download_link: "https://doc.rust-lang.org/cargo/getting-started/installation.html",
    },
    Prerequisite {
        name: "yarn",
        download_link: "https://yarnpkg.com/getting-started/install",
    },
];

struct Prerequisite {
    name: &'static str,
    download_link: &'static str,
}

pub fn check_prerequisites(shell: &Shell) {
    let mut missing_prerequisites = vec![];

    for prerequisite in &PREREQUISITES {
        if !check_prerequisite(shell, prerequisite.name) {
            missing_prerequisites.push(prerequisite);
        }
    }

    if !missing_prerequisites.is_empty() {
        logger::error("Prerequisite check has failed");
        logger::error_note(
            "The following prerequisites are missing",
            &missing_prerequisites
                .iter()
                .map(|prerequisite| {
                    format!("- {} ({})", prerequisite.name, prerequisite.download_link)
                })
                .collect::<Vec<_>>()
                .join("\n"),
        );
        logger::outro("Failed");
        std::process::exit(1);
    }
}

fn check_prerequisite(shell: &Shell, name: &str) -> bool {
    Cmd::new(cmd!(shell, "which {name}")).run().is_ok()
}
