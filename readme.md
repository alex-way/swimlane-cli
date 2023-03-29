# Swimlane CLI

A simple CLI to ease management of Swimlane instances.

⚠️ This tool is still in development and is not yet ready for production use. Please proceed with caution.

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
export SWIMLANE_CLI__SOURCE_SWIMLANE_URL=https://swimlane.example.com
export SWIMLANE_CLI__SOURCE_SWIMLANE_PAT=my-pat
swimlane-cli [command]
```

## Commands

### download-python-tasks

Downloads all Python tasks from a Swimlane instance and saves them to a local directory.

The format of the files will be: `app_name/task_name.py`. For example:

```bash
├── my_app
│   ├── siem__create_ticket.py
└────── siem__get_ticket.py
```

```bash
swimlane-cli download-python-tasks [directory]
```

### pip

A number of subcommands to manage Python packages on a Swimlane instance.

| Subcommand            | description                                           |
| :-------------------- | :---------------------------------------------------- |
| pip install [package] | Installs a specified package or requirements.txt file |
| pip remove [package]  | Removes a specified package from the Swimlane server  |
| pip freeze            | Lists all installed packages                          |

#### pip install

#### Usage

```bash
swimlane-cli pip install [package]

# Install a package
swimlane-cli pip install requests

# Install a package from a requirements.txt file
swimlane-cli pip install -r requirements.txt
```

##### Flags

- `-r` (string) - Path to a requirements.txt file to install

#### pip remove

#### Usage

```bash
swimlane-cli pip remove [package]

# Remove a package
swimlane-cli pip remove requests
```

#### pip freeze

#### Usage

```bash
swimlane-cli pip freeze

# List all installed packages
swimlane-cli pip freeze
> requests==2.25.1
> urllib3==1.26.2
> swimlane==10.12.0
```

### migrate

A number of subcommands to migrate content from one Swimlane instance to another.

Please note that an additional `--destination-swimlane-url` and `--destination-swimlane-pat` flag must be set for all `migrate` commands. Alternatively you can set the following environment variables: `SWIMLANE_CLI__DESTINATION_SWIMLANE_URL` and `SWIMLANE_CLI__DESTINATION_SWIMLANE_PAT`.

| Subcommand                 | description                                                                                           |
| :------------------------- | :---------------------------------------------------------------------------------------------------- |
| migrate users              | Migrates all users from the source Swimlane server to the destination Swimlane server                 |
| migrate user [username]    | Migrates the specified user from the source Swimlane server to the destination Swimlane server        |
| migrate groups             | Migrates all groups from the source Swimlane server to the destination Swimlane server                |
| migrate group [group_name] | Migrates the specified group from the source Swimlane server to the destination Swimlane server       |
| migrate roles              | Migrates all roles from the source Swimlane server to the destination Swimlane server                 |
| migrate role [role_name]   | Migrates the specified role from the source Swimlane server to the destination Swimlane server        |
| migrate apps               | Migrates the specified application from the source Swimlane server to the destination Swimlane server |
| migrate app  [app_name]    | Migrates the specified application from the source Swimlane server to the destination Swimlane server |
| migrate all                | Migrates all possible content from the source Swimlane server to the destination Swimlane server      |
