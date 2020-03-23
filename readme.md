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

### Activating an environment

To activate an environment which has been created, the `use` command can be used. The name of the environment to activate must be provided.

```bash
$ nen use my-node-project
```

When a project is activated, a few things happen:

+ Your current `$PATH` is copied into `$NEN_OLD_PATH`.
+ `$NEN_HOME/environments/my-node-project/.npm-global` is prepended to your `$PATH`.
+ `$NEN_HOME/environments/my-node-project/bin` is prepended to your `$PATH`.
+ `$NPM_CONFIG_PREFIX` is set to `$NEN_HOME/environments/my-node-project/.npm-global`.
+ `$NEN_ENV` is set to the name of your environment.

Be aware that the changes to your `$PATH` are only **additive**. If you already have a global npm modules folder in your path, **it will stay in your path**. This could result in a global module that was installed on your system becoming available in your activated environment.

### Deactivating an environment

To deactivate an environment, the `stop` command. This command will first try to determine if there is an active environment. If both `$NEN_OLD_PATH` *and* `$NEN_ENV` are unset, it is assumed that there is no active environment and the command stops.

If at least one of these variables is set, the following things happen:

+ `$NEN_ENV` is unset.
+ If `$NEN_OLD_PATH` is set, it is copied into `$PATH` and then unset.
