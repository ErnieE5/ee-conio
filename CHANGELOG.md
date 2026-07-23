# Changelog

All four crates (`ee-conio`, `ee-conio-engine`, `ee-conio-macro`,
`ee-conio-parse`) share a version and are released together.

This project is in __alpha__: the API may change in any release.

## 0.1.0-alpha.6 — unreleased

### Changed
- Replaced two `todo!()` and a `panic!` in `ee-conio-parse` with `expect`
  calls that state the invariant. All three sat on regex capture groups that
  always participate in a successful match, so none was reachable, but
  `todo!()` reads as unfinished work in a published crate. No behavior change.
- Fixed a typo in the message that went with the `panic!` ("expresion").

## 0.1.0-alpha.5

### Added
- `LICENSE-MIT` and `LICENSE-APACHE` are now present and are packaged with
  every crate. Earlier releases declared `license = "MIT OR Apache-2.0"` in
  the manifest but shipped no license text at all.
- `rust-version` is declared, and was measured rather than assumed:
  `ee-conio-engine` and `ee-conio-parse` need __1.85__ (the edition 2024
  floor); `ee-conio-macro` and `ee-conio` need __1.89__, because
  `rustc-literal-escaper` requires it.
- GitHub Actions CI: tests on Linux and Windows, `clippy -D warnings`, both
  MSRV floors, and a nightly documentation build with warnings denied.
- `esc!` is re-exported from `ee-conio`. It was listed in the escape
  primitives table in the documentation but had never been exported.
- This changelog.

### Fixed
- `ParseError`'s `Display` printed a fixed string and ignored the message it
  was carrying, so `println!("{e}")` never showed the actual problem.
- Error spans were computed and then thrown away. `transform_all` replaced an
  inner error's offsets with the span of the whole token, discarding the
  precision that `chardig` and `r_g_b_from_string` had worked out. Spans are
  now shifted rather than replaced, and the leaves that returned a
  placeholder `0, 0` report real ranges. On nightly, where `subspan`
  resolves, `~[c227 c999]` now underlines `999` instead of `c999`.
- The RGB patterns matched `#` or `$` followed by any six characters, so
  `#'abcd'` matched as hex as well as a quoted name and only the order of the
  pattern list kept it resolving correctly. They now require six hex digits.
- Malformed hex such as `#ZZZZZZ` is reported as such instead of as an
  unknown mnemonic.
- Three documentation defects: an R Markdown code block attribute that
  rustdoc rejects (the "will not compile" example is now a real
  `compile_fail` doctest), a broken `[ee_conio]` intra-doc link, and the
  unresolvable `[esc!]` link described above.
- Typo in the out of range message ("is is not").

### Known limitations
- Colors cannot be disabled at run time. `NO_COLOR`, `CLICOLOR`,
  `--color=never` and terminal detection are not implemented. See the crate
  documentation.

## 0.1.0-alpha.4

### Fixed
- Initial work on `ParseError` messages and spans (superseded and completed
  in alpha.5).

## 0.1.0-alpha.3

### Changed — BREAKING for `ee-conio-engine`
- The `~[...]` parser and the keyword and named colour tables moved out of
  `ee-conio-engine` into a new crate, __`ee-conio-parse`__.

  `ee-conio-engine` no longer exports `find_replacement_patterns`,
  `transform_all`, `transform_one`, `get_keyword`, `r_g_b_from_string`,
  `ParseError`, `get_named_foreground_escape`, `get_named_background_escape`,
  `named_color_iter` or `match_name_iter`. All of them are now in
  `ee-conio-parse` with unchanged behaviour, so the fix is to add that crate
  and change the import path.

  Because the dependency specifications are caret ranges, `cargo update` can
  move you from alpha.2 to a later alpha automatically. If you depended on
  `ee-conio-engine` __directly__ for any of the above, that update will not
  compile until you switch to `ee-conio-parse`.

  Users of `ee-conio` itself are unaffected — the facade's public surface did
  not change.

  The reason for the split: `ee-conio-engine` owned both the escape builders
  and the parser, which put `regex` in the dependency graph of every
  downstream crate even though nothing the facade re-exports uses it.
  `ee-conio-parse` now hangs only off the proc-macro crate, so `regex` and
  the roughly 1300 entry colour table are compile-time only and never reach a
  user's binary. `ee-conio-engine` has no dependencies at all.

### Added
- `ee-conio-parse`, published for the first time.

## 0.1.0-alpha.2 and earlier

Released before this changelog was kept. See the commit history.
