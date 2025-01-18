## Priority list

- [x] Create a meeting
- [x] Join a meeting
- For the Investor story (basic MVP)
    - [ ] Create a new meeting and join instantly
    - [ ] Create a new meeting for later
    - [ ] Join as a guest

- Customisability
    - [ ] Create a system package that abstracts all the functionality with minimal UI
    - [ ] Create a UI package based on the talk system package and use @dytesdk/ui components
    - [ ] On board @meenu and let her create a few more unique UIs

- Misc
    - [ ] Display past meetings on a dashboard page
    - [ ] Add participants by their FT username
    - [ ] Private meetings (only for fifthtry-staff)
        - [ ] Join as a participant (only authenticated user is allowed to join as himself)
        - [ ] allow selected guests (unique invite link) to join a private meeting

- Tandem like board (FT story)
    - [ ] list online ft employees
    - [ ] click on a ft employee to start an instant call with them
    ...



/talk/ -> dashboard page (show past meetings, create a new meeting)
/talk/:meeting_id -> A ftd page
    - make a call to talk.wasm to get current dyte token (stored in a cookie)
    - if got no token and logged in: make a call to talk.wasm to add the participant and re-render the same page (via redirect)
    - if got no token and not logged in: render a form asking for user's name
        (guest). Add them as a particpant and save the token and re-render the same
        page (via redirect)
    - if got a token: join the meeting by loading the dyte web component
/talk/new/ -> Create a new instant meeting
    - Show a page that has the link to copy to the clipboard
    - optionally a form to set the meeting name
