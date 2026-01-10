# Open Path Resolver Framework

## Overview

This project is designed to answer two questions. How to build a project or workspace 
folder structure and how to query for where to write to/where to find paths.

## Features

- A Rust and Python 3 API
- Support for building out a filesystem for a given context.
- Support for querying for paths and extracting information from paths.

## Requirements

- Rust: 1.92 or later (This is not the guaranteed minimum supported Rust
  version)

## Design

### Workspace

The workspace resolver is designed to not directly build a workspace. Instead it will 
provide information to an IO function that is responsible for creating the workspace. 
It is always assumed that the resolver cannot understand what the "root user" or "read 
and write permissions" means for a given organization. Instead, it'll provide context
to a developer so they can decide where to create a file or directory, what are the 
permissions, owner, etc.

### Path

The path resolver is designed to take some fields and a key, then return the path. 
There are sibling functions that can extract the information from the path based on 
what information is provided. For example, `get_key` will get the key from a path and 
fields, while `get_fields` will get the fields from the path and key. The `find_paths`
is slightly different in that it will find all of the paths for a key, while the fields
will control the filtering. For example, if there's a path 
`path/to/{entity}/{version}`, and the fields `{"entity": "foo"}` are supplied, then 
this will find all of the "foo" version paths such as 
`["path/to/foo/001", "path/to/foo/002"]`.

## Install

### Rust

```bash
cd /to/your/project
cargo add openpathresolver
```

### Python

#### For development

```bash
cd /path/to/openpathresolver/bindings/python

python -m pip install ".[build]"
python -m maturin develop
```
