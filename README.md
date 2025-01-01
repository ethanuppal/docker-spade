# docker-spade

Ubuntu image for [Spade](https://spade-lang.org).

## Local Install

```
git clone https://github.com/ethanuppal/spade-docker
cd spade-docker
```

## Local Usage

Here, we're using the helper script, but you can similarly install
`spade-docker` system-wide with `cargo install --path .` (and then use it as
`spade-docker` without the `./` prefix).

```
chmod u+x ./spade-docker

# build an image for aarch64 with the given Spade/swim versions
./spade-docker build \
    --arch aarch64 \
    --spade-rev 6ee46e1b35da629d15552c0672d5f470f9a94676 \
    --swim-rev 2a386a16b0fb3e2ba3a075e073279b25f97d6b56

# remove all images managed by the tool
./spade-docker clean
```
