# Raddon

A Rust CLI program that update your wow addon and install new one

Usage: raddon [OPTIONS]

Options:

* -f, --folder <FOLDER>    Optionnal : Path to your Addon Folder [default: ]
* -i, --install <INSTALL>  Optionnal : Install addon from git url (folder must be specified or in file.txt)
* -h, --help               Print help
* -V, --version            Print version


exemple 

check for update : 
```
raddon --folder /path/to/wow/addon
```

install a new addon

```
raddon --folder /path/to/wow/addon --install url/to/repo.git
```