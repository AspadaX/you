# You - Your Optimized UNIX (and Windows too)

English | [中文](./README_CN.md)

`you` is a command-line tool that translates natural language instructions into executable shell commands, making command-line operations more accessible and intuitive. It's designed especially for newcomers to command-line interfaces, but also helps experienced users by reducing cognitive load and documentation searches.

## Core Features

- **Talk in Plain Language**: Just tell the tool what you want to do in regular words, and it turns that into commands the computer understands
- **Chat Mode**: Have a back-and-forth conversation where you can ask for multiple things while the tool remembers what you talked about before
- **Command Review**: The tool handles the hard technical stuff but lets you decide when to run commands
- **Save for Later**: Keep useful commands to use again without having to ask the tool each time

## Installation

### Setup Script (Linux & macOS)

Download and run the setup script:

```bash
curl -O https://raw.githubusercontent.com/AspadaX/you/main/setup.sh && chmod +x ./setup.sh && ./setup.sh && rm ./setup.sh
```

To update:

```bash
curl -O https://raw.githubusercontent.com/AspadaX/you/main/update.sh && chmod +x ./update.sh && ./update.sh && rm ./update.sh
```

To uninstall:

```bash
curl -O https://raw.githubusercontent.com/AspadaX/you/main/uninstall.sh && chmod +x ./uninstall.sh && ./uninstall.sh && rm ./uninstall.sh
```

### Using Cargo

If you have Rust installed:

```bash
cargo install you
```

## Usage

### About Setting Up the LLM

By default, the environment variables in your `~/.zshrc` (for zsh, primarily macOS), or `~/.bashrc` (for bash, most Linux systems), should look like this:
```bash
export YOU_OPENAI_API_BASE="https://api.openai.com/v1"
export YOU_OPENAI_API_KEY="sk-yourapikey"
export YOU_OPENAI_MODEL="gpt-4.1" # model you want to use
```
Notice that for all OpenAI-compatible API bases, it needs to end with a `/v1`. If adding `v1` does not work, then you might want to remove it. 

For Ollama users, you will also need the `/v1` at the end of the url. For example, if your endpoint is `http://localhost:11434`, then you probably need to put `http://localhost:11434/v1`. 

### Basic Command Execution

Run a command described in natural language:

```bash
you run "find the largest file in my downloads directory"
```

### Command Explanation

Get an explanation of what a command does:

```bash
you explain "find . -type f -name '*.txt' -size +10M"
```

### Interactive Mode

Start a conversational session to run multiple related commands:

```bash
you run
```

### Cache Management

List all cached scripts:

```bash
you list
# or use the alias
you ls
```

Remove a specific cached script:

```bash
you remove <script_name>
# or use the alias
you rm <script_name>
```

### Configure your preferred CLI

You may want to use `fd` over `find`, or prefer using a different CLI rather than letting the LLM guess. In this case, you may update the configuration file located at `~/.you/configurations.json`. Below is an example:

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

Now, `you` will use `fd` over `find` when you issue commands relevant to searching files. 

### Enable Cache

If you would like to enable cache feature, you may also want to enable it in the `~/.you/configurations.json`:

```json
{
  "enable_cache": true,
  "preferred_clis": [
    {
      "name": "fd",
      "preferred_for": "search files. and replace find"
    }
  ]
}
```

## Other Examples

```bash
# Find files you've modified in the last week
you run "show me files I've modified in the last 7 days"

# Get system information
you run "how many CPU cores and how much RAM does this system have?"

# Complex tasks made simple
you run "compress all JPG images in the current directory and save them to a new folder"

# Remote operations
you run "connect to my server at 192.168.*.* and check disk space"
```

## LLM Support

`you` works with various LLMs:

- Works well with small models like `smollm2`
- Compatible with OpenAI compatible APIs, such as DeepSeek.
- Compatible with `ollama` for using any open-source model for free
- Configure your preferred model for the best balance of performance and accuracy

## Workflow

1. Type your request in natural language
2. Review the suggested command(s) and explanation
3. Type 'y' to execute or provide additional guidance
4. If there's an error, the AI automatically suggests a corrected command
5. Optionally save useful command sequences for future reuse

## Why Use You?

- **Reduce Documentation Searches**: Get the right command without extensive searching
- **Learning Tool**: See how natural language translates to actual commands
- **Productivity Boost**: Accomplish complex tasks with simple instructions
- **Safe Command Execution**: Review commands before execution
- **Error Recovery**: Get help when commands fail
- **Context Retention**: In interactive mode, the AI remembers previous commands

## License

MIT

## Credits

Created by Xinyu Bao

## Acknowledgments

This project would not be possible without these amazing libraries:

- **anyhow**: Error handling made simple and flexible
- **async-openai**: API client for interacting with OpenAI's language models
- **cchain**: Command chaining functionality for shell operations
- **chrono**: Date and time handling with precision
- **clap**: Command-line argument parsing with a beautiful interface
- **console**: Terminal text styling and utilities
- **indicatif**: Progress indicators for command-line applications
- **serde/serde_json**: Powerful serialization and deserialization framework
- **sysinfo**: System information gathering across platforms
- **tokio**: Asynchronous runtime for efficient operations
- **surfing**: Parse JSON out of plain texts

A big thank you to all the developers who maintain these open-source libraries!

---

_`you` - because command lines should understand you, not the other way around._
