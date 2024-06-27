# Rust 编写的 Java 虚拟机

这是一个用 Rust 编写的简单 Java 虚拟机。它允许你加载和运行 Java 字节码文件，并模拟了 Java 虚拟机的一些基本功能。

## 功能

- 加载和解释 Java 字节码文件（.class 文件）。
- 支持常见的 Java 字节码指令。
- 基本的类加载和方法解析。

## 快速入门

1. **安装 Rust**: 如果你还没有安装 Rust，你需要首先安装它。你可以在 [Rust 官方网站](https://www.rust-lang.org/tools/install) 上找到安装指南。

2. **克隆项目**: 使用 Git 克隆这个项目到本地：

   ```sh
   git clone https://github.com/kgagag/tomato.git
   cd java-vm-rust
   ```

3. **构建项目**: 在项目根目录运行以下命令以构建项目：

   ```sh
   cargo build
   ```

   将 test 目录改造为 maven 项目，通过 `mvn compile` 编译项目，并修改 `class_loader.rs` 加载类路径

4. **运行虚拟机**: 你可以使用以下命令运行 Java 虚拟机并指定要执行的 Java 类文件：

   ```sh
   cargo run
   ```

## 许可

这个项目根据 MIT 许可证授权。有关详细信息，请查看 [LICENSE](LICENSE) 文件。

## 联系我们

如果您有任何疑问或反馈，可以通过以下方式联系我们：[你的联系信息或链接]

## 致谢

我们要感谢所有为这个项目做出贡献的人和开源社区的支持。

## 常见问题

你可以在我们的 [FAQ](FAQ.md) 中找到一些常见问题和答案。

## Roadmap

我们计划在未来添加以下功能：

- 支持更多 Java 字节码指令。
- 增加性能优化。
- 添加对 Java 类库的更多支持。

请查看我们的 [Roadmap](ROADMAP.md) 以获取更多详细信息。

## 版本历史

0.0.1
