use std::{env, fs, path::Path};

fn main() {
    let current_dir = env::current_dir().unwrap();
    let proto_path = Path::new(&current_dir).join("proto");
    let mut proto_files = vec![];
    for entry in fs::read_dir(&proto_path).unwrap() {
        let entry = entry.unwrap();
        let md = entry.metadata().unwrap();
        if md.is_file() && entry.path().extension().unwrap() == "proto" {
            proto_files.push(entry.path().as_os_str().to_os_string())
        }
    }

    tonic_build::configure()
        .out_dir("src") // 生成代码的存放目录
        .build_client(true)
        .build_server(true)
        .compile(
            proto_files.as_slice(), // 欲生成的 proto 文件列表
            &[&proto_path],         // proto 依赖所在的根目录
        )
        .unwrap();
}
