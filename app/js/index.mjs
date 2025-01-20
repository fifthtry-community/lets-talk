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
        const token = data.token.get();
        const mid = data.mid.get();

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

customElements.define('talk-app', Talk);
defineDyteCustomElements();
