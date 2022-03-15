use color_eyre::Result;
use serde::Serialize;
use dialoguer::{
    theme::ColorfulTheme,
    Input
};
use std::fs;

#[derive(Serialize)]
struct Package {
    name: String,
    version: String,
    description: String,
    main: String,
    author: String,
    license: String,
    keywords: Vec<String>
}

fn main() -> Result<()> {
    color_eyre::install()?;

    println!("Scaffold package.json! (smolovk)");

    let theme = ColorfulTheme::default();
    let cwd = std::env::current_dir().unwrap();
    let dir_str = String::from(cwd.to_string_lossy());
    let dir_arr: Vec<String> = dir_str.split("/").map(str::to_string).collect();
    let current_dir = String::from(dir_arr.last().unwrap());

    let name: String = Input::with_theme(&theme)
        .with_prompt("name")
        .default(current_dir)
        .interact_text()?;
    
    let version: String = Input::with_theme(&theme)
        .with_prompt("version")
        .default(String::from("1.0.0"))
        .interact_text()?;

    let description: String = Input::with_theme(&theme)
        .with_prompt("description")
        .allow_empty(true)
        .interact_text()?;
    
    let main: String = Input::with_theme(&theme)
        .with_prompt("entry point")
        .default(String::from("index.js"))
        .interact_text()?;
    
    let author: String = Input::with_theme(&theme)
        .with_prompt("author")
        .allow_empty(true)
        .interact_text()?;

    let license: String = Input::with_theme(&theme)
        .with_prompt("license")
        .default(String::from("ISC"))
        .interact_text()?;

    let keywords_str: String = Input::with_theme(&theme)
        .with_prompt("keywords")
        .allow_empty(true)
        .interact_text()?;

    let keywords: Vec<String>;
        
    if keywords_str != String::from("") {
        keywords = keywords_str.split(", ").map(str::to_string).collect();
    } else {
        keywords = Vec::new();
    }

    let package = Package {
        name: name,
        version: version,
        description: description,
        main: main,
        author: author,
        license: license,
        keywords: keywords
    };

    let json = serde_json::to_string_pretty(&package)?;
    
    println!("{}", json);
    
    fs::write("package.json", json)?;
    
    Ok(())
}
