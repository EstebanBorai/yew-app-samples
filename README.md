<div>
  <div align="center" style="display: block; text-align: center;">
    <img src="https://avatars1.githubusercontent.com/u/49116234?s=200&v=4" height="120" width="120" />
  </div>
  <h1 align="center">yew-app-samples</h1>
  <h4 align="center">Samples of WASM applications with Rust and Yew</h4>
</div>

## Overview

This repository contains an set of projects to explore WASM capabilities with
Rust's Front-End library Yew.

If you are familiar with either ReactJS or Elm, Yew will be familiar to you.

## Usage

Scripts to consume these projects are available in the `bin` directory.

* `build`: Builds the specified project
* `setup`: Install dependencies using `cargo`

### Running a Project

In the following sample we build the `counter` project, keep in mind that
the project to build could be any of the projects listed in the [Projects](#Projects)
section.

```bash
# build the counter project
bin/build counter

# runs the http-server in the static directory
http-server ./counter/index.html

# open your browser in http://localhost:7878
```

## Projects

* `counter`: A basic counter using Yew's callbacks and state capabilities
* `adder`: A simple sum calculator

## References

These projects are inspired in Carlo Milanesi's book "Creative Projects for Rust
Programmers". The final projects are writen using a different setup and approach
but following the same goals.
