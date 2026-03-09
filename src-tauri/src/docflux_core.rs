use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;

/// 解析 DocFlux 的集中式数据根目录。
///
/// Windows 下遵循指令：
/// 1. 若检测到存在 D 盘，则默认放置到 `D:/DocFluxData/`，避免与系统盘用户目录混放。
/// 2. 若不存在 D 盘，则回退到 `%APPDATA%/DocFluxData/`。
///
/// 其他平台暂时采用各自常见的用户级应用数据目录：
/// - macOS: `~/Library/Application Support/DocFluxData`
/// - Linux/其他 Unix: 优先 `XDG_DATA_HOME/DocFluxData`，否则 `~/.local/share/DocFluxData`
///
/// 之所以不把仓库散落在文档旁边，是因为本项目采用集中式 Vault 架构，
/// 这样便于统一管理 SQLite 数据库与各文档 Git 仓库，也更利于后续迁移和备份。
pub fn resolve_docflux_data_dir() -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        let preferred_drive = PathBuf::from("D:/");
        if preferred_drive.exists() {
            return preferred_drive.join("DocFluxData");
        }

        let app_data = env::var_os("APPDATA")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("C:/Users/Public/AppData/Roaming"));

        return app_data.join("DocFluxData");
    }

    #[cfg(target_os = "macos")]
    {
        let home = env::var_os("HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("."));
        return home
            .join("Library")
            .join("Application Support")
            .join("DocFluxData");
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    {
        if let Some(xdg_data_home) = env::var_os("XDG_DATA_HOME") {
            return PathBuf::from(xdg_data_home).join("DocFluxData");
        }

        let home = env::var_os("HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("."));
        home.join(".local").join("share").join("DocFluxData")
    }
}

/// 启动时确保 Vault 根目录与 `repos` 子目录存在。
///
/// 这里使用 `create_dir_all`，原因是它具备幂等性：
/// - 目录已存在时不会报错；
/// - 中间层级缺失时会自动补齐。
/// 这非常适合应用启动阶段的环境修复型初始化。
pub fn ensure_vault_directories() -> io::Result<PathBuf> {
    let data_dir = resolve_docflux_data_dir();
    let repos_dir = data_dir.join("repos");

    fs::create_dir_all(&repos_dir)?;

    Ok(data_dir)
}