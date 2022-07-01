use std::env;
use std::fs::File;
use std::io::Write;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    if args.len() != 3 {
        println!("Usage: {} <image.jpg> <result.txt>", args[0]);
        return;
    }

    let filename = &args[1];
    let result_filename = &args[2];

    match rexif::parse_file(&filename) {
        Ok(exif) => {
            let mut file = File::create(&result_filename).expect("Nao foi possivel criar o arquivo de saida.");
            for entry in exif.entries {
                if entry.tag.to_string().starts_with("GPS") {
                    let entry = format!("{}: {}\r\n", entry.tag, entry.value_more_readable);
                    file.write_all(entry.as_bytes())
                        .expect("Nao foi possivel escrever no arquivo de saida.");
                }
            }
        }
        Err(e) => {
            println!("Error in {}: {}", filename, e);
        }
    }
}
