use std::fs;

fn main() {
	read_all_files("/Users/stuart/rust/vfpkb");
}

fn read_all_files(dir_path :&str) {
	
	for entry in fs::read_dir(dir_path).expect("Unable to list") {
        let entry = entry.expect("unable to get entry");
		let path = entry.path();

		if path.is_dir() {
             read_all_files(path.as_os_str().to_str().unwrap());
        } 
		else {
             println!("{:?}", path);
        }
    }
}
