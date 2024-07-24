use std::process::Command;

use anyhow::{anyhow, bail, Result};

pub struct Postgres {}

impl Postgres {
    pub fn start(name: &str) -> Result<()> {
        let home = dirs::home_dir().ok_or(anyhow!("no HOME"))?;
        let home = home.to_str().unwrap();

        let source = format!("{home}/spesogon_pg");
        let pg_data = "/spesogon_pg";

        let output = Command::new("docker")
            .arg("run")
            .arg("--name")
            .arg(name)
            .arg("--mount")
            .arg(format!("type=bind,source={source},target={pg_data}"))
            .arg("--cap-add=SYS_PTRACE")
            .arg("--security-opt")
            .arg("seccomp=unconfined")
            .arg("-p")
            .arg("5432:5432")
            .arg("-e")
            .arg("POSTGRES_PASSWORD=1111")
            .arg("-e")
            .arg("POSTGRES_DB=spesogon")
            .arg("-e")
            .arg(format!("PGDATA={pg_data}"))
            .arg("-d")
            .arg("postgres:16.2-alpine")
            .output()?;

        if !output.status.success() {
            bail!(String::from_utf8(output.stderr).unwrap());
        }

        Ok(())
    }
}

// #!/usr/bin/env python3
//
// import os
//
//
// def run(string):
// print(string)
// if os.system(string):
// raise Exception("Shell script has failed")
//
//
// pg_data = "/spesogon_pg"
//
// source = os.path.normpath(os.path.expanduser("~/spesogon_pg"))
//
// print(source)
//
// run(f"sudo docker run \
//     --name postgres_spesogon_test \
//     --mount type=bind,source={source},target={pg_data} \
//     --cap-add=SYS_PTRACE \
//     --security-opt seccomp=unconfined \
//     -p 5432:5432 \
//     -e POSTGRES_PASSWORD=1111 \
//     -e POSTGRES_DB=spesogon \
//     -e PGDATA={pg_data} \
//     -d postgres:16.2-alpine")
