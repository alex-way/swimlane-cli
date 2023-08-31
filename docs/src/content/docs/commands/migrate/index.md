---
title: swimlane-cli migrate
description: Reference documentation for the swimlane-cli migrate command.
---

```bash
swimlane-cli migrate
swimlane-cli migrate users
swimlane-cli migrate user USERNAME
swimlane-cli migrate groups
swimlane-cli migrate group GROUP_NAME
swimlane-cli migrate roles
swimlane-cli migrate role ROLE_NAME
swimlane-cli migrate apps
swimlane-cli migrate app APP_NAME
```

## Description

The `swimlane-cli migrate` command is used to migrate resources from one Swimlane instance to another.

## Options

### `--target-url`

The URL of the Swimlane instance to migrate to. Can also be configured with the `SWIMLANE_CLI__TARGET_URL` environment variable.

### `--target-pat`

The Access Token for the target instance. Can also be configured with the `SWIMLANE_CLI__TARGET_URL` environment variable.

### `--dry-run`

Whether or not to perform a dry run of the migration.

### `--auto-approve`

Automatically approve the migration without prompting.

### `-h, --help`

Show the help message
