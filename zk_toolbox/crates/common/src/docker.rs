use crate::cmd::Cmd;
use xshell::{cmd, Shell};

pub fn up(shell: &Shell, docker_compose_file: &str) -> anyhow::Result<()> {
    Cmd::new(cmd!(shell, "docker-compose -f {docker_compose_file} up -d")).run()
}
pub fn down(shell: &Shell, docker_compose_file: &str) -> anyhow::Result<()> {
    Cmd::new(cmd!(shell, "docker-compose -f {docker_compose_file} down")).run()
}
