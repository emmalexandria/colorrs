# colorrs

*Important caveat: In all implementation details, this application should be flawlessly cross-platform. However, I haven't yet had the time 
to actually boot up VMs to test it.*

<img width="1392" height="409" alt="output" src="https://github.com/user-attachments/assets/e815ee96-613e-4c9f-9f43-4c8623bee448" />

Colorrs is an alternative to [shell-color-scripts](https://gitlab.com/dwt1/shell-color-scripts). This was initially motivated simply by the fact that 
it can be hellishly slow at picking a random script.

The headline is that (on my machine) it's much faster at choosing a random script (around here we call them patterns), and about 2-3x faster at
displaying a pre-selected script. In addition, it's effortlessly cross platform. Instead of relying on shell scripts, it has a `.toml` format for patterns. Think
`cowsay` or `figlet` typa deal.

Scripts are easy to port from the original (or you can just use the original scripts, but you'll lose out on a lot of the speed advantage/cross platform-ness).

The main cost of all this niceness is code complexity and size. `shell-color-scripts` has about 91 lines of code. `colorrs` is a mean and lean 1400.

*Note: On Windows, the application forces ANSI colors instead of using Windows APIs, meaning it isn't compatible with older Windows terminals.*

## Features 
- Linux, MacOS, and Windows support
- Automatically download patterns from a Git repository and copy them to the appropriate directory
- Shell completions
- TOML format offering speed and ease advantages in creating scripts
- Pattern listing with preview (optional)

## Installation

`cargo install colorrs`

Once installed, run `colorrs download emmalexandria/colorrs` to automatically download and install the contents of 
the patterns directory. This command works for any Git repository with a `patterns` or `colorscripts` directory.

New pattern files can be created and installed in these default directories.
|Linux/XDG|Windows|MacOS|
------|-------|------
|`$HOME/.config/colorrs`| `C:\Users\{User}\AppData\Roaming\colorrs`| `/Users/{User}/Library/Application Support/colorrs` |


## Usage

```
Usage: colorrs [OPTIONS] <COMMAND>

Commands:
  print     Print a given or random pattern
  list      List available patterns
  download  Download patterns from a git repository
  generate  Generate completions for a given shell
  help      Print this message or the help of the given subcommand(s)

Options:
  -d, --dir <DIRECTORY>  Set a custom directory for pattern description files
  -h, --help             Print help
  -V, --version          Print version
```

*Usage note: the `-d` flag works across commands. For example, `download` will download to the value of `-d` if set.*

```
colorrs print <PATTERN>                   print the given pattern
colorrs print -r                          prints a random pattern

colorrs list                              list available patterns
colorrs list -p                           list w/ pattern preview

colorrs download <OWNER>/<REPOSITORY>     downloads and install patterns from the given GitHub repo
colorrs download <URL>                    same as the GitHub ex. but from a provided Git (http) url

colorrs generate <SHELL>                  generates shell completions (bash, Fish, zsh, elvish, Powershell)
```

## Patterns

There are two kinds of 'patterns' this program can run. A pattern is what `colorrs` calls both shell color scripts and
files written in its TOML format.

### Scripts

The first kind is just any executable program, e.g. a bash script with the correct shebang. Self-explanatory.
Just executes as a subprocess. Support is included for this to make moving over from `shell-color-scripts` easier. 

**N.B. At present no validation is run on these scripts. Any executable in the patterns directory can be 
executed by `colorrs`. For this reason, don't run `colorrs` with sudo/admin privileges. If it asks for them, something is 
up.**

### TOML

*The cool, cross-platform, chef's special way!*

TOML patterns were created with portability in mind and are heavily recommended for new patterns. They are entirely cross platform as the responsibility for them running is placed on `colorrs`, not the shell. They are generally 4x faster than equivalent shell scripts. In addition, I think they are more comprehensible to users wishing to make modifications, although this may increase the complexity of writing them (especially simple pattern scripts).

A pattern file looks like so:

```toml
# example.toml

# Colors within the pattern will be replaced with the appropriate escape code based on exact string matches. 
# If for some bizarre reason your ASCII art needs to contain the characters '{red}', you can simply name your 
# colour slightly differently. No need to mess around with escapes.
pattern = """
  {red}This will be in red {blue}Hello{reset}
{blue}{bold}This is blue and bold{reset}
      {red}And this is back to red. Indentation is preserved due to the multiline string! Feel free 
to just paste                  ASCII art in here
"""

[colors]
# Colours are defined by purely what goes inside the ANSI escape code (\x1b[...m)
# So this code will be interpreted as \x1b[31m, setting the foreground to red
red = "31"
# We can do a 256 color code by simply including the 256 color code marker:
blue = "38;5;25"
bold = "1"
```

This TOML format was designed to be really easy to learn, and (vitally) fast to parse and apply. It's worth noting that `{reset}` is a built in "color" which
will apply the ANSI reset sequence. If you *REALLY* need to create a pattern which uses the exact string `{reset}`, then make an issue and maybe
I'll work out some escaping.

#### Technical details
This format is implemented with `serde` and `toml`, and a simple find and replace for the defined colors is run, in which `{red}` for example will be 
replaced by `x1b[31m` wherever it's present. This means that your patterns can actually define any escape sequences you like in `colorrs`, so you can 
get creative with it.

## Advantages

### Speed

`colorrs` is faster than `shell-color-scripts`. For a specific pattern, both are
faster than human reaction time, but hey, `colorrs` is still roughly 2-3x faster with TOML patterns even if you won't notice.

Here are some quick comparative benchmark results of execution time measured with the `time` command (on an M4 Macbook Air). Please note
that this is comparing the `.toml` format of `colorrs` to the shell scripts of `shell-color-scripts`.

|application|random|arch|alpha|
|--------------|------|----|-------|
|shell-color-scripts|~40-500(!)ms|~28-35ms|~28-35ms|
|colorrs|~12-17ms|~12-15ms|~12-15ms|

For printing a specific script both are very fast. However, for random scripts `colorrs` is significantly faster and doesn't
display the notable delay of `shell-color-scripts`.

### Portability & Ease

Any pattern written in the `.toml` format *should* run on MacOS, Linux, and Windows. It's a universal format, because
`colorrs` acts as the interpretor. In addition, it's my personal opinion that these `.toml` files are easier to read and write
than sh/bash scripts, and porting over those scripts is usually super easy.

## Disadvantages

Maybe I'm just too biased towards my own work, but I don't think there's any disadvantage to using this over `shell-color-scripts`. The `.toml`
format does have a disadvantage however in that one cannot programatically repeat a string, so patterns showing multiple copies of the same
ASCII art in different colours require more manual labour.

## Roadmap

- Add better error handling (too many unwraps around)
- Beautify output
- Add code comments

## Contributing

Please help me convert more of the scripts to `.toml`. I'm begging you. There's like 100 of them. I just. Do not have time.

Also, see [CONTRIBUTING.md](CONTRIBUTING.md)
