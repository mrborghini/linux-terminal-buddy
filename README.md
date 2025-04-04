# Linux Terminal Buddy

This application is able to use a LLM to execute commands on Linux.

## Disclaimer

Do **NOT** run this on important hardware. It might ruin your OS. You are giving
control to a terminal to an AI.

## Usage

```bash
cargo run --release
```

By default it will use Ollama, but if you want to use OpenAI you need to do:

```bash
cp .env.example .env
```

And then add your API key to the `.env` file.

By default for security reasons the `ALLOW_ALL_COMMANDS` in the `.env` is set to
`false`. If you set that to `true` it will run the commands automatically
without asking confirmation.
