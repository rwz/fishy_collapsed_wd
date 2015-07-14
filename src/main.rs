use std::env;
use std::path::Path;

fn main(){
    let pwd = env::current_dir().unwrap();
    let pwd = pwd.to_str().unwrap();
    let home = env::home_dir().unwrap();
    let home = home.to_str().unwrap();
    let pwd = pwd.replace(home, "~");
    let pwd : &str = &pwd[..];
    let path = Path::new(pwd);

    let parent = path.parent().unwrap();

    for component in parent.iter() {
        let component = component.to_str().unwrap();
        let component_char = component.chars().nth(0).unwrap();
        print!("{}/", component_char);
    }

    println!("{}", path.file_name().unwrap().to_str().unwrap());
}
