# A Short History of `~[c255]`

*Background on the notation behind ee-conio, and the fourteen years of ports
that led to it. The Rust crate is the fifth implementation of a pattern that
has been in continuous personal use since roughly 2012.*

Note: Text based on a conversation with Claude while it was looking at my
source trees.

## Origins: the 16-color years (~2012)

The pattern began in bash, in an era when 256-color support was genuinely
unreliable. macOS Terminal.app had only just gained 256-color support in Lion
(2011); Konsole was ahead of the curve; everything else was a gamble involving
`TERM=xterm-256color` and stale terminfo entries on remote machines.

The first generation wasn't a DSL at all — just a small vocabulary of shell
functions expanded inline by the host language:

```bash
printf "$(c 197)Must run as ROOT.$(x 0)\n"
```

`c` for foreground color, `x` for a raw SGR sequence, `X` for a raw CSI
sequence without the `m` terminator. No parser, no grammar — the shell's
command substitution did the composition. The vocabulary proved to be the
durable part: `c`/`C`, `x`/`X`, and later `#RRGGBB`/`$RRGGBB` have survived
every port since, unchanged.

Two properties of that original vocabulary turned out to matter for the next
decade and a half:

1. **It was sized right.** Small enough to memorize, terse enough to type
   inline without breaking the rhythm of a format string.
2. **It never had an opinion about terminal capabilities.** Because `x`/`X`
   are raw pass-throughs, the grammar absorbed 256-color, then truecolor,
   then anything else the terminal learned to do — without ever needing a
   revision. The cost of a wrong guess landed on the color values chosen in a
   given string, never on the notation.

## Generation 2: Lua, the first real DSL (~2014–2020)

The Lua port (part of a personal `ee5_*` utility library) was the first
version where the markup became a single parseable string rather than
host-language macro expansion:

```lua
cprintf("~[c255]%s ~[c197]%s~[x0]\n", label, error_message)
```

The implementation was deliberately minimal — a chain of `gsub` passes over
the format string — but two ideas appeared here that every later version
kept:

- **Markup lives in the template, never in the data.** `cformat` expands
  `~[...]` in the format string *before* `string.format` splices in
  arguments, so runtime data is never scanned for markup. Data cannot inject
  color codes. This started as an accident of implementation order and ended
  up as a design principle.
- **The strip dual.** `qformat` applies the same grammar in reverse, removing
  markup (and raw escapes) to produce plain text from the same template — a
  homemade `NO_COLOR` years before the convention had a name.

## Generation 3: Python, the maximalist (~2019, stable since ~2022)

The Python port grew into the richest runtime implementation — less a
formatting function than a small console I/O subsystem:

- A two-stage parser: capture `~[...]` contents, then apply an *ordered*
  table of token regexes (truecolor before 256-color, because `#c0ffee`
  contains something that looks like `c0`).
- Long-form aliases that exist in no other version: `~[fore 255]`,
  `~[back 0]`, `~[SGR 0]`, `~[CSI ...]`, `F:`/`B:` for truecolor.
- Output policy as keyword arguments: auto-reset after each print,
  per-call `ansi_off` plain mode, pluggable output sinks, newline and flush
  control.
- Multiprocessing locks around emission, DSL-rendered timestamp prefixes,
  colored exception and call-stack formatting.
- A win32 shim — `SetConsoleMode(..., ENABLE_VIRTUAL_TERMINAL_PROCESSING)` —
  from the era when Windows had VT processing but classic conhost didn't
  enable it.

The Python version has been essentially unchanged for years. That is by
choice, not neglect: what works doesn't need many changes. Its stability is
the strongest evidence that the notation, not any implementation, is the
actual artifact.

## The sibling ports: PowerShell

A PowerShell version exists in the same spirit as the original bash one —
macro expansion of the shared vocabulary rather than a parsed DSL. It only
became worth writing once Windows had a terminal worthy of it: the arrival of
ConPTY (2018) and Windows Terminal (2019) — truecolor from day one,
GPU-rendered text, built in the open — turned the platform that couldn't run
this pattern at all for its first several years into a first-class target.

## Generation 4 (Rust, 2026–): keep the language, discard the machine

Looking back across the ports, the trajectory is a pendulum:

| Generation | Binding mechanism | When markup becomes escapes |
|---|---|---|
| bash / PowerShell | host macro expansion | at interpolation, every echo |
| Lua | runtime rewriting (gsub chain) | every `cformat` call |
| Python | runtime rewriting (ordered regex table) | every call, with policy kwargs |
| **Rust (ee-conio)** | **proc-macro rewriting of literals** | **once, at compile time** |

ee-conio sheds nearly all of the Python generation's runtime surface —
aliases, policy kwargs, locks, runtime strip — and keeps only the notation,
moving expansion to compile time. Two long-standing conventions became
structural guarantees in the process:

- *Markup in the template, never the data* is now enforced by the compiler:
  proc macros can only ever see literals.
- Unknown tokens, which the script generations passed through or leaked
  silently at runtime, can now be rejected at compile time with a real
  diagnostic.

The vocabulary, meanwhile, is still growing the way it always has —
additively. Named colors (a ~1300-entry table that costs nothing because it
never ships in the binary), `~[cls]`, cursor addressing via the same `X`
convention the bash version used, and line-drawing mnemonics are the first
tokens that commit to specific terminal capabilities — a luxury of 2026
terminals being boringly uniform compared to the ones this notation was born
on.

What was traded away is runtime flexibility: escapes are baked into string
literals, so colors cannot be turned off at run time. That is a deliberate
inversion of the Lua/Python philosophy — they treated the markup as data that
could be interpreted two ways; ee-conio treats it as notation that ceases to
exist after compilation.

## Why it lasted

Fourteen years and five languages later, the reasons this pattern survived
are mostly reasons it was allowed to stay small:

- The grammar never enumerated what terminals could do, so it never went out
  of date.
- The template/data separation made it safe by convention long before it was
  safe by construction.
- Each port kept the strings themselves portable — a message written for the
  Lua tools works verbatim under `ctransform!`.
- And when an implementation worked, it was left alone.

The Rust version is the first one where the part that historically needed
maintenance — the expansion machinery — is the compiler's problem.

