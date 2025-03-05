import DyteClient from "@dytesdk/web-core";
import { defineCustomElements as defineDyteCustomElements } from "@dytesdk/ui-kit/loader";

class Talk extends HTMLElement {
    data;

    constructor() {
        super();
        // this.style.width = "100%";
        // this.style.height = "100%";
        this.classList.add('fastn-ignore-global-keyboard'); // TODO(siddhantk232): learn what this means?
    }

    async connectedCallback() {
        const data = window.ftd.component_data(this);
        this.data = data;
        window.mdata = data;
        const mid = this.data.mid.get();

        const endpoint_url = ftd.app_url_ex("/talk/session/", "lets-talk");
        const req_url = `${endpoint_url}?meeting-id=${mid}`;

        const res = await fetch(req_url).then(r => r.json());

        if (res.redirect) {
            window.location.href = res.redirect;
        }

        const token = res.token;

        console.log("Meeting ID: ", mid);
        console.log("Token: ", token);

        if (!token) {
            throw new Error("Token not provided. Quitting");
        }

        const meeting = await DyteClient.init({
            authToken: token,
        });

        console.info("initialized meeting: ", meeting);

        window.meeting = meeting;

        this.data.meeting_initialized.set(true)

        try {
            document.querySelector("dyte-meeting").meeting = window.meeting;
        } catch (e) {
            console.info("dyte-meeting not found. Ignoring");
        }

        window.meeting.self.on("*", (event, ...args) => {
            if (event === "mediaScoreUpdate") return;

            console.info("self update: ", event, args);
            this.updateSelf();

            if (event == "roomJoined") {
                this.data.inside_meeting.set(true);
                this.data.meeting_title.set(window.meeting.meta.meetingTitle);

                try {
                    const id = window.meeting.self.id;
                    console.info(`Setting video stream for self#${id}`);
                    document.querySelector(`video[id='${id}']`).srcObject = new MediaStream([meeting.self.videoTrack]);
                } catch (e) {
                    console.error("Error setting video stream: ", e);
                }
            }
            if (event === "roomLeft") {
                this.data.inside_meeting.set(false);
            }
        });

        window.meeting.participants.joined.on("*", (event, ...args) => {
            if (event === "mediaScoreUpdate") return;

            console.info("participant update: ", event, args);
            this.updateParticipantsList();
            this.refreshParticipantVideoStreams();
        });

        // TODO: listen for pinned and waiting participants and update their lists
    }

    /** Updates this.data.self */
    updateSelf() {
        const self = {
            id: window.meeting.self.id,
            name: window.meeting.self.name,
            mic: window.meeting.self.audioEnabled,
            video: window.meeting.self.videoEnabled,
            screen: window.meeting.self.screenShareEnabled,
        }

        this.data.self.set(fastn.recordInstance(self));
    }

    /** Recreate this.data.participants list */
    // TODO(siddhantk232): This can be optimized
    updateParticipantsList() {
        this.data.participants.clearAll();
        /** @type {DyteClient} */
        const meeting = window.meeting;
        for (const [_id, p] of meeting.participants.joined) {
            this.data.participants.push(fastn.recordInstance({
                id: p.id,
                name: p.name,
                mic: p.audioEnabled,
                video: p.videoEnabled,
                screen: p.screenShareEnabled,
            }))
        }
    }

    refreshParticipantVideoStreams() {
        /** @type {DyteClient} */
        const meeting = window.meeting;
        for (const p of meeting.participants.joined.toArray()) {
            try {
                console.info(`Setting video stream for participant#${p.id}`);
                document.querySelector(`video[id='${p.id}']`).srcObject = new MediaStream([p.videoTrack]);
            } catch (e) {
                console.info("Error setting video stream: ", e);
            }
        }
    }
}

if (!window.customElements.get('talk-app')) {
    customElements.define('talk-app', Talk);
}

if (!window.customElements.get('dyte-meeting')) {
    defineDyteCustomElements();
}
