# Colorrs

*Warning to those coming from Reddit: I literally just made this this morning, and so far it has only been tested on MacOS. There's no reason it wouldn't
work on Windows or Linux, but you know how this stuff goes. The code is still a little messy and unit-test free.*

<img width="1392" height="409" alt="output" src="https://github.com/user-attachments/assets/e815ee96-613e-4c9f-9f43-4c8623bee448" />

Colorrs is a *blazingly fast* 🚀, *completely* ***pointless*** rewrite in Rust 🦀. I kid. I was using [shell-color-scripts](https://gitlab.com/dwt1/shell-color-scripts) in my configuration and found that it was a bit slow at selecting random scripts. As all logical software engineers do, I rewrote it in Rust.

The headline is that (on my machine) it's roughly 10x faster at choosing a random script (around here we call them patterns), and about 4x faster at
displaying a pre-selected script. In addition, it's effortlessly cross platform. Instead of relying on shell scripts, it relies on a `.toml` format. Think
`cowsay` or `figlet` typa deal.

Scripts are easy to port from the original (or you can just use the original scripts, but you'll lose out on a lot of the speed advantage/cross platform-ness).

*Note: On Windows, the application forces ANSI colors instead of using Windows APIs, meaning it isn't compatible with older Windows terminals.*

## Installation

`cargo install colorrs`

Once installed, copy the contents of the `patterns` directory to:

|Linux/XDG|Windows|MacOS|
------|-------|------
|`$HOME/.config/colorrs`| `C:\Users\{User}\AppData\Roaming\colorrs`| `/Users/{User}/Library/Application Support/colorrs` |

The `patterns` directory of this repo only contains `.toml` patterns. If you want a wider variety, feel free to drop script patterns from the original
into your pattern directory.

## Usage

```
A Rust CLI for outputting terminal colour test images

Usage: colorrs [OPTIONS]

Options:
  -p, --print <PATTERN>  Print the given pattern
  -r, --random           Choose a random pattern
  -l, --list             List all available patterns
  -d, --dir <DIRECTORY>  Set a custom directory for pattern description files
  -h, --help             Print help
  -V, --version          Print version
```

## Patterns

There are two kinds of 'patterns' this program can run. A pattern is what `colorrs` calls both shell color scripts and
files written in its TOML format.

### Scripts

The first kind is just any executable program, e.g. a bash script with the correct shebang. Self-explanatory.
Just executes as a subprocess.

### TOML

*The cool, cross-platform way which gets the chef's recommendation!*

TOML patterns were created with portability in mind and are heavily recommended. They are entirely cross platform as the responsibility for them running is placed on `colorrs`, not the shell. They have also been proven to be faster. In addition, I think they are more comprehensible to users wishing to make modifications, although this may increase the complexity of writing them (especially simple pattern scripts).

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

## Advantages

### Speed

`colorrs` is much faster than `shell-color-scripts`, especially in selecting a random pattern. For a specific pattern, both are
faster than human reaction time, but hey, `colorrs` is still roughly 4x faster even if you won't notice.

Here are some quick comparative benchmark results of execution time measured with the `time` command (on an M4 Macbook Air). Please note
that this is comparing the `.toml` format of `colorrs` to the shell scripts of `shell-color-scripts`.

|application|random|arch|alpha|
|--------------|------|----|-------|
|shell-color-scripts|~400ms|~30ms|!30ms|
|colorrs|~30ms|~9ms|~9ms|

Obviously for printing a specific script, both are very fast. However, for random scripts `colorrs` is significantly faster, and doesn't
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

- Command which uses `wget` or `curl` to download the pattern files from this repository and put them in the correct place.
- Add better and formatted error handling (STOP PUTTING IT OFF!!!)

## Contributing

Please help me convert more of the scripts to `.toml`. I'm begging you. There's like 100 of them. I just. Do not have time.

Also, see [CONTRIBUTING.md](CONTRIBUTING.md)
