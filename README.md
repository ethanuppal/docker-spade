# docker-spade

Build Ubuntu images for [Spade](https://spade-lang.org).

```
$ spade-docker --help
Usage: spade-docker <command> [<args>]

Manage Spade docker images.

Options:
  --help, help      display usage information

Commands:
  build             Build a new image.
  list              List built images.
  clean             Prune built images.
```

You can see fine-grained usage information by passing `--help` for each subcommand.

## Local Install

```
git clone https://github.com/ethanuppal/spade-docker
cd spade-docker
chmod u+x ./spade-docker
```

Here, we're using the helper script, but you can similarly install
`spade-docker` system-wide with `cargo install --path .` (and then use it as
`spade-docker` without the `./` prefix).

## Local Usage

### Building an image

```
# build an image for aarch64 with the given Spade/swim versions
./spade-docker build \
    --arch aarch64 \
    --spade-rev 6ee46e1b35da629d15552c0672d5f470f9a94676 \
    --swim-rev 2a386a16b0fb3e2ba3a075e073279b25f97d6b56
```

Then, using the output hash, you can run the image in a terminal with `docker run --rm -it HASH`.

## Managing images

```
# list all images managed by the tool
./spade-docker list

# remove all images managed by the tool
./spade-docker clean
```
