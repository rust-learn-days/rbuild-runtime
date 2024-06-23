use std::fmt;
use std::str::FromStr;

use clap::Parser;

#[derive(Parser, Debug)]
pub enum BuildRuntime {
    #[clap(name = "build", about = "build the runtime for sealos runtime images")]
    Build(BuildOpts),
}

#[derive(Parser, Debug)]
pub struct BuildOpts {
    #[clap(subcommand)]
    pub build: BuildCommand,
    #[arg(long, default_value = "v1.25.0")]
    pub kube_version: String,
    #[arg(long, default_value = "v4.3.7")]
    pub sealos_version: String,
    #[arg(long, default_value = "2.8.2")]
    pub registry_version: String,
}

#[derive(Parser, Debug)]
pub enum BuildCommand {
    #[clap(name = "docker", about = "build the runtime for docker runtime images")]
    Docker(DockerOpts),
    #[clap(
        name = "containerd",
        about = "build the runtime for containerd runtime images"
    )]
    Containerd(ContainerdOpts),
}

#[derive(Parser, Debug)]
pub struct DockerOpts {
    #[arg(default_value = "20.10.9")]
    pub version: String,
    #[arg(default_value = "v0.3.14")]
    pub cri_docker_version: String,
}

#[derive(Parser, Debug)]
pub struct ContainerdOpts {
    #[arg(long, default_value = "v1.6.23")]
    pub version: String,
    #[arg(short, long, default_value = "runc", value_parser = parse_container_runtime)]
    pub runtime: ContainerRuntime,
}

#[derive(Debug, Copy, Clone)]
pub enum ContainerRuntime {
    RunC,
    CRun,
    Youki,
}

fn parse_container_runtime(s: &str) -> Result<ContainerRuntime, &'static str> {
    s.parse()
}

impl From<ContainerRuntime> for &'static str {
    fn from(f: ContainerRuntime) -> Self {
        match f {
            ContainerRuntime::RunC => "runc",
            ContainerRuntime::CRun => "crun",
            ContainerRuntime::Youki => "youki",
        }
    }
}

impl FromStr for ContainerRuntime {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "runc" => Ok(ContainerRuntime::RunC),
            "crun" => Ok(ContainerRuntime::CRun),
            "youki" => Ok(ContainerRuntime::Youki),
            _ => Err("Invalid file format"),
        }
    }
}

impl fmt::Display for ContainerRuntime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
