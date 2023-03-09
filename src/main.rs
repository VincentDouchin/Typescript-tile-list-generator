use std::collections::HashMap;
use std::env::args;
use std::env::{self, current_exe};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
fn main() {
    fn read_dir(dir: &Path, mut files: &mut Folder) {
        let arguments: Vec<String> = args().collect();
        let type_of_file = &arguments[1];
        let paths = fs::read_dir(dir).unwrap();
        let mut folder: Vec<String> = Vec::new();
        for path in paths {
            let p = path.unwrap();
            let is_path = p.metadata().unwrap().is_dir();
            if !is_path {
                match p.path().extension() {
                    None => (),
                    Some(extension) => {
                        if extension.to_str().unwrap() == type_of_file {
                            let file_name: String =
                                p.path().file_stem().unwrap().to_str().unwrap().to_string();

                            folder.push(file_name);
                        }
                    }
                }
            }
            if is_path {
                read_dir(&p.path(), &mut files)
            }
        }
        if folder.len() != 0 {
            let path_name = dir.parent().unwrap().to_str().unwrap().to_string();
            let dir_name = path_name.split("\\").last().unwrap();
            files.insert(dir_name.to_string(), folder);
        }
    }
    let current_dir = env::current_dir().unwrap();
    type Folder = HashMap<String, Vec<String>>;
    let mut files: Folder = HashMap::new();
    read_dir(&current_dir, &mut files);
    let mut type_string: String = String::new();
    for (folder, files) in files {
        type_string.push_str("type ");
        type_string.push_str(&folder.replace(".", "_").replace(" ", "_"));
        type_string.push_str(" = ");
        for (index, file) in files.iter().enumerate() {
            type_string.push_str("\"");
            type_string.push_str(&file);
            type_string.push_str("\"");
            if index != files.len() - 1 {
                type_string.push_str(" | ");
            }
        }
        type_string.push_str("\n")
    }

    let mut output_path = current_exe().unwrap();
    output_path.pop();
    let output_dir_string = output_path.to_str().unwrap().to_string();
    let mut output_file_name = output_dir_string.split("\\").last().unwrap().to_string();

    output_file_name.push_str(".d.ts");

    let final_output_path = output_path.join(output_file_name);
    let mut output = File::create(final_output_path).expect("can't create file");
    write!(output, "{}", type_string).expect("can't write to file");
}
