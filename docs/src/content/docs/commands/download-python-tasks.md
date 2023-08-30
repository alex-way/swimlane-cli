---
title: swimlane-cli download-python-tasks
description: Reference documentation for the swimlane-cli download-python-tasks command.
---

```bash
swimlane-cli download-python-tasks PATH
swimlane-cli download-python-tasks help
```

## Description

Given a directory, `swimlane-cli` will download all python tasks from the specified Swimlane instance and save them to the specified directory.

> ⚠️ Existing files will be overwritten so please ensure you have version control configured ⚠️

The format of the directory is as follows:

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

| Option       | Description           |
| :----------- | :-------------------- |
| `-h, --help` | Show the help message |

## Caveats

The following caveats apply to this command:

- Non-python tasks will not be downloaded.
- Forked tasks will not be downloaded.
