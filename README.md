# MyShorten
version 1.0.1

It's a really simple CLI tool, you just add links to it and then using your code you can get the link. I made it just to practise new crates on Rust, such as `rusqlite` to make queries to the table and `clap` to parse commands from user input.

---

## Adding to your PATH
- ## Windows
1. Click `Win` + `R`

2. Type `sysdm.cpl`

3. Go to the `Advanced` tab

4. Then in user variables choose `Path` and click `Edit`

5. After that just click `New` and paste the path to the folder containing the binary file (shortcut.exe)\
E.g. `C:/Users/User1/Downloads/Folder/`

6. Then restart a terminal, then try to type there `shortcut --help`

- ## Linux / MacOS
1. Go to the directory containing the binary file
2. Then just type `export PATH="/dir/with/binary/:$PATH"`



## Commands
`shortcut add <YOUR_LINK> <CODE>` - adds to the sql table your URL with code. SQL table is in your home directory in MyShorten directory now
`shortcut get <CODE>` - returns the URL associated with the given code

## What's NEW?
- Now you can freely use PATH, without any concern.