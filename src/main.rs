use std::{
    env,
    path::Path,
    io::{self, BufRead, Write},
    process::{Command, Stdio}
};


fn main() {
    let stdin = io::stdin();

    'main: loop {
        let mut code = String::from("from pysh import *\n");

        loop {
            let input = stdin.lock().lines().next().unwrap().unwrap();

            code.push_str(&format!("{input}\n"));

            if input.ends_with("\t") {
                let mut splited = input.split_whitespace();

                match splited.next() {
                    Some("cd") => {
                        let newdir = splited.peekable().peek().map_or("/", |x| *x);
                        let root = Path::new(newdir);

                        if let Err(e) = env::set_current_dir(&root) {
                            eprintln!("{}", e);
                        }

                        continue 'main;
                    },
                    _ => break
                }
            }
        }

        let mut python = Command::new("python")
            .stdin(Stdio::piped())
            .spawn()
            .unwrap();

        let mut stdin = python.stdin.take().unwrap();
        std::thread::spawn(move || {
            stdin.write_all(code.as_bytes()).unwrap()
        });
        python.wait();
    }
}
