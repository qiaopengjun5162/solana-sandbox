# Rust 条件编译示例

这个项目展示了 Rust 中条件编译的各种用法。

## 条件编译基础

条件编译使用 `#[cfg(...)]` 属性，允许根据编译时的条件来包含或排除代码。

## 运行示例

### 1. 默认编译（包含 debug 特性）

```bash
cargo run
```

### 2. 启用所有特性

```bash
cargo run --features "debug,experimental,advanced,logging,verbose"
```

### 3. 禁用所有特性

```bash
cargo run --no-default-features
```

### 4. 只启用特定特性

```bash
cargo run --features "logging,verbose"
```

## 条件编译类型

### 1. 基于目标平台

```rust
#[cfg(target_os = "windows")]
fn windows_function() { }

#[cfg(target_os = "linux")]
fn linux_function() { }

#[cfg(target_os = "macos")]
fn macos_function() { }
```

### 2. 基于架构

```rust
#[cfg(target_arch = "x86_64")]
fn x86_function() { }

#[cfg(target_arch = "aarch64")]
fn arm_function() { }
```

### 3. 基于特性标志

```rust
#[cfg(feature = "debug")]
fn debug_function() { }

#[cfg(not(feature = "debug"))]
fn release_function() { }
```

### 4. 组合条件

```rust
// 所有条件都满足
#[cfg(all(target_os = "windows", feature = "debug"))]
fn windows_debug() { }

// 任一条件满足
#[cfg(any(target_os = "linux", target_os = "macos"))]
fn unix_function() { }
```

## 常用条件编译选项

### 目标平台

- `target_os = "windows"`
- `target_os = "linux"`
- `target_os = "macos"`
- `target_os = "android"`
- `target_os = "ios"`

### 目标架构

- `target_arch = "x86_64"`
- `target_arch = "aarch64"`
- `target_arch = "x86"`
- `target_arch = "arm"`

### 编译模式

- `debug_assertions` - 调试断言是否启用
- `test` - 是否在测试模式下编译

### 自定义特性

在 `Cargo.toml` 中定义：

```toml
[features]
debug = []
experimental = []
advanced = []
```

## 使用场景

1. **跨平台开发** - 为不同操作系统提供不同的实现
2. **性能优化** - 在发布版本中移除调试代码
3. **功能开关** - 通过特性标志控制功能启用
4. **架构特定优化** - 为不同 CPU 架构提供优化代码
5. **测试和调试** - 在测试环境中启用额外功能

## 编译命令示例

```bash
# 发布版本（禁用调试）
cargo build --release --no-default-features

# 启用所有特性
cargo build --features "debug,experimental,advanced,logging,verbose"

# 交叉编译到不同平台
cargo build --target x86_64-unknown-linux-gnu
cargo build --target aarch64-apple-darwin
```
