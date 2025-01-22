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

- `TALK_SECURE_SESSIONS`: Use secure http cookies.
- `TALK_PRESET_PARTICIPANT`: Preset name that'll be used for participants (can only join and chat).
- `TALK_PRESET_HOST`: Preset name that'll be used for meeting hosts (can kick and end meetings for all).
- `LETS_TALK_REQUIRE_VERIFICATION`: (default false) Require email verification of account before they can create a new meeting.
- `LETS_TALK_ALLOWED_EMAIL_DOMAINS`: (default empty) Comma separated list of allowed email domains. If empty, no one can create meetings.


# UI Storybook

All lets-talk UIs are developed using storybook.  

Run `fastn` service using `run-ui` alias: 

```shell
source scripts/auto.sh
run-ui
```

Visit `http://127.0.0.1:8000/storybook/`.
