//! Stands in for /usr/bin/open and $BROWSER: web URLs go to the Firefox profile
//! named by FIREFOX_PROFILE, everything else passes through to /usr/bin/open.
use std::env;
use std::os::unix::process::CommandExt;
use std::process::{Command, Stdio};

const FIREFOX: &str = "/Applications/Firefox.app/Contents/MacOS/firefox";

fn is_web_url(arg: &str) -> bool {
    arg.starts_with("http://") || arg.starts_with("https://")
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let profile = env::var("FIREFOX_PROFILE").ok().filter(|p| !p.is_empty());

    if let Some(profile) = profile {
        if !args.is_empty() && args.iter().all(|a| is_web_url(a)) {
            let spawned = Command::new(FIREFOX)
                .args(["-P", &profile, "--new-tab"])
                .args(&args)
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .process_group(0)
                .spawn();
            if spawned.is_ok() {
                return;
            }
        }
    }

    let err = Command::new("/usr/bin/open").args(&args).exec();
    eprintln!("ff-open: failed to exec /usr/bin/open: {err}");
    std::process::exit(1);
}
