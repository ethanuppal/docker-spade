# docker-spade

> [!TIP]
> You can use prebuilt images right now! See the [Spade docs](https://docs.spade-lang.org/installation.html) for details:
>
> ```shell
> docker run -it --rm ghcr.io/ethanuppal/spade-docker:latest
> ```

Build Docker images for [Spade](https://spade-lang.org).

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
  data-directory    Print data directory.
```

You can see fine-grained usage information by passing `--help` for each subcommand.

## Prerequisites

Please install [buildx](https://github.com/docker/buildx), the new build system
for Docker, if you don't have it already (the old one is deprecated). Also, make
sure to use the official Docker daemon. You can, however, use `podman` to
download and run the built images.

## Install

You can install from source as follows:

```
git clone https://github.com/ethanuppal/spade-docker
cd spade-docker

# to install locally
chmod u+x ./spade-docker

# to install to $PATH
cargo install --path .
```

You can also grab a version from <crates.io>:

```
cargo install spade-docker
```

Here, we're using the helper script, but you can similarly install
`spade-docker` system-wide with `cargo install --path .` (and then use it as
`spade-docker` without the `./` prefix).

## Local Usage

### Building an image

```
# build an image for x86_64 with the given Spade/swim versions
./spade-docker build --arch x86_64
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

Finally, you can `rm -rf` the cloned directory and `cargo uninstall spade-docker` if applicable.tjj 
