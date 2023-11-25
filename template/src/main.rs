use std::fs;
use std::io::{Read, Write};
use std::path::Path;

fn main() -> std::io::Result<()> {
    let day = 1;
    let template_path = "data/day_0x";

    let out_dir = format!("out/day_{:02}", day);
    let src_dir = "src";
    let test_dir = "tests";

    fs::create_dir_all(&format!("{out_dir}/{src_dir}")).expect("Unable to create directories");
    fs::create_dir_all(&format!("{out_dir}/{test_dir}")).expect("Unable to create directories");

    create_file(&out_dir, template_path, "src/main.rs", None);

    let day_replace_fn =
        move |text: &String| -> String { text.replace("day_0x", &format!("day_{:02}", day)) };
    create_file(
        &out_dir,
        template_path,
        "tests/test_example.rs",
        Some(&day_replace_fn),
    );

    create_file(&out_dir, template_path, "example.txt", None);
    create_file(&out_dir, template_path, "input.txt", None);
    create_file(&out_dir, template_path, "Cargo.toml", Some(&day_replace_fn));

    let readme_day_replace_fn =
        move |text: &String| -> String { text.replace("Day X", &format!("Day {}", day)) };
    create_file(
        &out_dir,
        template_path,
        "readme.md",
        Some(&readme_day_replace_fn),
    );

    Ok(())
}

type TextReplaceFn = dyn Fn(&String) -> String;

fn create_file(
    out_dir: &str,
    template_dir: &str,
    file_name: &str,
    replace_fn: Option<&TextReplaceFn>,
) {
    let out_path = Path::new(&out_dir);

    let mut main_template = String::new();
    fs::File::open(&format!("{template_dir}/{file_name}"))
        .expect("Unable to open file")
        .read_to_string(&mut main_template)
        .expect("Unable to read file");

    let main_path = out_path.join(file_name);

    let processed = if replace_fn.is_some() {
        replace_fn.unwrap()(&main_template)
    } else {
        main_template
    };

    let mut output_file = fs::File::create(main_path).expect("Unable to create file");
    write!(output_file, "{}", processed).expect("Unable to write file");
}
