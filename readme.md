# Swimlane CLI

A simple CLI to ease management of Swimlane instances.

⚠️ This tool is still in development and is not yet ready for production use. Please proceed with caution.

View the documentation at [https://alex-way.github.io/swimlane-cli/](https://alex-way.github.io/swimlane-cli/)

## Installation

### Install from source

## Usage

```bash

swimlane-cli [command]


# Run `help` for detailed information about CLI commands
swimlane-cli [command] help

# All CLI commands need either the --source-swimlane-url and --source-swimlane-pat flags set:
swimlane-cli --source-swimlane-url https://swimlane.example.com --source-swimlane-pat my-pat [command]

# Or alternatively set them as environment variables:
export SWIMLANE_CLI__URL=https://swimlane.example.com
export SWIMLANE_CLI__PAT=my-pat
swimlane-cli [command]
```
