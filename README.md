# touchp ✨

A tiny, no-nonsense utility that creates files along a given path,
automatically creating any missing directories on the way. Perfect when
you’re tired of typing `mkdir -p` like an animal 🪓.

🚀 Features

-   🗂️ Creates all missing directories in the specified path
-   📄 Creates an empty file by default
-   ✍️ Writes custom file contents when using `--contents` or `-c`
-   🧘 Works with absolute and relative paths without drama

💡 Usage

Create a file and any missing directories

    touchp foo/bar/baz.txt

Creates:

    ./foo/bar/
    └── baz.txt (empty)

Use an absolute path

    touchp /home/username/foo/bar/baz.txt

Creates the full structure if needed, then the file.

Create a file with contents

    touchp /home/username/foo/bar/baz.txt -c "foobar"

Creates the file and writes:

    foobar

(No quotes included 😎)

🧰 Flags

|Flag|         Alias|   Description|
|------------| -------| ------------------------------------------------|
|`--contents`|   `-c` |     Writes the provided text into the created file|

📜 License

MIT. Go wild.
