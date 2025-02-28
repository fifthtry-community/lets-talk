import DyteClient from "@dytesdk/web-core";
import { defineCustomElements as defineDyteCustomElements } from "@dytesdk/ui-kit/loader";

class Talk extends HTMLElement {
    /** @type {DyteClient} */
    meeting;

    constructor() {
        super();
        // this.style.width = "100%";
        // this.style.height = "100%";
        this.classList.add('fastn-ignore-global-keyboard'); // TODO(siddhantk232): learn what this means?
    }

    async connectedCallback() {
        const data = window.ftd.component_data(this);
        const mid = data.mid.get();

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

        document.querySelector("dyte-meeting").meeting = this.meeting;

        this.meeting.self.on("roomJoined", () => {
            const self = {
                id: this.meeting.self.id,
                name: this.meeting.self.name,
                mute: this.meeting.self.audioEnabled,
                video: this.meeting.self.videoEnabled,
            }
            data.self.set(fastn.recordInstance(self));
            data.inside_meeting.set(true);
            console.log("room joined by self: ", self.name);
            window.pass_meeting(this.meeting);
        });

        this.meeting.participants.joined.on("participantJoined", (p) => {
            console.log("participant joined: ", p.name);
            data.participants.push(fastn.recordInstance({
                id: p.id,
                name: p.name,
                mute: p.audioEnabled,
                video: p.videoEnabled,
            }));
        });

        this.meeting.participants.joined.on("participantLeft", (p) => {
            console.log("participant left: ", p.name);
            const index = data.participants.get().findIndex(participant => {
                const id = participant.item.get("id").get();
                return id == p.id;
            });
            console.log("removing index: ", index);
            data.participants.deleteAt(index);
        });

        // TODO: listen for self audio/video change and update self record
        // TODO: listen for screenshare change and update multiple participant lists
        // TODO: listen for pinned change and update multiple participant lists
        // we want pinned participant list, screen sharing participants and, participants
    }

}

function UTCDateStringToFormattedString(dateString) {
    const date = new Date(dateString.get());
    const formatted =  new Intl.DateTimeFormat('en-US', {
        dateStyle: 'medium',
        timeStyle: 'short',
    }).format(date);

    return formatted;
}

window.UTCDateStringToFormattedString = UTCDateStringToFormattedString;

/** 
 * @param {DyteClient} meeting
 * */
window.pass_meeting = function (meeting) {
    document.querySelector("dyte-grid").meeting = meeting;
    document.querySelector("dyte-camera-toggle").meeting = meeting;
    document.querySelector("dyte-mic-toggle").meeting = meeting;
}

customElements.define('talk-app', Talk);
defineDyteCustomElements();
