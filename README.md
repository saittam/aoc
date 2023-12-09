Advent of Code solutions
========================

These are my solutions to the [Advent of Code](https://www.adventofcode.com)
programming puzzles. I've started to do them in 2018, using the opportunity 
to learn the Rust programming language. The puzzles are fun (thanks Eric Wastl
for making them!) and writing solutions in Rust has been enjoyable, so I have
stuck with it over the years and eventually went back to fill in solutions for
years 2015-2017 as well.

# House Rules

Here are some personal notes and house rules I've adopted:

* I do all the puzzles on my mobile phone. I've found this works best for me,
  since it allows me to make use of whatever unused 5 minutes in my day present
  themselves for working on the puzzles.

* Nowadays I'm using the repl.it Android app for writing AOC code on the phone,
  in the earlier years I relied on Dcoder. I'm thankful that these fine folks
  help make coding one the phone halfway bearable, but I've still found it to
  be a struggle in a number of ways. In particular, the apps could be more
  reliable - I've lost significant work multiple times over the years...
  
* Coding on the phone also forces me to keep things simple, and I consider this
  a plus. Each part of every puzzle gets its own file, and I don't use any
  elaborate project templates, libraries, etc. - they'd be more hassle than
  it's worth in the phone environment. My solutions are all independent files
  that you just build, provide input on stdin and the solution will be printed
  on stdout.

* Since managing dependencies in the phone environment is a hassle, I generally
  restrict myself to only the Rust standard library and avoid any other
  dependencies. If I need any fancy data structures that aren't in the rust
  standard library, I implement them myself. This is rarely necessary however.

* Due to having to type things out on the phone, the code doesn't look like
  what I would write professionally. While terse variable names hurt
  readability, the narrow columns-per-line limit actually helps clarity. Code
  isn't auto-formatted though, and white space and indentation can be somewhat
  messy/random.

* Partially to keep verbosity low, but mainly as a way of thinking, I find that
  I write my solutions in a rather functional style. Rust iterators are a great
  tool for expressing transformations on collections of numbers, which are the
  bread and butter of the AoC puzzles. The absence of itertools often hurts
  quite a bit though ;-)

* The phone environment also keeps me honest at finding a solution that
  completes in a few seconds at most. The puzzles are designed to allow this,
  and while brute forcing is sometimes feasible, I enjoy the algorithmic
  optimization side more than I would get out of optimizing compute resource
  management. Thus, there is no parallelization, multi-threading, etc. in my
  solutions.

* I have a personal rule to only look at the r/adventofcode subreddit *after*
  solving the day's puzzle. I'm strict about avoiding spoilers or any hints -
  part of the AoC challenge for me personally is to find the solution all by
  myself. This means that I sometimes get stuck on a puzzle for a while -
  that's OK :)

* I don't go for the leaderboard - not only is it incompatible with my daily
  schedule, but I personally find the most rewarding aspect of AoC is to ponder
  the structure of a problem posed in a puzzle and take the time to think
  through that instead of rushing to implement a solution. Similarly for coding
  - when I see multiple approaches, I like to think of their relative benefits
  before starting to write code. It also happens that I pick an unusual
  approach to try how the idea might look like in Rust.

All that said, of course there are cases where I have broken these rules. For
example, I haven't written an MD5 implementation and instead used a dependency
for that (some day I might go back and replace the dependency with a simple MD5
implementation, it's not that hard). Similarly, there have been a few puzzles
that I have deliberately decided to brute-force outside of the mobile phone
environment.

# My AOC experience

While at it, I'll use the opportunity to share some general thoughts on AoC.

I find the puzzles to be well made, and splitting each day into two consecutive
parts creates great reward/motivation mechanics. The simpler first part gives
me a quick affirmation, and the twists / changes in the second parts are often
enough interesting. Plus, I can't help speculate what the second part will
bring while still working on the first. Sometimes I guess correctly and are
well prepared, sometimes I have no idea, sometimes I'm just wrong, and then
there are days where part 2 isn't really harder than part 1. The fun is that
you never know in advance which kind you're dealing with :)

An ingenious puzzle mechanic that I want to comment is made possible by the
assembly language interpretation style puzzles, in particular the series of
intcode puzzles in AoC 2019. Puzzles that define hypothetical assembly
languages and ask to run a program given as the puzzle input are a common theme
in AoC, but 2019 took this a step further to create interactivity: The programs
are then taking the role of a black-box element that you need to interact with
as part of solving the puzzle. That's adding so much opportunity to puzzle
design! My favorite puzzle in this category and within all of AoC is
"Springdroid Adventure" (day 21 of 2019), where the intcode program is itself
an interpreter and acts as the oracle for the fate of a robot, which is
controlled by a program that you create and give to the intcode program!
Unfortunately, we no longer see these elaborate puzzle mechanics in more recent
years, apparently due to the desire to keep different days' puzzles independent
of each other, and a practical limit of much conceptual and practical
complexity can go into a single day.

On the other end of the spectrum, the puzzles that I find somewhat unsatisfying
are the ones that require finding hidden structure in the input. These
sometimes (but not exclusively) show up as language interpretation themed
puzzles as well, where the key to the solution is to reverse-engineer the
program to understand what it does at a higher level. The main thing I dislike
with these is that it is often awkward to write a solution program that
extracts the hidden structure and works for "arbitrary" inputs. While there are
different inputs for AoC participants, I do believe the inputs for these days
are merely parameterized differently, but in theory the there could be entirely
different structure hidden in different inputs. Separately, it feels more
rewarding to me to understand and exploit structure that's inherent to the
problem, and not just a feature of the particular problem instance on is given.

Finally, one thing that impresses me every year is how diverse the community of
AoC participants is, as evident by the interaction on the r/adventofcode
subreddit. I'm not actively engaging, but like to lurk and see how other people
approach the puzzles, as well as their unique approaches, solutions,
challenges, etc.

# State of this repository

Part due to the mobile phone environment and part due to laziness, this
repository isn't entirely complete. Some days are missing for various reasons
(e.g. when I did the puzzle using pen and paper), and other days aren't
compiling (some due to Rust version incompatibilities or because I lost the
actual version of the code that produced the solution). I figured it still
makes sense to centralize all the AoC code I have in a single repository for
future reference. Perhaps I will go back and fix up the gaps and issued at some
point.

