# Swimlane CLI

A simple CLI to ease management of Swimlane instances.

⚠️ This tool is still in development and is not yet ready for production use. Please proceed with caution.

## Installation

### Install from source

```bash
git clone
cd swimlane-cli
pip install -e .
```

## Usage

### download-python-tasks

Downloads all Python tasks from a Swimlane instance and saves them to a local directory.

```bash
swimlane-cli download-python-tasks --host https://my-swimlane-instance.com --username admin --password admin --output-dir /path/to/output/dir
```
