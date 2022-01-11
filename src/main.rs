use chrono::Utc;
use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::process::Command;

type QuickResult<T = ()> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Read commands from file.
    #[clap(short, long = "arg-file")]
    arg_file: String,
}

fn main() -> QuickResult {
    let args = Args::parse();

    // Note: cannot read from STDIN as `tmux attach` expects STDIN ~= /dev/null
    let cmds = read_commands(args.arg_file)?;
    if cmds.len() == 0 {
        return Err("no commands given".into());
    }

    let now = Utc::now().format("%Y%m%dT%H%M%S_%f");
    let session_name = &format!("vixargs{}", now);

    tmux(&["new-session", "-d", "-s", session_name])?;
    tmux(&["select-window", "-t", &format!("{}:0", session_name)])?;

    let mut inited = false;
    for cmd in cmds {
        if !inited {
            inited = true;
            tmux_type(&cmd)?;
            continue;
        }

        tmux(&["split-window"])?;
        tmux_type(&cmd)?;
        tmux(&["select-layout", "tiled"])?;
    }

    let _ = Command::new("tmux")
        .arg("attach-session")
        .arg("-t")
        .arg(session_name)
        .spawn()?
        .wait();
    Ok(())
}

fn tmux_type(cmd: &str) -> QuickResult {
    let cmd = format!("{} && exit", cmd);
    tmux(&["send-keys", &cmd, "C-m"])
}

fn tmux(args: &[&str]) -> QuickResult {
    let mut cmd = Command::new("tmux");
    for tmux_arg in args {
        cmd.arg(tmux_arg);
    }

    let status = cmd.status()?;
    match status.code() {
        Some(0) => Ok(()),
        Some(_) | None => {
            let e = format!("calling tmux {} failed", args[0]);
            Err(e.into())
        }
    }
}

fn read_commands<P>(filename: P) -> QuickResult<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let mut cmds = vec![];
    for line in io::BufReader::new(file).lines() {
        let line = line?;
        cmds.push(line);
    }
    Ok(cmds)
}
