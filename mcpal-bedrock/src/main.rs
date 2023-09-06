//use console::Term;
use std::env;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;
use std::process::{Command, Stdio};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let start_server_cmd = "./bedrock_server";
    let save_hold_cmd = "save hold";
    let save_query_cmd = "save query";
    let save_resume_cmd = "save resume";

    if args.len() != 0 {
        let backup_path = &args[0];
        let bedrock_server_location  = Path::new(&args[1]);

        env::set_current_dir(&bedrock_server_location).expect("Couldn't change directory");

        let cmd_output = Command::new(start_server_cmd)
            .stdin(Stdio::null())
            .spawn()
            .unwrap();
        //println!("Status: {}", cmd_output.status);
        //std::io::stdout().write_all(&cmd_output.stdout).unwrap();
        //std::io::stderr().write_all(&cmd_output.stdout).unwrap();

        let mut server_stdin = cmd_output.stdin.unwrap();
        let mut writer = BufWriter::new(&mut server_stdin);
    }
}
