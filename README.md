# jeff-normile-artifact
Artifact repository for the 2023-2024 SE Senior Comp

## Installing Rust

This project requires a working Rust installation. It's recommended to use `rustup`, a version manager that makes the installation process incredibly straightforward.

On Linux or Mac, use the following command to install `rustup` (which will also install the latest stable version of Rust):

```
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

On Windows, use [this link](https://www.rust-lang.org/tools/install) to the official Rust download page to download the appropriate `rustup` tool and stable Rust release for your system architecture.

## Running `ast-complexity`

After cloning this Git repository onto your device, navigate to the `ast-complexity` directory within your preferred terminal. From there, run the following command:

```
cargo run -- data/python/hello-world.py python
```

This will run the `ast-complexity` tool using the prepackaged `hello-world.py` file included with the repository.

The `data/python/hello-world.py` and `python` arguments can be replaced with a relative path to a file of your own choosing, as well as the programming language that the file is written in.

## Supported Languages

Currently, this tool supports five programming languages, with the goal to expand this to include the wide ecosystem of open source `tree-sitter` grammars currently available. The currently supported languages are:

* C
* JavaScript
* JSON
* Python
* Rust

## Collected Data

This repository contains a `data` folder that comes prepackaged with a number of sample programs that can be used to see the software artifact in action! Many of these programs are grouped into directories that contain programs that all accomplish the same task (albeit in different languages). These programs were collected from [The Algorithms](https://the-algorithms.com/).

## About the Software Engineering Process

Given that this is a senior comp project for the Software Engineering major, documentation pertaining to various aspects of the actual software engineering process itself is contained below. 

### User Stories

According to the folks at [Atlassian](https://www.atlassian.com/agile/project-management/user-stories), the company behind popular enterprise-level software engineering solutions like Confluence and Jira, a user story is "an informal, general explanation of a software feature written from the perspective of the end user".

The user stories below detail the requirements that the software artifact ought to satisfy in order to be considered *complete*.

* "As an *educator*, I want to *quickly identify complex and deeply nested code*, so that I can *ensure my assignments are suitably complex*."

* "As a *developer*, I want a *metric that can indicate code legibility/complexity*, so that I can *develop maintainable code*."

* "As a *student*, I want to *identify parts of my code that are overly complex*, so that I can *build good habits when it comes to writing readable and maintainable code*.

## Further Insights: Research Notebook

To view ongoing documentation pertaining to first reader meetings and additional research that supports the development of this software artifact, please view the GitHub discussion for my research notebook [here](https://github.com/orgs/ReadyResearchers-2023-24/discussions/7).