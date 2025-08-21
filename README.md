# About
This is a program that installs and uses [scoop](https://scoop.sh) to install a list of personal software I generally setup on my Windows computers. It can also be used to deploy some of my configuration files.
This tool does not require administrator privileges.

# Usage
Download and run the EXE of the latest release, then follow the prompt's instructions.

# Developer
This software is built and tested on Linux with the Nix package manager, then deployed on Windows machines.

## Building
To build the EXE yourself, clone the repo and use the Nix package manager.
You can find the resulting EXE in `./result/bin/winsetup.exe`.
```sh
git clone https://github.com/Shringe/winsetup.git
cd winsetup
nix build
```

## Testing/Debugging
The application automatically enters `dryrun` mode when it is not compiled with release optimizations. In `dryrun` mode, the software is cross platform, and will not make any changes to the system; `dryrun` mode can be used for debugging or exploring the software.

The following code below can be used to compile a debug build (without release optimizations).
The following binary will be in `./target/debug/winsetup`.
```sh
git clone https://github.com/Shringe/winsetup.git
cd winsetup
nix develop
cargo build
```
