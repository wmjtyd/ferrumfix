# Fasters

Fasters is a suite of tools to work with FIX standards. It is **not**, at the time of writing, production ready. I currently can't afford to put in the hours and make it robust and fully standard-compliant. Rigor is nevertheless a core goal of the project and it might serve as a useful foundation for others' work.

This is the planned feature roadmap:

**Core features:**

- [X] Code generation (Rust, possibly others).
- [X] FIX 4.2.
- [X] FIX 4.4.
- [X] FIX 5.0 Service Pack 2.

**Encodings:**

- [ ] Tagvalue (classic FIX).
- [ ] FIXML.
- [ ] Simple Binary Encoding (SBE).
- [ ] Google Protocol Buffers (GPB).
- [ ] JavaScript Object Notation (JSON).
- [ ] Abstract Syntax Notation (ASN.1).
- [ ] FIX Adapted for STreaming (FAST).

**Session protocols:**

- [ ] FIX4.
- [ ] FIXT.
- [ ] FIXP.
- [ ] SOFH.
- [ ] FIXS.

As dictated by [SemVer 2.0](https://semver.org/), I will bump the major version to `1` once I've settled on sensible APIs.

Fasters is intended to be an all-in-one solution for everything concerning FIX & FAST data handling. You can use it as a build-time dependency to generate message codecs or run it live for reflection capabilities.

- [FIXwiki](http://fixwiki.org/fixwiki/FIXwiki)
- [FIX @ Wikipedia](https://it.wikipedia.org/wiki/Financial_Information_eXchange_Protocol)
- [FAST @ Wikipedia](https://en.wikipedia.org/wiki/FAST_protocol)
- [FIX Protocol, Ltd's official website](https://www.fixtrading.org)
- [ValidFIX: FIX parser online](http://www.validfix.com/fix-analyzer.html)
- [OnixS FIX dictionary browser](https://www.onixs.biz/fix-dictionary.html)

---

#### License

Licensed under either of <a href="LICENSE-APACHE">Apache License 2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.