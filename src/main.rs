use std::task::Context;

// step1
use clap::Parser;
#[derive(Parser)]

// step1
// struct Cli {
//     pattern: String,
//     text: String,
// }

// step2
struct Cli {
    pattern: String,
    #[clap(parse(from_os_str))] // from_os_str形式に変換する
    file: std::path::PathBuf,
}

fn main() {
    // step1
    // let args = Cli::parse();
    // println!("pattern:{}", args.pattern);
    // println!("text:{}", args.text);

    // step2
    // let args = Cli::parse();
    // println!("pattern:{}", args.pattern);
    // println!("file:{}", args.file.display());

    // step3
    // let args = Cli::parse();
    // let result = std::fs::read_to_string(&args.file);
    // match result {
    //     Ok(Context) => {
    //         println!("fileData:{}", Context);
    //     }
    //     Err(error) => {
    //         println!("Oh nose:{}", error);
    //     }
    // }

    // step4
    let args = Cli::parse();
    let result = std::fs::read_to_string(&args.file);
    let content = match result {
        Ok(content) => content,
        Err(error) => {
            panic!("Oh nose:{}", error);
        }
    };

    // step5
    let mut is_existe_ditor = false;
    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
            is_existe_ditor = true;
        }
    }
    if !&is_existe_ditor {
        println!("select Editor name. like Vim or Vim or Vim or Vim");
    }
}
