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
            // NOTE: The component handles events itself so we don't need to update it manually
            document.querySelector("dyte-participants-audio").meeting = window.meeting;
        } catch (e) {
            console.info("dyte-participant-audio component not found. Ignoring");
            console.info(e);
        }

        try {
            document.querySelector("dyte-meeting").meeting = window.meeting;
        } catch (e) {
            console.info("dyte-meeting component not found. Ignoring");
            console.info(e);
        }

        window.meeting.self.on("*", (event, ...args) => {
            if (event === "mediaScoreUpdate") return;

            console.info("self update: ", event, args);
            this.updateSelf();
            this.updateActiveParticipant();
            this.maybeClearActiveParticipant();

            setTimeout(() => {
                this.refreshSelfVideoFeed();
                this.refreshParticipantVideoStreams();
            }, 0);

            if (event == "roomJoined") {
                this.data.inside_meeting.set(true);
                this.data.meeting_title.set(window.meeting.meta.meetingTitle);
            }
            if (event === "roomLeft") {
                this.data.inside_meeting.set(false);
                // when I leave the room, clear the participants list
                this.data.participants.clearAll();
            }
        });

        window.meeting.participants.joined.on("*", (event, ...args) => {
            if (event === "mediaScoreUpdate") return;

            console.info("participant update: ", event, args);
            this.updateParticipantsList();
            this.updateActiveParticipant();
            this.maybeClearActiveParticipant();

            setTimeout(() => {
                this.refreshSelfVideoFeed();
                this.refreshParticipantVideoStreams();
            }, 0);
        });

        if (this.data.auto_join?.get()) {
            console.info("Auto-joining meeting. Set auto-join in initialize-meeting to false to disable this");
            await window.meeting.join();
            console.info("Joined meeting");
        }

        // TODO: listen for pinned and waiting participants and update their lists
    }

    /** Updates this.data.self */
    updateSelf() {
        /** @type {DyteClient} */
        const meeting = window.meeting;
        const self = makeParticipant(meeting.self);
        this.data.self.set(fastn.recordInstance(self));
    }

    /** Recreate this.data.participants list */
    // TODO(siddhantk232): This can be optimized
    updateParticipantsList() {
        this.data.participants.clearAll();
        /** @type {DyteClient} */
        const meeting = window.meeting;
        for (const [_id, p] of meeting.participants.joined) {
            this.data.participants.push(fastn.recordInstance(makeParticipant(p)));
        }
    }

    refreshParticipantVideoStreams() {
        /** @type {DyteClient} */
        const meeting = window.meeting;
        for (const p of meeting.participants.joined.toArray()) {
            try {
                if (p.videoEnabled) {
                    console.info(`Setting video stream for participant#${p.id}`);
                    // NOTE: audio of everyone is handled by dyte-participants-audio component
                    const stream = new MediaStream([p.videoTrack]);
                    document.querySelector(`video[id='${p.id}']`).srcObject = stream;
                }
            } catch (e) {
                console.info("Error setting video stream: ", e);
            }

            try {
                if (p.screenShareEnabled) {
                    console.info(`Setting screen share stream for participant#${p.id}`);
                    const stream = new MediaStream([p.screenShareTracks.video]);
                    document.querySelector(`video[id='screen-${p.id}']`).srcObject = stream;
                }
            } catch (e) {
                console.info("Error setting screen share stream: ", e);
            }
        }
    }

    refreshSelfVideoFeed() {
        /** @type {DyteClient} */
        const meeting = window.meeting;

        if (meeting.self.videoEnabled) {
            try {
                const id = meeting.self.id;
                console.info(`Setting video stream for self#${id}`);
                // NOTE: audio of everyone is handled by dyte-participants-audio component
                const stream = new MediaStream([meeting.self.videoTrack]);
                document.querySelector(`video[id='${id}']`).srcObject = stream;
            } catch (e) {
                console.error("Error setting video stream: ", e);
            }
        }

        if (meeting.self.screenShareEnabled) {
            try {
                const id = meeting.self.id;
                console.info(`Setting screen stream for self#${id}`);
                const stream = new MediaStream([meeting.self.screenShareTracks.video]);
                document.querySelector(`video[id='screen-${id}']`).srcObject = stream;
            } catch (e) {
                console.error("Error setting screen share stream: ", e);
            }
        }
    }

    /**
     * Set the first participant with video enabled
     *
     * we check on `meeting.self` too. The UI can choose to not show your
     * screen share to yourself and instead show a button to stop sharing
     * based on:
     * ```js
     * if (activeParticipant.id === self.id) { showStopSharingButton()  }
     * else { showScreenShareFeed()  }
     * ```
     */
    updateActiveParticipant() {
        // don't set if it's already set
        if (this.data.active_participant.get()) {
            console.info("Active participant already set. Ignoring");
            return;
        }

        /** @type {DyteClient} */
        const meeting = window.meeting;

        for (const p of meeting.participants.joined.toArray()) {
            if (p.screenShareEnabled) {
                console.info(`Setting active participant#${p.id}`);
                this.data.active_participant.set(fastn.recordInstance(makeParticipant(p)));
                return;
            }
        }

        console.info("No participant with video enabled found. Checking self");

        if (meeting.self.screenShareEnabled) {
            console.info(`Setting self as active participant#${meeting.self.id}`);
            this.data.active_participant.set(fastn.recordInstance(makeParticipant(meeting.self)));
        }
    }

    /** Clear active participant if no one is sharing their screen */
    maybeClearActiveParticipant() {
        if (!this.data.active_participant.get()) {
            console.info("Active participant already clear. Ignoring");
            return;
        }

        /** @type {DyteClient} */
        const meeting = window.meeting;

        for (const p of meeting.participants.joined.toArray()) {
            if (p.screenShareEnabled) {
                console.info(`Active participant#${p.id} found. Ignoring`);
                return;
            }
        }

        if (meeting.self.screenShareEnabled) {
            console.info(`Self is active participant#${meeting.self.id}. Ignoring`);
            return;
        }

        console.info("Clearing active participant");
        this.data.active_participant.set(null);
    }
}

/**
 * @typedef {import('@dytesdk/web-core').DyteParticipant} DyteParticipant
 * @typedef {import('@dytesdk/web-core').DyteSelf} DyteSelf
 *
 * @typedef {{id: string, name: string, mic: boolean, video: boolean, screen: boolean}} LetsTalkParticipant
 */

/** @param {DyteParticipant | DyteSelf} p
 *  @returns {LetsTalkParticipant}
 * */
function makeParticipant(p) {
    return {
        id: p.id,
        name: p.name,
        mic: p.audioEnabled,
        video: p.videoEnabled,
        screen: p.screenShareEnabled,
    };
}

if (!window.customElements.get('talk-app')) {
    customElements.define('talk-app', Talk);
}

if (!window.customElements.get('dyte-meeting')) {
    defineDyteCustomElements();
}
