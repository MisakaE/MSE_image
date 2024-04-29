use std::{
    env,
    error::Error,
    fs,
    path::{self, Path},
};

pub fn read_list<T: AsRef<Path>>(path: T) -> Result<Vec<String>, Box<dyn Error>> {
    let mut list = vec![];
    if path.as_ref().is_file(){
        list.push(path.as_ref().display().to_string());
        return Ok(list)
    }
    let path_list: Vec<_> = fs::read_dir(path)?
        .map(|f| {
            let dir = f.unwrap();
            dir.path()
        })
        .collect();
    
    for i in path_list {
        match i.extension() {
            Some(t) => match t.to_str().unwrap() {
                "PNG" | "png" | "jpg" | "JPG" | "jpeg" | "JPEG" | "svg" | "SVG" | "WEBP"
                | "webp" | "ico" | "ICO" => list.push(i.to_str().unwrap().to_string()),
                _ => (),
            },
            None => (),
        }
    }
    Ok(list)
}
pub fn parse() -> Result<String, Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        Some(a) => {
            let path = path::Path::new(a);
            if path.is_dir() || path.is_file(){
                return Ok(a.to_string());
            } else {
                return Err(format!("\"{}\" is not a dir",a).into())
            }
        }
        None => return Err("No enough args".into()),
    }
}
