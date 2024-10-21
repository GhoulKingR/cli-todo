# CLI ToDo list app
A CLI ToDo list app built in Rust.

# Installing the command
To install `cli-todo` follow these steps:
1. Clone this github repository to your system
```bash
git clone https://github.com/GhoulKingR/cli-todo.git
```
2. **Cd** to the `cli-todo` folder.
```bash
cd cli-todo
```
3. Run the cargo install command:
```bash
cargo install --path .
```

After installing, you need to include cargo's bin directory to your system path if it's not already there. You can do that by:
* Bash, Zsh: Adding this line to your `~/.bashrc` or `~/.zshrc`:
```bash
export PATH="$HOME/.cargo/bin:$PATH"
```
* Fish: Add this line to your `~/.config/fish/config.fish`:
```bash
set -gx PATH $HOME/.cargo/bin $PATH
```

After adding its bin directory to your system path, run this command to activate the command in your current shell:
```bash
source ~/.bashrc  # or ~/.zshrc, depending on the shell
```

Now, you can verify that the installation was successful by running this command in the terminal:
```Bash
$ cli-todo -h
CLI ToDo list app

Usage: cli-todo [OPTION]

Options:
    --help, -h                      Display this help menu
    --list, -l                      List all items in the todo list
    --add, -a                       Interactively add a new todo item
    --preview ITEM, -p ITEM         Preview an item in more detail
    --edit-note ITEM, -en ITEM      Edit an item's note
    --edit-title ITEM, -et ITEM     Edit an item's title
    --toggle ITEM, -t ITEM          Toggle complete status of an item
    --delete ITEM, -d ITEM          Delete an item from the todo list
    --erase-all                     Delete every item in the todo list
```

# Uninstalling the command
If you wish to uninstall `cli-todo`, run this command:
```bash
cargo uninstall your_command_name
```

# Basic actions

## Listing all items in the todo list
To list all items in your todo list, run the `cli-todo` command with the `--list` or `-l` flag. For example:
```Bash
$ cli-todo --list
All items:
1. [ ] First task
2. [x] Second task
3. [x] Third task
4. [ ] Fourth
```

> Items with `[x]` before it's title are completed tasks, while items with `[ ]` are not completed

## Previewing items
`cli-todo` allows you to see more information about an item with the `--preview` or `-p` flag followed by the item index (1-index). For example:
```Bash
$ cli-todo --preview 4
Title: Fourth task
Completed: true

This is a more in-depth description
```

## Adding items to the todo list
To add a new item to the todo list, use the `--add` or `-a` flag, and follow the prompts. Like so:
```Bash
$ cli-todo --add
```

## Editing existing title
To change an existing item's title, use the `--edit-title` or `-et` flag followed by the item's index. For example:
```bash
$ cli-todo --edit-title 4
Original title: Fourth task
New title (Leave empty to leave unchanged): Fourth

$ cli-todo --list
All items:
1. [ ] First task
2. [x] Second task
3. [x] Third task
4. [ ] Fourth
```

## Editing existing notes
To change an existing item's note, use the `--edit-note` or `-en` flag followed by the item's index. This would open a vi instance with the item's existing note for you to edit an save. Run the command like so:
```bash
$ cli-todo --edit-note 4
```

## Toggling tasks between completed and uncompleted
To toggle tasks between completed and uncompleted, use the `--toggle` or `-t` flag followed by the item's index. Like so:
```bash
$ cli-todo --toggle 1
Title: First task
Original: false
Current: true

$ cli-todo --list
All items:
1. [x] First task
2. [x] Second task
3. [x] Third task
4. [ ] Fourth
```

## Deleting items
To delete items in the todo list, use the `--delete` or `-d` flag followed by the item's index. Like so:
```bash
$ cli-todo -d 4
Are you sure you want to delete this item "Fourth"? [yN] y

$ cli-todo --list
All items:
1. [x] First task
2. [x] Second task
3. [x] Third task
```

## Erasing all items
In case you want to delete every item in the todo list, `cli-todo` provides an `--erase-all` command:
```bash
$ cli-todo --erase-all
Are you sure you want to erase all items? [yN] y
```
