/// 校验参数
///
/// Check parameters
pub fn check_param() -> Vec<String> {
    let outer_param: Vec<String> = std::env::args().collect();

    // 必须传递源文件和文件夹参数
    // Source file and folder parameters must be passed
    if outer_param.len() != 3 {
        panic!("Source file and folder parameters must be passed")
    }

    // 判断源文件是否存在
    // Determine whether the source file exists
    if let Ok(file) = std::fs::metadata(&outer_param[1]) {
        if !file.is_file() {
            panic!("source file is not file")
        }
    } else {
        panic!("source file path is not exist")
    }

    // 生成路径不存在
    // dst path is not exist
    if !std::path::Path::exists(outer_param[2].as_ref()) {
        // 创建文件夹
        // create folder
        std::fs::create_dir(&outer_param[2])
            .unwrap_or_else(|_| panic!("{} is not path", &outer_param[2]));
    } else {
        // 清空生成路径
        // clear and delete dst path
        std::fs::remove_dir_all(&outer_param[2]).expect("remove dst dir wrong");
        std::fs::create_dir(&outer_param[2])
            .unwrap_or_else(|_| panic!("{} is not path", &outer_param[2]));
    }

    outer_param
}
