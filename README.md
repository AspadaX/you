# You - Your Optimized UNIX, but also supports Windows

`you` is a command-line tool that translates natural language instructions into executable shell commands, making command-line operations more accessible and intuitive, especially for beginners in programming and command lines. 

## Features

- **Natural Language Command Execution**: Type what you want to do in plain English, or any other natural language you prefer, and let the AI figure out the exact commands
- **Interactive Workflow**: Review suggested commands before execution
- **Error Handling**: When commands fail, the AI automatically suggests fixes based on error messages
- **Semi-Autonomous Operation**: The AI handles the technical details while keeping you in control

## Installation

### Setup Script
You may download [this](./setup.sh) shell script to install `you`. This is supported on both Linux and macOS platforms. 
First, you need to grant execution priviliges to the script:
```bash
chmod +x ./setup.sh
```
Then, execute it:
```bash
./setup.sh
```
You may also uninstall `you` with [this](./uninstall.sh) script. You should also grant execution priviliges to the script first. 
```bash
chmod +x ./uninstall.sh
```
Then, execute it:
```bash
./uninstall.sh
```

### Cargo

```bash
cargo install you
```

## Usage

Run a command described in natural language:

```bash
you run "find all large text files in my home directory"
```

The tool will:
1. Process your request through an LLM
2. Suggest appropriate command(s)
3. Ask for your confirmation before execution
4. Handle any errors by suggesting corrections

## Examples

```bash
# Recall commands that you had forgotten
you run "check out what has been imported in this Rust project"

# Don't know what to look after?
you run "how many cores does this system have?"

# Even open up a SSH terminal for you
you run "open a new window with an SSH terminal to xxx.xxx.xxx.xxx as root"
```

If you would like to continually prompt the LLM without losing the context, you may run the below command to enter the interactive mode. This is useful when you would like to perform several tasks at a time. 
```bash
you run
```

You may also would like to save the command(s) generated. `you` will prompt you for whether to save the commands when it is ready. A chain file can be executed by [cchain](https://github.com/AspadaX/cchain), which means that you can reuse the same set of commands without prompting the LLM again and again. 

## Tips

- So far as I tested, `you` works well with a small model called `smollm2` and its smallest version, 135m. 
- You may also use `ollama` with any open source model you want for free.

## Workflow

1. Type your request in natural language
2. Review the suggested command(s)
3. Type 'y' to execute or provide additional guidance/clarification
4. If there's an error, the AI will automatically suggest a corrected command

## Why Use You?

- **Reduces Mental Load**: No need to remember complex command syntax
- **Safer Command Execution**: Review commands before they run
- **Learning Tool**: See how your natural language request translates to actual commands
- **Productivity Booster**: Accomplish complex tasks with simple instructions
- **Interactive Learning**: Learn new commands and techniques through interactive feedback

## License

MIT

## Credits

Created by Xinyu Bao

---

*`you` - because command lines should understand you, not the other way around.*
