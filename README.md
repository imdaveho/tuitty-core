# tuitty
A cross platform, interoperable, simplfied terminal library that is meant to be wrapped by multiple languages.

![tuitty-banner](https://user-images.githubusercontent.com/13990019/68438603-a6972e00-0192-11ea-8fc9-ff334ee79432.png)

> **tui • tty** \ˈtwē-dē \ *n.* - **t**ext-based **u**ser **i**nterface library for **t**ele**ty**pe writers (aka. **terminals**)

## Table of Contents
* [Features](#sparkles-features)
* [Rationale](#thought_balloon-rationale)
* [Definitions](#notebook_with_decorative_cover-definitions)
* [Getting Started](#zap-getting-started)
  * [API Design](#api-design)
  * [Dispatcher](#dispatcher)
  * [Event Handle](#event-handle)

### :sparkles: Features
[(Back to top)](#table-of-contents)

* Cross-platform (Linux, Mac, Windows)
* Focused (read: _small_) API footprint - unified, consistent capabilities across terminals
  - avoid leaky abstractions that force you to think about what may or may not work
  - prefer to keep dependencies limited (*Unix*: __libc__, *Windows*: __winapi__) and avoid including the kitchen sink
* Thread-safe (guarantees provided by Rust's Send + Sync traits)
* **Cursor navigation** - _eg. goto(col, row), move up/down/left/right, alternate screen_
* **Screen manipulations** - _eg. resize, clear, print_
* **Styling output** - _eg. fg and bg colors, bold, dim, underline_
* **Terminal settings** - _eg. raw/cooked, hide/show cursor, mouse on/off_
* **User input handling** - _eg. keyboard/mouse events_
* Minimal memory `unsafe` code: only OS specific calls and FFI which follows the [Rust FFI Nomicon](http://jakegoulding.com/rust-ffi-omnibus/) very closely

### :thought_balloon: Rationale
[(Back to top)](#table-of-contents)
* **Why not use _curses_?**
  While _[n/pd]curses_ is widely used and wrapped, there is also plenty issues regarding them: wide character support, cross-platform support, [C-style/low-level imports](https://pypi.org/project/blessings/#before-and-after) that reduce clarity, etc.


* **Why not use _[blessings](https://github.com/erikrose/blessings) (Python)_, _[tty-tk](https://github.com/piotrmurach/tty)_ (Ruby), _[terminal-kit](https://github.com/cronvel/terminal-kit)_ (Node), or _[insert project](#rationale)_ (_insert language_)?**
  As you can see, there is already a proliferation of various implementations of terminal libraries...and yes I'm aware of the irony that this project is [+:one:](https://xkcd.com/927/)  to the list of implementations out there. 

  However, unlike other attempts, what this project intends to do is to create a unifying API across languages that eliminates the need to repeat yourself. This is actually very similar to how [asdf-vm](https://asdf-vm.com/#/?id=ballad-of-asdf) addressed the proliferation of "version managers" like `rbenv`, `gvm`, `nvm`, and `pyenv`. By creating something unifying and extensible, users won't have to re-discover and re-learn a new API every time they switch programming languages.
  
  Additionally, many of the implementations out there do not provide cross-platform support (mainly Windows Console), which I'm specifically targeting with this project.
  
* **Why the command line? Why cross-platform? Why, why, why?!**
  At the end of the day, many development workflows begin and end with a terminal prompt. I wanted to learn and better understand this critical component of a software engineer's journey. Consequently, this process has gotten me familiar with systems programming languages (Rust, Go, C, and Nim), low-level OS syscalls, the Windows Console API, and countless other intangibles that have made me a more well-rounded individual.


### :notebook_with_decorative_cover: Definitions
[(Back to top)](#table-of-contents)
* **cross-platform**
* **interoperable**
* **simplied**

### :zap: Getting Started
[(Back to top)](#table-of-contents)

#### API Design

#### Dispatcher
[(Back to top)](#table-of-contents)

#### Event Handle
[(Back to top)](#table-of-contents)

#### Tested Terminals
* Windows 10 - Cmd.exe (legacy and  modern modes)
* Windows 10 - PowerShell (legacy and modern modes)
* Windows 10 - git-bash (w/ [winpty](https://stackoverflow.com/questions/48199794/winpty-and-git-bash))
* Ubuntu 17.04 - gnome-terminal

### Contributing

Please read [CONTRIBUTING.md](https://gist.github.com/PurpleBooth/b24679402957c63ec426) for details on our code of conduct, and the process for submitting pull requests to us.

Specifically, there are labels created for each of these areas:
* <kbd>[unicode](https://github.com/imdaveho/tuitty/labels/unicode)</kbd> language support
* <kbd>[unicode](https://github.com/imdaveho/tuitty/labels/unicode)</kbd> emoji support
* <kbd>[interop](https://github.com/imdaveho/tuitty/labels/interop)</kbd> os/arch support (bsds, arm, amd) 32/64-bit 
* <kbd>[performance](https://github.com/imdaveho/tuitty/labels/performance)</kbd> performance
* <kbd>[rust-stdlib](https://github.com/imdaveho/tuitty/labels/rust-stdlib)</kbd> migrations (Futures, Streams)
* <kbd>[ffi-ports](https://github.com/imdaveho/tuitty/labels/ffi-ports)</kbd> ports (Ruby, Python, NodeJS, etc)
* <kbd>[ergonomics](https://github.com/imdaveho/tuitty/labels/ergonomics)</kbd> ergonomics without being overly clever

### Versioning

We use [SemVer](http://semver.org/)_(-ish)_ for versioning. For the versions available, see the _TBD_ <!--[tags on this repository](https://github.com/your/project/tags).-->

### Authors

* **Dave Ho** - *Creator and project maintainer* - [imdaveho](https://github.com/imdaveho)

<!-- See also the list of [contributors](https://github.com/your/project/contributors) who participated in this project. -->

### License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details

### Closing Shoutouts :clap: 
> _nanos gigantum humeris insidentes_

**Many thanks** to the authors and projects below for various implementations that have inspired this project.

* [Termion](https://gitlab.redox-os.org/redox-os/termion)
* [Crossterm (TimonPost)](https://github.com/crossterm-rs/crossterm)
* [Termbox-go (nsf)](https://github.com/nsf/termbox-go)
* [Asciimatics (peterbrittain)](https://github.com/peterbrittain/asciimatics)
* [Vorpal (dthree)](https://github.com/dthree/vorpal)
* [Tty Toolkit (piotrmurach](https://github.com/piotrmurach/tty)
* [Prompt-toolkit (jonathanslenders)](https://github.com/prompt-toolkit/python-prompt-toolkit)
* [Colorama (tartley)](https://github.com/tartley/colorama)
* [Blessings (erikrose)](https://github.com/erikrose/blessings)
