# Why this?

We wanted an easier way of building our mods for [Unreal ModLoader]() so I made this tool to simplify building & copying the files to their proper names into the proper folder for your mod

# Building the program

Make sure you have [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) installed and then open a command line / Powershell window

```
git clone https://github.com/RedBrumbler/UnrealModLoaderTool.git
cd UnrealModLoaderTool
cargo build --release
```

# Using the program

Download the program from [Github Actions](https://github.com/RedBrumbler/UnrealModLoaderTool/actions/workflows/cargo-build.yml)

Make sure you have a valid mods.config.json in your Project folder. An example can be [seen here](https://github.com/RedBrumbler/UnrealModLoaderTool/example.mods.config.json)

then you should be able to run the command as follows:

```
ue-modloader-tool ./ProjectName.uproject -copy
```

and the program should package your project and copy over the pak files for you into the appropriate folder.

feel free to use the help commands to explore more options

```
ue-modloader-tool --help
```