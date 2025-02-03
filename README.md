# lets-talk

An online video chat application in fastn app ecosystem.

This repository contains four fastn packages. `www` contains the source code
of the official website of `lets-talk` app: 
[lets-talk.fifthtry-community.com](https://lets-talk.fifthtry-community.com),
`app`, contains the source of the fastn package `lets-talk.fifthtry.site` that 
people are supposed to use. `app/.packages/lets-talk-system.fifthtry.site` 
contains the source code of fastn package `lets-talk-system.fifthtry.site` 
containing UI and other configurations used by the `lets-talk` app.

## Configuration

See the `lets-talk-template.fifthtry.site` folder for an example of how to
configure an instance of the `lets-talk` app.


## UI Storybook

All lets-talk UIs are developed using storybook.  

Run `fastn` service using `run-ui` alias: 

```shell
source scripts/auto.sh
update-ui  # only run this when modifying dependencies in FASTN.ftd
run-ui
```

Visit `http://127.0.0.1:8000/storybook/`.
