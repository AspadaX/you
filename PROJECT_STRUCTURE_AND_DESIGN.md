# You - Project Structure and Design Documentation

## Project Overview

`you` is a Rust-based command-line tool that translates natural language instructions into executable shell commands using Large Language Models (LLMs). The project follows a modular architecture with clear separation of concerns.

## Project Structure

```
you/
├── .github/
│   └── workflows/
│       └── release.yml          # GitHub Actions for releases
├── src/
│   ├── agents/                  # AI agent implementations
│   │   ├── command_json.rs      # JSON command structures
│   │   ├── command_line_explain_agent.rs  # Command explanation agent
│   │   ├── mod.rs               # Module declarations
│   │   ├── semi_autonomous_command_line_agent.rs  # Main command agent
│   │   └── traits.rs            # Agent trait definitions
│   ├── arguments.rs             # CLI argument parsing
│   ├── cache.rs                 # Command caching system
│   ├── configurations.rs       # User configuration management
│   ├── constants.rs             # Application constants
│   ├── helpers.rs               # Utility functions
│   ├── information.rs           # System context gathering
│   ├── llm.rs                   # LLM client and communication
│   ├── main.rs                  # Application entry point
│   ├── shell.rs                 # Shell command execution
│   ├── styles.rs                # UI styling and formatting
│   └── traits.rs                # Global trait definitions
├── Cargo.toml                   # Rust package configuration
├── README.md                    # Project documentation
├── README_CN.md                 # Chinese documentation
├── objective.md                 # Project objectives
├── setup.sh                     # Installation script
├── update.sh                    # Update script
└── uninstall.sh                 # Uninstallation script
```

## Architecture Design

### Core Components

#### 1. Agent System (`src/agents/`)
The project uses an agent-based architecture for processing natural language commands:

- **SemiAutonomousCommandLineAgent**: Main agent that breaks down natural language into executable commands
- **CommandLineExplainAgent**: Specialized agent for explaining existing commands
- **Traits**: Define common interfaces for agent behavior (`Step`, `Context`, `AgentExecution`)

#### 2. LLM Integration (`src/llm.rs`)
- Abstracts OpenAI API communication
- Supports configurable API endpoints and models
- Implements context management for conversation history
- Uses async/await pattern with Tokio runtime

#### 3. Configuration System (`src/configurations.rs`)
- JSON-based configuration storage
- User preferences for CLI tools
- Cache enablement settings
- Global resource initialization pattern

#### 4. Caching System (`src/cache.rs`)
- Stores previously generated commands for reuse
- File-based storage in user's home directory
- Search functionality for command retrieval

#### 5. Command Processing Pipeline
```
Natural Language Input → Agent Processing → LLM Translation → 
User Confirmation → Command Execution → Optional Caching
```

### Design Patterns

#### 1. Trait-Based Architecture
- **Context Trait**: Manages conversation history and message handling
- **Step Trait**: Defines workflow steps for agents
- **GlobalResourceInitialization**: Standardizes resource setup

#### 2. Command Pattern
- Commands are represented as structured JSON objects
- Different action types (Execute, Explain, etc.)
- Separation of command representation from execution

#### 3. Strategy Pattern
- Different agents for different types of operations
- Pluggable LLM backends through configuration
- Configurable command interpreters

#### 4. Builder Pattern
- Used extensively with OpenAI API message construction
- Clap command-line argument building

### Key Dependencies

- **async-openai**: OpenAI API client
- **clap**: Command-line argument parsing
- **tokio**: Async runtime
- **serde**: Serialization/deserialization
- **anyhow**: Error handling
- **cchain**: Display and UI utilities
- **console**: Terminal styling
- **indicatif**: Progress indicators

## Data Flow

### 1. Initialization Phase
```
Parse CLI Arguments → Initialize Configurations → 
Initialize Cache → Load Contextual Information
```

### 2. Command Processing Phase
```
User Input → Agent Creation → LLM Context Setup → 
Natural Language Processing → Command Generation → 
User Confirmation → Execution → Optional Caching
```

### 3. Interactive Mode
```
Continuous Loop: User Input → Processing → Execution → 
Context Update → Next Input
```

## Security Considerations

- API keys stored in environment variables
- User confirmation required before command execution
- No hardcoded credentials in source code
- Shell command validation through LLM processing

## Extensibility Points

1. **New Agent Types**: Implement `Step` and `Context` traits
2. **Additional LLM Providers**: Extend LLM client abstraction
3. **Custom Command Types**: Add new action types to command JSON
4. **Storage Backends**: Implement alternative caching mechanisms
5. **UI Enhancements**: Extend styling and display utilities

## Configuration

### Environment Variables
- `YOU_OPENAI_API_BASE` / `DONE_OPENAI_API_BASE`: API endpoint
- `YOU_OPENAI_API_KEY` / `DONE_OPENAI_API_KEY`: Authentication key
- `YOU_OPENAI_MODEL` / `DONE_OPENAI_MODEL`: Model selection

### User Configuration File
- Located in user's home directory
- JSON format for preferences and settings
- Cache enablement and CLI preferences

## Error Handling

- Uses `anyhow` crate for error propagation
- Comprehensive error messages for user guidance
- Graceful handling of LLM API failures
- Validation of user inputs and system requirements

## Performance Considerations

- Async/await for non-blocking LLM API calls
- Local caching to reduce API calls
- Efficient file-based storage for command history
- Progress indicators for long-running operations

## Future Enhancements

1. Support for additional shell interpreters
2. Enhanced caching with semantic search
3. Plugin system for custom agents
4. Web interface for command management
5. Integration with additional LLM providers
6. Command validation and safety checks
7. Batch command processing capabilities