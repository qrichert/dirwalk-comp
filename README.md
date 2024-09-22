# Dir Walking Benchmarks

## macOS

<https://users.rust-lang.org/t/whats-the-fastest-way-to-read-a-lot-of-files/39743/7>

Looks like macOS can't parallelize file traversal?
Maybe that's why walkdir not slower?

```console
$ cargo run --release
Carbon:   Found 2019 files in 2.203670657
CarbonP:  Found 2019 files in 0.24633502
WalkDir:  Found 2019 files in 0.187563985
WalkDirP: Found 2019 files in 0.203403979
Ignore:   Found 1934 files in 0.272850863
IgnoreP:  Found 1934 files in 0.089098544
JWalk:    Found 2023 files in 0.32670597
fd:       Found 1934 files in 0.126591406
find:     Found 2019 files in 3.167510889
```

Warm cache (looks like the clearing doesn't work):

```console
$ cargo run --release
Carbon:   Found 2019 files in 0.587767921
CarbonP:  Found 2019 files in 0.255289296
WalkDir:  Found 2019 files in 0.188038603
WalkDirP: Found 2019 files in 0.205131933
Ignore:   Found 1934 files in 0.194090154
IgnoreP:  Found 1934 files in 0.101232262
JWalk:    Found 2023 files in 0.439331888
fd:       Found 1934 files in 0.09947689
find:     Found 2019 files in 2.524565515
```