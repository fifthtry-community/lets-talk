# lets-talk

An online video chat application in fastn app ecosystem.

This repository contains four fastn packages:
- `lets-talk.fifthtry-community.com/` contains the source code of the official website of `lets-talk` app. This is available at https://lets-talk.fifthtry-community.com.
- `lets-talk.fifthtry.site/` is the app package that can be mounted in other fastn packages using [`-- fastn.app`](https://fastn.com/app/).
- `lets-talk-template.fifthtry.site/` is a template that shows how to use `lets-talk.fifthtry.site` package.

## Usage

See the `lets-talk-template.fifthtry.site` folder for an example of how to
configure an instance of the `lets-talk` app.

## Development

### Using [Nix](https://nixos.org/)

`fastn` binary should be available in your `$PATH`. This is not included in the
nix flake because we want to easily change the version of `fastn` on the fly.

```sh
git clone git@github.com:fifthtry-community/lets-talk.git
cd lets-talk
nix develop
update-template # only run this when lets-talk-template.fifthtry.site dependencies are modified
run-template
```

### Without Nix

You'll need to install the following dependencies:
- [fastn](https://fastn.com/install/)
- nodejs 20
- rust (See ./rust-toolchain.toml for version details)
- sqlite3

Then do:

```sh
git clone git@github.com:fifthtry-community/lets-talk.git
cd lets-talk
source scripts/auto.sh
update-template # only run this when lets-talk-template.fifthtry.site dependencies are modified
run-template
```


## UI Development

All lets-talk UIs are developed using [storybook](https://design-system.fifthtry.site/introduction/storybook/).

Run `fastn` service using `run-ui` alias: 

```shell
source scripts/auto.sh
update-ui  # only run this when modifying dependencies in FASTN.ftd
run-ui
```

Visit `http://127.0.0.1:8000/storybook/`. The port number may vary depending on the availability.
