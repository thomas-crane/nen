# nen

A simple Node.js environment manager.

## About

nen is a small CLI tool for managing Node.js environments. The primary goal of nen is to maintain a small API (small enough for you to remember all of the commands), yet still remain functional enough to provide some benefit.

## Usage

### Creating new environments

To create a new Node.js environment, the `new` command can be used. The name of the environment and the desired Node.js version must be provided to the command.

```bash
$ nen new my-node-project -v12
```

The version number can be

+ Just a major version number (e.g. `10` or `13`), in which case the latest release of that major version will be used.
+ A major version and a minor version (e.g. `10.18`, `13.9`), in which case the latest release of that minor version will be used.
+ A specific version (e.g. `10.18.0`, `13.9.2`), in which case that specific version will be used.

If the requested version of Node.js has not been used before, it will be downloaded.

Once the appropriate Node.js version is available, a new environment folder will be created in the nen home directory (`$NEN_HOME`, or `$HOME/.nen` by default).

This environment folder will contain a `bin/` directory which contains symlinks to the appropriate Node.js binaries, and an `.npm-global/` folder in which global npm modules for that environment will be stored.
