mod docflux_core;

use docflux_core::{ensure_vault_directories, resolve_docflux_data_dir};

/// 提供给前端的最小通信测试接口。
///
/// 这里不接收额外参数，避免第一阶段把前后端通信样例设计得过重。
/// 返回值中附带当前解析出的数据目录，便于开发阶段快速确认目录决策是否正确。
#[tauri::command]
fn greet_docflux() -> String {
    let data_dir = resolve_docflux_data_dir();
    format!(
        "DocFlux 核心引擎已启动，数据目录：{}",
        data_dir.display()
    )
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app_data_dir = ensure_vault_directories()
        .unwrap_or_else(|error| panic!("DocFlux 初始化失败：{error}"));

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(move |_app| {
            // 启动阶段再次保留一份可读日志，便于开发者在调试台确认实际落盘位置。
            println!("DocFlux 数据目录已就绪：{}", app_data_dir.display());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet_docflux])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
