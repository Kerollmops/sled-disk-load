use std::env;
use std::path::PathBuf;

fn main() {
    let mut args = env::args();

    let _program_name = args.next();
    let path = args.next().map(PathBuf::from).unwrap();

    let db = sled::Db::open(path).unwrap();
    let size = 1024*5; // 5 KB

    for i in 0..600_000 {

        if i % 1000 == 0 {
            println!("turn {}", i);
            db.flush().unwrap();
        }

        for key in 0..3000 {
            let key = key.to_string();
            db.insert(key, vec![1; size]).unwrap();
        }
    }
}
