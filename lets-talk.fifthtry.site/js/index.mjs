import DyteClient from "@dytesdk/web-core";
import { defineCustomElements as defineDyteCustomElements } from "@dytesdk/ui-kit/loader";

class Talk extends HTMLElement {
    /** @type {DyteClient} */
    meeting;
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

        this.meeting = await DyteClient.init({
            authToken: token,
        });

        window.meeting = this.meeting;
        document.querySelector("dyte-meeting").meeting = this.meeting;

        this.meeting.self.on("*", (event, ...args) => {
            if (event === "mediaScoreUpdate") return;

            console.info("self update: ", event, args);
            this.updateSelf();

            if (event == "roomJoined") {
                this.data.inside_meeting.set(true);
            }
            if (event === "roomLeft") {
                this.data.inside_meeting.set(false);
            }
        });

        this.meeting.participants.joined.on("*", (event, ...args) => {
            if (event === "mediaScoreUpdate") return;

            console.info("participant update: ", event, args);
            this.updateParticipantsList();
        });

        // TODO: listen for pinned and waiting participants and update their lists
    }

    /** Updates this.data.self */
    updateSelf() {
        const self = {
            id: this.meeting.self.id,
            name: this.meeting.self.name,
            mic: this.meeting.self.audioEnabled,
            video: this.meeting.self.videoEnabled,
            screen: this.meeting.self.screenShareEnabled,
        }

        this.data.self.set(fastn.recordInstance(self));
    }

    /** Recreate this.data.participants list */
    // TODO(siddhantk232): This can be optimized
    updateParticipantsList() {
        this.data.participants.clearAll();
        for (const [_id, p] of this.meeting.participants.joined) {
            this.data.participants.push(fastn.recordInstance({
                id: p.id,
                name: p.name,
                mic: p.audioEnabled,
                video: p.videoEnabled,
                screen: p.screenShareEnabled,
            }))
        }
    }
}

function UTCDateStringToFormattedString(dateString) {
    const date = new Date(dateString.get());
    const formatted = new Intl.DateTimeFormat('en-US', {
        dateStyle: 'medium',
        timeStyle: 'short',
    }).format(date);

    return formatted;
}

window.UTCDateStringToFormattedString = UTCDateStringToFormattedString;

customElements.define('talk-app', Talk);
defineDyteCustomElements();
