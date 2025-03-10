mainObjective {
  input: "operations described in natural languages",
  output: "command execution result",
}

breakdown NaturalLanguageToCommands {
  - use the LLM to dissect the user query into smaller and actionable commands.
  - ask the user for permisson on executions.
  - execute the commands.
}
