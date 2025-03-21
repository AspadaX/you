# You - Your Optimized UNIX, but also supports Windows

`you` is a command-line tool that translates natural language instructions into executable shell commands, making command-line operations more accessible and intuitive.

## Features

- **Natural Language Command Execution**: Type what you want to do in plain English, or any other natural language you prefer, and let the AI figure out the exact commands
- **Interactive Workflow**: Review suggested commands before execution
- **Error Handling**: When commands fail, the AI automatically suggests fixes based on error messages
- **Semi-Autonomous Operation**: The AI handles the technical details while keeping you in control

## Installation

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
# Organize files
you run "organize my downloads folder by file type"

# Find system information
you run "show me system resource usage"

# Complex data processing
you run "extract the second column from my CSV file and count unique values"
```

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
