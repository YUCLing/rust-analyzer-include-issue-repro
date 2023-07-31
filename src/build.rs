use std::{env, path::Path, fs};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    
    let mod_path = Path::new(&out_dir).join("test_mod.rs");
    fs::write(&mod_path, 
"pub mod test_mod {
    mod test_mod_inner;
    pub use self::test_mod_inner::test;
}"
    ).unwrap();

    let dir_path = Path::new(&out_dir).join("test_mod");
    if !dir_path.exists() {
        fs::create_dir(dir_path).unwrap();
    }

    let mod_inner_path = Path::new(&out_dir).join("test_mod/test_mod_inner.rs");
    fs::write(&mod_inner_path, 
"pub fn test() {
    println!(\"test\");
}"
    ).unwrap();
}