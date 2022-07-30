# headers-vy

Generate perfect _Vyper-compatible_ code headers every time.

## Build

You need Rust and Cargo installed on your machine. See the installation guide
[here](https://doc.rust-lang.org/cargo/getting-started/installation.html).

Then clone the repo and install the CLI globally like this:

```sh
cargo install --path .
```

## Usage

```sh
λ ./headers-vy "testing 123"
```

```sh
################################################################
#                         TESTING 123                          #
################################################################
```

It will also copy the header to your clipboard automatically.

### With VSCode

Set your global [`tasks.json`](https://stackoverflow.com/questions/41046494/making-global-tasks-in-vs-code) like so to add the command as task:

```json
{
  "version": "2.0.0",
  "tasks": [
    {
      "label": "Generate Vyper Header",
      "type": "shell",
      "command": "headers-vy ${input:header}",
      "presentation": {
        "reveal": "never"
      }
    }
  ],
  "inputs": [
    {
      "id": "header",
      "description": "Header",
      "type": "promptString"
    }
  ]
}
```

To really speed-up your workflow, you can even add a keybind for the task in [`keybindings.json`](https://code.visualstudio.com/docs/getstarted/keybindings):

```json
[
  {
    "key": "CMD+y",
    "command": "workbench.action.tasks.runTask",
    "args": "Generate Vyper Header"
  }
]
```

This will copy the generated header to your clipboard.

## Credits

Forked from [`headers`](https://github.com/transmissions11/headers).
