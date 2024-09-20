# Common-error: High performance universal error code management for Rust

English | [简体中文](README)

## Overview

Common-error is a High performance universal error code management for **Rust**.

Provide a build to parse common-error.cvs in crate and convert it into a rust file.

All parsing and conversion processes are generated into code in advance to achieve a certain degree of high performance.

## Brief

![CSV.eg](assets/template.png)

- **High Performance**: Literal assembly that generates error messages in advance during the build phase
- **Easy Manager**: Manage error codes through file control

## Issues

Please use this project with caution due to the following issues:

- **Unformatted Generated Code**: The code is displayed in a single line due to the large number of error codes.

![Unformatted](assets/_common_error.png)

- **Manual Inclusion Required**: Due to conflicts between the order of include files and the compilation order when
  macros are expanded, crate compilation may fail, hence manual inclusion is necessary.
  ![Unformatted](assets/use_demo.png)

- **Why Not Use Macros**: The use of macros is not conducive to unified maintenance; if needed, you can contact me via
  email. The following image shows the state of using macros.
  ![Unformatted](assets/use_macro.png)

- **Support for xml, toml, properties**: The data structure validation has been streamlined; if needed, you can contact
  me via email.

> The author has always been excited about maintaining their project in a way that looks simple and sounds simple, thus
> not including what they personally consider to be suboptimal writing methods.

## Quick Start

Step 1: Add dependencies to your crate

```chatinput
[build-dependencies]
common-error-build = "0.1.0"

[dependencies]
common-error = "0.1.0"
```

Step 2: Call the `error_build` function in the build script.
Step 3: Maintain your common-error.csv file.
Step 4: Include the `_common_error` mod in your crate.

## License

This project is licensed under the [Apache 2.0 License](LICENSE).
