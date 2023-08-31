---
title: swimlane-cli task
description: Reference documentation for the swimlane-cli task command.
---

```bash
swimlane-cli task save
swimlane-cli task save -p [PATH]
swimlane-cli task save -a [APPLICATION] -p [PATH]
swimlane-cli task help
```

## Description

Given a directory, `swimlane-cli` will download all python tasks from the specified Swimlane instance and save them to the specified directory.

> ⚠️ Existing tasks will be overwritten so please ensure you have version control configured ⚠️

The format of the downloaded tasks is as follows:

```plaintext
<directory>
├── <application_name>
│   ├── <task_name>.py
│   ├── <other_task_name>.py
```

Sample Output:

```plaintext
    tasks
    ├── Security Information and Event Management
    │   ├── Add to Watchlist.py
    │   ├── Add to Watchlist (CSV).py
    │   ├── Add to Watchlist (JSON).py
    │   ├── Add to Watchlist (XML).py
```

## Options

### `-a, --app`

Specifies the application to download tasks from. If not specified, all tasks will be downloaded.

### `-p, --path`

Specifies the path to save the tasks to. If not specified, the tasks will be saved to the current working directory.

### `-h, --help`

Show the help message

## Caveats

- Non-python tasks will not be downloaded
- Forked tasks will not be downloaded
- Existing tasks will be overwritten
- Task names with special characters will not be downloaded
