use std::{env, path::Path};

use glob::glob;

use crate::structs::err::Err;

pub fn list(dir_path:&str) -> Result<Vec<String>,Err>{
    let mut list:Vec<String> = Vec::new();
    let pattern = format!("{}/*.*",dir_path);
    for i in match glob(&pattern){
        Ok(ok) => ok,
        Err(_) => return Err(Err::Errs),
    }{
        match i {
            Ok(path) =>{
                let path_string = path.to_str().unwrap();
                let re = regex::Regex::new(r"[\.][A-Z]*[a-z]*$").unwrap();
                let j = re.find(path_string).unwrap();
                match j.as_str() {
                    ".jpg"|".JPG"|".png"|".PNG"|".webp"|".WEBP" =>{
                        list.push(path.display().to_string());
                    }
                    _ => ()
                }
            }
            Err(_)=> return Err(Err::Errs),
        }
    }
    Ok(list)
}

pub fn parameters() -> Result<(bool,String),Err>{
    let para:Vec<String> = env::args().collect();
    if para.len()<=1{
        return Err(Err::Errs);
    }
    if para.len() == 2{
        if Path::new(para[1].as_str()).is_file(){
            return Ok((false,para[1].clone()));
        }
        else {
            return Err(Err::Errs);
        }
    }
    if para.len() == 3{
        if para[1] == "-d"{
            if Path::new(para[2].as_str()).is_dir(){
                return Ok((true,para[2].clone()));
            }
        }
    }
    Err(Err::Errs)

}