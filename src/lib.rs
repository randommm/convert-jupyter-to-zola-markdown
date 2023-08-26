use regex::Regex;
use std::fs;
use std::path::Path;
use std::process::Command;
use walkdir::WalkDir;

const MD_HEADER: &str = r#"+++
title = "$1"
description = "First steps with Rust programming"
draft = false
weight = 30
sort_by = "weight"
template = "docs/page.html"

[extra]
toc = true
top = false
+++
"#;

const MD_FOOTER: &str = r#"
<a id="download_notebook_anchor" download>Download this page as a Jupyter Notebook</a>
<script>
var url = window.location.href;
if (url.slice(-1) == "/") {
    url = url.slice(0, -1);
}
url = url + ".ipynb";
document.getElementById("download_notebook_anchor").href = url;
</script>
"#;

fn search_ipynb_files(directory: &str) {
    for entry in WalkDir::new(directory).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() && path.extension().map(|ext| ext == "ipynb").unwrap_or(false) {
            convert_ipynb_to_md(path);
            post_process_md_file(path);
        }
    }
}

fn post_process_md_file(path: &Path) {
    let path = path.to_str().unwrap();
    let path = &path[..path.len() - 6];
    let path = format!("{}.md", path);
    let path = path.as_str();
    println!("Post processing {:?}", path);

    let contents = fs::read_to_string(path).unwrap();
    let re = Regex::new(r"#+ *?([^ ]+?) *?\n").unwrap();
    let mut contents = re.replace(&contents, MD_HEADER).to_string();
    contents.push_str(MD_FOOTER);

    fs::write(path, contents).expect("Unable to write file");
}

fn convert_ipynb_to_md(path: &Path) {
    println!("Converting {:?}", path);
    let cmd = format!("jupyter nbconvert --to markdown {}", path.to_str().unwrap());
    println!("{}", cmd);
    let output = Command::new("sh").arg("-c").arg(cmd).output().unwrap();

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("Output: {}", stdout);
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("Error: {}", stderr);
    }
}

pub fn run() {
    search_ipynb_files("./content")
}
