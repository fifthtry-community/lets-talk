# lets-talk

An online video chat application in fastn app ecosystem.

This repository contains four fastn packages. `www` contains the source code
of the official website of `lets-talk` app: 
[lets-talk.fifthtry-community.com](https://lets-talk.fifthtry-community.com),
`app`, contains the source of the fastn package `lets-talk.fifthtry.site` that 
people are supposed to use. `app/.packages/lets-talk-ui.fifthtry.site` contains 
the source code of fastn package `lets-talk-ui.fifthtry.site` containing UI 
elements used by the `lets-talk` app.



# Required environment variables

- `DYTE_ORG_ID`
- `DYTE_API_KEY`
- `TALK_SECURE_SESSIONS`: use secure http cookies


# UI Storybook

All lets-talk UIs are developed using storybook.  

Run `fastn` service using `run-ui` alias: 

```shell
source scripts/auto.sh
run-ui
```

Visit `http://127.0.0.1:8000/storybook/`.
