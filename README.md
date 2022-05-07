# What is this ?

A JavaScript environment where AvdanOS Search extensions can run.

## Features :
- API
    - OS Integration
        - Clipboard Access ✅
        - File System Access ⏲️ 
    - Network Access ⏲️


## Security Policy

The security policy of a particular extension is defined in the `security` section of its `manifest.avdan.json` file.
An extension's security policy determines the various actions the extension can make.

### Permissions
In `security.permissions`, the manifest can outline which API features the extension wishes to use.
If an extension tries to use an API feature which has not been declared, a `SecurityException` is thrown.

Similarly, if an extension uses external commands, they must also be declared in the manifest `security.commands`. 
```
avdan
│   
├───clipboard
│   │   
│   │   read    --- Reading of the contents of any clipboard.
│   │   write   --- Modifying/Writing to the contents of any clipboard.
│   │   type    --- Sending keystrokes.
│
├───file
│   │   
│   │   read    --- Reading from a file.
│   │   write   --- Writing to a file.
```

## Core Avdan API dependencies
- xclip
- xdotool
