# You - 您优化的 UNIX（Windows 也支持）

[English](./README.md) | 中文

`you` 是一个命令行工具，可将自然语言指令转换为可执行的 shell 命令，使命令行操作更加便捷直观。它专为命令行界面的新手设计，同时也帮助经验丰富的用户减少认知负担和文档搜索。

## 核心功能

- **自然语言交流**：只需用普通话语告诉工具您想做什么，它就会将其转换为计算机理解的命令
- **聊天模式**：进行来回对话，您可以请求多个操作，同时工具会记住之前的对话内容
- **命令审查**：工具处理复杂的技术细节，但让您决定何时运行命令
- **保存备用**：保存有用的命令以供再次使用，无需每次都向工具询问

## 安装

### 安装脚本（Linux 和 macOS）

下载并运行安装脚本：

```bash
curl -O https://raw.githubusercontent.com/AspadaX/you/main/setup.sh && chmod +x ./setup.sh && ./setup.sh && rm ./setup.sh
```

更新：

```bash
curl -O https://raw.githubusercontent.com/AspadaX/you/main/update.sh && chmod +x ./update.sh && ./update.sh && rm ./update.sh
```

卸载：

```bash
curl -O https://raw.githubusercontent.com/AspadaX/you/main/uninstall.sh && chmod +x ./uninstall.sh && ./uninstall.sh && rm ./uninstall.sh
```

### 使用 Cargo 安装

如果已安装 Rust：

```bash
cargo install you
```

## 使用方法

### 基本命令执行

运行用自然语言描述的命令：

```bash
you run "在我的下载目录中找到最大的文件"
```

### 命令解释

获取命令作用的解释：

```bash
you explain "find . -type f -name '*.txt' -size +10M"
```

### 交互模式

启动对话会话以运行多个相关命令：

```bash
you run
```

### 配置您偏好的命令行工具

您可能想使用 `fd` 而不是 `find`，或者偏好使用特定的命令行工具而不是让 LLM 猜测。在这种情况下，您可以更新位于 `~/.you/configurations.json` 的配置文件。以下是一个示例：

```json
{
  "preferred_clis": [
    {
      "name": "fd",
      "preferred_for": "search files. and replace find"
    }
  ]
}
```

现在，当您发出与搜索文件相关的命令时，`you` 将使用 `fd` 而不是 `find`。

## 其他示例

```bash
# 查找您在过去一周修改的文件
you run "显示我在过去7天内修改的文件"

# 获取系统信息
you run "这个系统有多少CPU核心和多少内存？"

# 简化复杂任务
you run "压缩当前目录中的所有JPG图像并保存到新文件夹"

# 远程操作
you run "连接到我的服务器 192.168.*.* 并检查磁盘空间"
```

## LLM 支持

`you` 可与各种 LLM 配合使用：

- 与小型模型如 `smollm2` 配合良好
- 兼容 OpenAI 兼容的 API，如 DeepSeek
- 兼容 `ollama`，可免费使用任何开源模型
- 配置您偏好的模型以获得最佳性能和准确性平衡

## 工作流程

1. 用自然语言输入您的请求
2. 查看建议的命令和解释
3. 输入 'y' 执行或提供额外指导
4. 如果出错，AI 会自动建议修正的命令
5. 可选择保存有用的命令序列以供将来重用

## 为什么使用 You？

- **减少文档搜索**：无需大量搜索即可获得正确命令
- **学习工具**：了解自然语言如何转换为实际命令
- **提高生产力**：通过简单指令完成复杂任务
- **安全命令执行**：执行前先审查命令
- **错误恢复**：命令失败时获得帮助
- **上下文保留**：在交互模式中，AI 会记住之前的命令

## 许可证

MIT

## 致谢

由 Xinyu Bao 创建

## 鸣谢

本项目的实现离不开以下优秀库的支持：

- **anyhow**：简单灵活的错误处理
- **async-openai**：用于与 OpenAI 语言模型交互的 API 客户端
- **cchain**：用于 shell 操作的命令链功能
- **chrono**：精确的日期和时间处理
- **clap**：具有美观界面的命令行参数解析
- **console**：终端文本样式和实用工具
- **indicatif**：命令行应用程序的进度指示器
- **serde/serde_json**：强大的序列化和反序列化框架
- **sysinfo**：跨平台的系统信息收集
- **tokio**：高效操作的异步运行时
- **surfing**：从纯文本中解析 JSON

衷心感谢所有维护这些开源库的开发者！

---

_`you` - 因为命令行应该理解您，而不是相反。_