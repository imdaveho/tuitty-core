# tuitty
A cross-platform, interoperable, simplfied terminal library that is meant to be wrapped by multiple languages.

![tuitty-banner](https://user-images.githubusercontent.com/13990019/68438603-a6972e00-0192-11ea-8fc9-ff334ee79432.png)

> **tui • tty** \ˈtwē-dē \ *n.* - **t**ext-based **u**ser **i**nterface library for **t**ele**ty**pe writers (aka. **terminals**)

***NOTE: (Nov. 8, 2019) - This library is still in alpha stage and the API is still in flux. However, the core concepts and goals outlined below will remain fairly stable. [Contributions](#contributing) are most welcome! :smile:***

## Table of Contents
* [Features](#sparkles-features)
* [Rationale](#thought_balloon-rationale)
* [Definitions](#notebook_with_decorative_cover-definitions)
* [Getting Started](#zap-getting-started)
  * [Dispatcher](#dispatcher)
  * [Event Handle](#event-handle)
* [Aspirations, but not Guarantees](#crystal_ball-aspirations-but-not-guarantees)
* [Contributing](#contributing)
* [Versioning](#versioning)
* [Authors](#authors)
* [License](#license)
* [Shoutouts](#closing-shoutouts-clap)

### :sparkles: Features
[(Back to top)](#table-of-contents)

* Cross-platform (Linux, Mac, Windows)
* Focused (read: _small_) API footprint - unified, consistent capabilities across terminals
  - avoid leaky abstractions that force you to think about what may or may not work
  - prefer to keep dependencies limited (*Unix*: __libc__, *Windows*: __winapi__) and avoid including the kitchen sink
* Thread-safe (guarantees provided by Rust's Send + Sync traits)
* **Cursor navigation** - _eg. goto(col, row), move up/down/left/right_
* **Screen manipulations** - _eg. resize, clear, print, enter/leave alternate screen_
* **Styling output** - _eg. fg and bg colors, bold, dim, underline_
* **Terminal settings** - _eg. raw/cooked, hide/show cursor, mouse on/off_
* **User input handling** - _eg. keyboard/mouse events_
* Minimal memory `unsafe` code: only OS specific calls and FFI which follows the [Rust FFI Nomicon](http://jakegoulding.com/rust-ffi-omnibus/) very closely

### :thought_balloon: Rationale
[(Back to top)](#table-of-contents)

* **Why not use _curses_?**
  <details>
   <summary>Show response</summary>
    <br/>
   <blockquote>
   While <em>[n/pd]curses</em> is widely used and wrapped, there is also plenty issues regarding them: wide character support, cross-platform support, <a href="https://pypi.org/project/blessings/#before-and-after">C-style/low-level imports</a> that reduce clarity, etc.
   </blockquote>
  </details>


* **Why not use _[blessings](https://github.com/erikrose/blessings) (Python)_, _[tty-tk](https://github.com/piotrmurach/tty)_ (Ruby), _[terminal-kit](https://github.com/cronvel/terminal-kit)_ (Node), or _[insert project](#rationale)_ (_insert language_)?**
  <details>
   <summary>Show response</summary>
   <br/>
   <blockquote>
   <p>As you can see, there is already a proliferation of various implementations of terminal libraries...and yes I'm aware of the irony that this project is <a href="https://xkcd.com/927/">+:one: </a> to the list of implementations out there.</p>
   <p>However, unlike other attempts, what this project intends to do is to create a unifying API across languages that eliminates the need to repeat yourself. This is actually very similar to how <a href="https://asdf-vm.com/#/?id=ballad-of-asdf">asdf-vm</a> addressed the proliferation of "version managers" like <code>rbenv</code>, <code>gvm</code>, <code>nvm</code>, and <code>pyenv</code>. By creating something unifying and extensible, users won't have to re-discover and re-learn a new API every time they switch programming languages.</p>
   <p>Additionally, many of the implementations out there do not provide cross-platform support (mainly Windows Console), which I'm specifically targeting with this project.</p>
   </blockquote>
  </details>
  
* **Why the command line? Why cross-platform? Why, why, why?!**
  <details>
   <summary>Show response</summary>
   <br/>
   <blockquote>
   At the end of the day, many development workflows begin and end with a terminal prompt. I wanted to learn and better understand this critical component of a software engineer's journey. Consequently, this process has gotten me familiar with systems programming languages (Rust, Go, C, and Nim), low-level OS syscalls, the Windows Console API, and countless other intangibles that have made me a more well-rounded individual.
   </blockquote>
  </details>

### :notebook_with_decorative_cover: Definitions
[(Back to top)](#table-of-contents)

**Cross-platform**

  <details>
   <summary>Expand description</summary>
   <br/>
   <blockquote><ul>
    <li>Needs to consistently work on MacOS, Linux, and Windows
     <ul><li>BSDs and others would be secondary</li></ul>
    </li>
    <br/>
    <li>Needs to work on these architectures:
     <ul>
      <li>ARM - 32/64-bit</li>
      <li>Intel - 32/64-bit</li>
      <li>AMD - 32/64-bit</li>
     </ul></li>
   </ul></blockquote>
  </details>

**Interoperable**

  <details>
    <summary>Expand description</summary>
    <br/>
    <blockquote>
    <ul><li>Needs to be portable to multiple languages (ones that have an FFI with C)
      <ul><li>C has too many :hourglass_flowing_sand::bomb::boom: so such interoperability is provided by Rust (maybe Nim)</li></ul>
    </li></ul>
    </blockquote>
  </details>

**Simplified**

  <details>
    <summary>Expand description</summary>
    <br/>
    <blockquote><ul>
     <li>Basic functionality scoped to the below:
      <ul>
       <li>Cursor actions (motion)</li>
       <li>Screen actions (printing/clearing)</li>
       <li>Output actions (styling)</li>
       <li>Term mode actions (raw/cooked)</li>
       <li>Input event handling</li>
      </ul>
     </li>
     <br/>
     <li>Implemented with as little "in the middle" as possible
      <ul><li>Tight scoping allows us to focus on specific elements to optimize performance rather than peanut-buttering across too many concerns</li></ul>
     </li>
     <br/>
     <li>Being clear > being clever
      <ul>
       <li>Rust actually provides great options for abstractions (eg. Traits, macros) but these should be carefully considered over a more straight-forward method—even if they are more idiomatic Rust. Often, traits and macros make code less understandable for newcomers as they can be/get quite "magical".</li>
       <li>The analogy that comes to mind is that, for the longest time, Go(lang) did not want to provide generics because the feeling was that they reduced readability and made the language more complex. Instead, the tradeoff made was that <em>some</em> repetition was more beneficial towards maintainable code than bluntly trying to be <em>DRY</em>. Likewise, to keep things simplified, I'd rather repeat things that make what is going on obvious and less opaque.</li>
      </ul>
     </li>
    </ul></blockquote>
  </details>

### :zap: Getting Started
[(Back to top)](#table-of-contents)

#### API Design
**tuitty**'s architectural design attempts to mirror reality. There are actually two (2) feedback loops happening when an application begins: 

1. The "outer loop": a User receives visual cues from the terminal and, in response, does things that emits input events (eg. pressing keys on a keyboard), which in turn does stuff to the terminal, and 

2. The "inner loop": the Application receives a signal or request, processes or fetches application state/data accordingly, updates the application state, and performs operations to the view, which causes the "stuff" to be done to the view that provides the visual cue to the User.

<details>
 <summary>Bear in mind, it's <em>just</em> a loop!</summary>
  <br/>
   The mental model to bear in mind is similar to the <a href="https://facebook.github.io/flux/docs/in-depth-overview#structure-and-data-flow">Flux</a> pattern for React.js popularized by Facebook.
  <br/>
  <!--<img src="https://user-images.githubusercontent.com/13990019/68451356-719fd100-01bc-11ea-8eb2-139057bf5be7.png" alt="tuitty-flow" width="640"/>-->
  <img src="https://facebook.github.io/flux/img/overview/flux-simple-f8-diagram-1300w.png" alt="Unidirectional data flow in Flux" width="640"/>
</details>

<details>
 <summary>Phase 1: Receiving an Input Event from the User</summary>
 <p>The <code>Dispatcher</code> replicates the parsed <code>InputEvent</code> and sends it to each listening <code>Event Handle</code>.</p>
 <img src="https://user-images.githubusercontent.com/13990019/68457844-4377bc80-01cf-11ea-92f5-e3367aff0444.png" alt="input-flow" width="640" />
</details>

<details>
 <summary>Phase 2: App Requests some internal state</summary>
 <p>For example, a <code>Signal</code> was received to get the character underneath the cursor. This requires a <code>Request</code> made to the <code>Dispatcher</code> to fetch the cursor position and the character at the corresponding location in the internal screen buffer.</p>
 <img src="https://user-images.githubusercontent.com/13990019/68459195-afa7ef80-01d2-11ea-8c5e-2e2d28ca6ecb.png" alt="state-flow" width="640" />
</details>

<details>
 <summary>Phase 3: App Signals an appropriate Action to be taken</summary>
 <p>Perhaps, you want to take the character at position and print it somewhere else on the screen, like a <code>copy + paste</code> operation.</p>

 <img src="https://user-images.githubusercontent.com/13990019/68459513-9d7a8100-01d3-11ea-8363-b8ff5f9c9e0a.png" alt="signal-flow" width="640" />

 <p>After the terminal updates, the User will receive that visual cue and provide more inputs for the cycle to start over again.</p>
</details>

<details>
 <summary><b>Is this really a big deal?</b></summary>
 
<br/>
<p>These separate diagrams were meant to help build a mental model regarding how the internals of the library work. It is helpful to understand that the <code>Dispatcher</code> is responsible for sending and receiving <code>Signal</code> or <code>Request</code> messages that either does stuff (signal actions) or fetches stuff (request app state). This uses channels under the hood.</p>

<p><img src="https://github.com/day8/re-frame/raw/master/images/Readme/6dominoes.png?raw=true" align="right" width="360"/></p>

<p>This is important, because on Unix systems, in order to parse user input, you would have to read <code>stdin</code>. But that would be a blocking call. If you wanted to run things concurrently (eg. autocomplete, syntax checking, etc), you would have to read things asynchronously through a spawned thread. It would be impractical to spawn a thread every time you wanted a concurrent process to read from <code>stdin</code>. Also, why would you need more than a single process reading and parsing from <code>stdin</code>? Instead of a thread, this implementation creates a new channel that receives <code>InputEvent</code>s from a single reader of <code>stdin</code> that is within the <code>Dispatcher</code>.</p>

<p>Similarly, if you wanted to take actions on the terminal, in the previous paradigm, terminal actions were methods with an object that also held some mutable state (eg. screen buffers, multiple screen contexts, etc). It wasn't clear how that would cross the FFI boundary when attempting multi-threaded or async/await event loops in other languages. Passing a mutable <code>Box&lt;T&gt;</code> (heap allocated chunk of memory) seemed like a bad idea. However, with this pattern in a similar manner, multiple entities can send <code>Signal</code>s and make <code>Request</code>s to the <code>Dispatcher</code> to be handled safely.</p>
 
<p>Like I mentioned previously, this is not a pattern that was invented for this particular library. Rather, this pattern pulled inspiration from reactive programming (Rx.js), the actor model / concurrency via message passing (Kafka, Erlang), and web frameworks like <a href="https://guide.elm-lang.org/architecture/">Elm</a>, React.js (aforementioned <a href="https://facebook.github.io/flux/docs/in-depth-overview#structure-and-data-flow">Flux</a>), and <a href="https://github.com/day8/re-frame/blob/master/README.md#domino-4---query">re-frame</a>. Actually, the documentation for <b>re-frame</b> has a similar diagram: (see right). The relevant parts are mainly 1-5 since the web stuff is irrelevant here. But notice how similar the flows are to each other. It has been well-documented and proven how these patterns reduce compexity and errors and improve maintainability and speed of development.</p>
 </details>


#### Dispatcher
[(Back to top)](#table-of-contents)

#### Event Handle
[(Back to top)](#table-of-contents)

#### Tested Terminals
[(Back to top)](#table-of-contents)
* Windows 10 - Cmd.exe (legacy and  modern modes)
* Windows 10 - PowerShell (legacy and modern modes)
* Windows 10 - git-bash (w/ [winpty](https://stackoverflow.com/questions/48199794/winpty-and-git-bash))
* Ubuntu 17.04 - gnome-terminal

### :crystal_ball: Aspirations, but not Guarantees
[(Back to top)](#table-of-contents)

<details>
  <summary>
  Expand description
  </summary>
  <br/>
  <ul>
   <li>High performance (can't expect it all to be there as a v1)</li>
   <li>Work flawlessly on <b>all</b> platforms, <b>all</b> architectures, etc. (this is non-trivial)</li>
   <li>Cover <b>all</b> world languages and keyboard layouts (unicode is hard)</li>
   <li>Match idomatic paradigms across programming languages (eager to adopt the best from each)</li>
   <li>Have feature X from this other library Y (eager to evaluate and learn from)</li>
   <li>Completeness (not always is the terminal the best tool for the job; we won't force a square peg into a round hole)</li>
 </ul>
</details>

### Contributing
[(Back to top)](#table-of-contents)

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
[(Back to top)](#table-of-contents)

We use [SemVer](http://semver.org/)_(-ish)_ for versioning. For the versions available, see the _TBD_ <!--[tags on this repository](https://github.com/your/project/tags).-->

### Authors
[(Back to top)](#table-of-contents)

* **imdaveho** - *Creator and project maintainer* ([profile](https://github.com/imdaveho))

<!-- See also the list of [contributors](https://github.com/your/project/contributors) who participated in this project. -->

### License
[(Back to top)](#table-of-contents)

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details

### Closing Shoutouts :clap: 
[(Back to top)](#table-of-contents)

> _nanos gigantum humeris insidentes_

**Many thanks** to the authors and projects below for various implementations that have inspired this project.

* [Termion](https://gitlab.redox-os.org/redox-os/termion)
* [Crossterm (TimonPost)](https://github.com/crossterm-rs/crossterm)
* [Termbox-go (nsf)](https://github.com/nsf/termbox-go)
* [Asciimatics (peterbrittain)](https://github.com/peterbrittain/asciimatics)
* [Vorpal (dthree)](https://github.com/dthree/vorpal)
* [Tty Toolkit (piotrmurach)](https://github.com/piotrmurach/tty)
* [Prompt-toolkit (jonathanslenders)](https://github.com/prompt-toolkit/python-prompt-toolkit)
* [Colorama (tartley)](https://github.com/tartley/colorama)
* [Blessings (erikrose)](https://github.com/erikrose/blessings)
