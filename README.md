# test_console_colors
A neat utility to test the legibility of basic terminal color themes.

Requires a terminal that supports ANSI escape sequences.

## Building
Written using Rust and built with Cargo, it's easy as 1-2-3. In the project
directory:

```
$ cargo build
$ cargo run -- --mode=cycle --output=ipsum
     Running `target/debug/test_console_colors --mode=cycle --output=ipsum`
Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nam tempor cursus liber
o, nec porta mauris auctor eget. Curabitur arcu mauris, egestas euismod neque no
n, semper interdum elit. Vestibulum tempor, nisi id blandit laoreet, velit arcu
laoreet mauris, vitae porta sem justo a purus. Ut eleifend suscipit dui, sit ame
t faucibus mauris placerat eu. Ut imperdiet massa nec justo fermentum lobortis.
Sed hendrerit et nibh. 

Done. How does it look?
$
```

Here's a sample with `mode` set to the default (`cycle`) and `output` set to `block`:

![Example output](http://aaronbolyard.com/images/11.png)

(And it's easy to see there needs to be more contrast on this sample theme for
a few of the colors.)

## Usage
Simple! There's two options that modify behavior. You can specify what text to
display by providing an argument to the `output` option. Similarly, you can
modify the color generation behavior by providing an argument to the `mode`
option.

### --output
All output is 64 "words" long.

* **ipsum:** A "Lorem Ipsum" filler text.
* **block:** Four distinct block characters ("░▒▓█", or U+2591, U+2592, U+2593, and U+2588) repeated.

### --mode
* **cycle:** Cycles between normal and bold/bright colors repetitively in a specific order: black, red, green, yellow, blue, magenta, cyan, white).
* **random:** Chooses a random color and normal/bold flag.
* **cycle-random:** Like `cycle` for the first 16 combinations, then like random for the remainder.

## License (MIT)
Copyright (c) 2015 Aaron Bolyard

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.