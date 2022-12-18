use std::env;
use std::io::Write;
use std::time::{SystemTime};


fn prompt(name:&str) -> String {
  let mut line = String::new();
  print!("{}", name);
  std::io::stdout().flush().unwrap();
  std::io::stdin().read_line(&mut line).expect("Error: Could not read a line");

  return line.trim().to_string()
}

fn repl() {
  loop {
    let input = prompt("-> ");
    if input == "@now" {
      let unixtime = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
      print!("Current Unix time is {:?}\n", unixtime);
    } 
    else if input == "@exit" { 
      std::process::exit(0);
    };
  }
}
 
fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() == 1 {
    repl()
  }
  match args[1].as_str() {
    "version" => {
      let version = env!("CARGO_PKG_VERSION");
      println!("si version {}", &version);
    }
    _ => {
      return;
    }
  }
}