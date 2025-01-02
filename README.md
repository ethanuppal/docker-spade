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

## Prerequisites

Please install [buildx](https://github.com/docker/buildx), the new build system
for Docker (the old one is deprecated). Also, make sure to use the official
Docker daemon.

## Install

```
git clone https://github.com/ethanuppal/spade-docker
cd spade-docker

# to install locally
chmod u+x ./spade-docker

# to install to $PATH
cargo install --path .
```

Here, we're using the helper script, but you can similarly install
`spade-docker` system-wide with `cargo install --path .` (and then use it as
`spade-docker` without the `./` prefix).

## Local Usage

### Building an image

```
# build an image for x86_64 with the given Spade/swim versions
./spade-docker build \
    --arch x86_64 \
    --spade-rev main \
    --swim-rev main
```

Then, using the output hash, you can run the image in a terminal with `docker run --rm -it HASH`.

## Managing images

```
# list all images managed by the tool
./spade-docker list

# remove all images managed by the tool
./spade-docker clean
```

## Uninstall

First, prune all images managed by the tool:
```
./spade-docker clean
```

Then, remove the directory used by the tool to store data:
```
rm -rf "$(./spade-docker data-directory 2>/dev/null)"
```
