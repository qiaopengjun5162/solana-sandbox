// 条件编译示例

// 1. 基于目标平台的条件编译
#[cfg(target_os = "windows")]
fn get_platform_info() -> &'static str {
    "Windows 平台"
}

#[cfg(target_os = "linux")]
fn get_platform_info() -> &'static str {
    "Linux 平台"
}

#[cfg(target_os = "macos")]
fn get_platform_info() -> &'static str {
    "macOS 平台"
}

// 2. 基于架构的条件编译
#[cfg(target_arch = "x86_64")]
fn get_architecture() -> &'static str {
    "x86_64 架构"
}

#[cfg(target_arch = "aarch64")]
fn get_architecture() -> &'static str {
    "ARM64 架构"
}

// 3. 基于特性标志的条件编译
#[cfg(feature = "debug")]
fn debug_function() {
    println!("调试功能已启用");
}

#[cfg(not(feature = "debug"))]
fn debug_function() {
    println!("调试功能已禁用");
}

// 4. 组合条件编译
#[cfg(all(target_os = "windows", feature = "debug"))]
fn windows_debug_function() {
    println!("Windows 平台的调试功能");
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
fn unix_function() {
    println!("Unix 系统功能");
}

// 5. 自定义特性标志
#[cfg(feature = "experimental")]
fn experimental_feature() {
    println!("实验性功能");
}

// 6. 条件编译的模块
#[cfg(feature = "advanced")]
mod advanced {
    pub fn advanced_function() {
        println!("高级功能模块");
    }
}

// 7. 条件编译的结构体
#[cfg(feature = "logging")]
struct Logger {
    level: String,
}

#[cfg(feature = "logging")]
impl Logger {
    fn new(level: &str) -> Self {
        Logger {
            level: level.to_string(),
        }
    }

    fn log(&self, message: &str) {
        println!("[{}] {}", self.level, message);
    }
}

fn main() {
    println!("=== Rust 条件编译示例 ===");

    // 显示平台信息
    println!("平台: {}", get_platform_info());
    println!("架构: {}", get_architecture());

    // 调试功能
    debug_function();

    // Unix 系统功能
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    unix_function();

    // Windows 调试功能
    #[cfg(all(target_os = "windows", feature = "debug"))]
    windows_debug_function();

    // 实验性功能
    #[cfg(feature = "experimental")]
    experimental_feature();

    // 高级功能模块
    #[cfg(feature = "advanced")]
    {
        advanced::advanced_function();
    }

    // 日志功能
    #[cfg(feature = "logging")]
    {
        let logger = Logger::new("INFO");
        logger.log("这是一条日志消息");
    }

    println!("\n=== 编译时信息 ===");
    println!("目标操作系统: {}", std::env::consts::OS);
    println!("目标架构: {}", std::env::consts::ARCH);

    // 8. 运行时检查特性标志
    #[cfg(feature = "verbose")]
    println!("详细模式已启用");

    #[cfg(not(feature = "verbose"))]
    println!("详细模式已禁用");
}
