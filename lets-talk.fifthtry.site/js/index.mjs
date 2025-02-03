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
        const meeting_page_url = data.meeting_page_url.get();

        const endpoint_url = ftd.app_url_ex("/talk/session/", "lets-talk");
        const req_url = `${endpoint_url}?meeting-id=${mid}&meeting-page-url=${meeting_page_url}`;

        const res = await fetch(req_url).then(r => r.json());
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

customElements.define('talk-app', Talk);
defineDyteCustomElements();
