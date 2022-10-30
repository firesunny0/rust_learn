use std::fs;

const ROOT_PATH: &str = "E:/Study/WorkSpace/Dataset/广医1003";

fn main() {
    // let path = OsString::from("e:/Study/WorkSpace");
    let mut src_path = String::from(ROOT_PATH);
    let mut dst_path = String::from(ROOT_PATH);
    let mut wav_file_cnt = 0;
    src_path += &String::from("/卒中患者视频");
    dst_path += &String::from("/卒中_merge");
    show_files_in_dir(&src_path, &dst_path, &mut wav_file_cnt);
    println!("{:#?}", src_path);
    println!("{}", wav_file_cnt);
}

fn show_files_in_dir(dir: &String, dst_dir: &String, file_cnt: &mut i32) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                // Here, `entry` is a `DirEntry`.
                println!("Process {:?} :", entry.file_name());
                let file_name = entry.file_name();
                let ref_file_name = file_name.to_str().unwrap_or_else(|| {
                    println!(
                        "Transform {:?} from OsString to String failed. ",
                        entry.file_name()
                    );
                    ""
                });
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_dir() {
                        show_files_in_dir(
                            &(String::from(dir) + "/" + ref_file_name),
                            dst_dir,
                            file_cnt,
                        );
                    } else {
                        if ref_file_name.contains("mp4")
                            && (ref_file_name.contains("言语")
                                || ref_file_name.contains("语言")
                                || ref_file_name.contains("中华人民"))
                        {
                            *file_cnt = *file_cnt + 1;
                            println!(
                                "{}: find a file : {}",
                                *file_cnt,
                                String::from(dir) + "/" + ref_file_name
                            );
                            let len = fs::copy(
                                &(String::from(dir) + "/" + ref_file_name),
                                &(dst_dir.to_string() + "/" + &(*file_cnt).to_string() + ".mp4"),
                            )
                            .unwrap();
                        }
                    }
                }
            }
        }
    }
}
