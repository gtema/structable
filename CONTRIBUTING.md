# Contributing

Thank you so much for having interesting in contributing to the project.

## Issues reporting

- crashes, panics and such should be reported there directly. Code should not
  crash, it is a bug.

## Rust design decisions:

- If a function needs to take ownership of the data, pass by value

- If a function only needs to read the data, pass a reference (borrow)

- If a function needs to change the data, pass a mutable reference.

## Linters and co

- pre-commit is present in the repository to help ensuring linters will not
  complain later. just issue `pre-commit install` in the repository root to
  enable it.

- spell checking is performed using 'crate-ci/typos' and is a mandatory check.

- doctstrings in generated files are produced from OpenStack services code.
  There is no point fixing them here and instead syntax or typos should be
  fixed upstream. The only exception is that during code generation spelling
  issues are automatically addressed by `typos -w`.

## Releasing

`release-plz` validates challenges and prepares PR to cut a release proposing
the corresponding version. Merging PR will trigger `release-plz` to create
new version, tag it, perform Rust release to <crates.io>.

Since every crate in the workspace is currently having independent versioning
it is not trivial to rely on "latest" release. This is explicitly harming
publishing binary artifacts. As of now every crate is released in a separate
tag with binary artifacts included in the corresponding release. Maybe combined
tag (i.e. in a form YYYY-MM-DD) may be produced repackaging binaries, but it is
not the case right now.
