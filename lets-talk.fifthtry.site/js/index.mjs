import DyteClient from "@dytesdk/web-core";
import { defineCustomElements as defineDyteCustomElements } from "@dytesdk/ui-kit/loader";

class Talk extends HTMLElement {
    /** @type {DyteClient} */
    meeting;

    constructor() {
        super();
        // this.style.width = "100%";
        // this.style.height = "100%";
        this.classList.add("fastn-ignore-global-keyboard"); // TODO(siddhantk232): learn what this means?
    }

    async connectedCallback() {
        const data = window.ftd.component_data(this);
        const mid = data.mid.get();

        const endpoint_url = ftd.app_url_ex("/talk/session/", "lets-talk");
        const req_url = `${endpoint_url}?meeting-id=${mid}`;

        const res = await fetch(req_url).then((r) => r.json());

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
    }
}

function UTCDateStringToFormattedString(dateString) {
    const date = new Date(dateString.get());
    const formatted = new Intl.DateTimeFormat("en-US", {
        dateStyle: "medium",
        timeStyle: "short",
    }).format(date);

    return formatted;
}

function to_timestamp_millis(date, time) {
    const dateStr = date.toString();
        const year = parseInt(dateStr.slice(0, 4));
        const month = parseInt(dateStr.slice(4, 6)) - 1; // JS months are 0-based
        const day = parseInt(dateStr.slice(6, 8));

        // Parse time: nanoseconds since midnight
        const nanosSinceMidnight = BigInt(time);

        // Calculate hours, minutes, seconds and milliseconds
        const nanosPerSecond = 1_000_000_000n;

        const totalSeconds = Number(nanosSinceMidnight / nanosPerSecond);
        const hour = Math.floor(totalSeconds / 3600);
        const minute = Math.floor((totalSeconds % 3600) / 60);
        const second = totalSeconds % 60;
        const millisecond = Math.floor(
            Number(nanosSinceMidnight % nanosPerSecond) / 1_000_000
        );

        // Construct JS Date (ms precision)
        const dateObj = Date.UTC(
            year,
            month,
            day,
            hour,
            minute,
            second,
            millisecond
        );

        return dateObj;
}

function schedule_meeting(title, start_datetime, end_datetime, meeting_attendees) {
    const meeting_title = title.getAllFields().value.get("value");
    const start_date = start_datetime.get().getAllFields().date.get();
    const start_time = start_datetime.get().getAllFields().time.get();
    const end_date = end_datetime.get().getAllFields().date.get();
    const end_time = end_datetime.get().getAllFields().time.get();
    const meeting_attendees_string = meeting_attendees.getAllFields().value.get("value").toString()
    const start_datetime_millis = to_timestamp_millis(start_date, start_time).toString();
    const end_datetime_millis = to_timestamp_millis(end_date, end_time).toString(); 

    let data = {"title": meeting_title, "start_datetime": start_datetime_millis.toString(), "end_datetime": end_datetime_millis.toString(), "attendees": meeting_attendees_string}
    console.log("Data is:", data);
    let init = {
        method: "POST",
        redirect: "error",
        credentials: "same-origin",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(data),
    };
    
    fetch("/talk/schedule-meeting", init)
            .then((res) => {
                if (!res.ok) {
                    return new Error("[http_post]: Request failed: " + res);
                }
                return res.json();
            })
            .then((response) => {
                console.log("[http]: Response OK", response);
                if (response.redirect) {
                    window.location.href = response.redirect;
                } else if (!!response && !!response.reload) {
                    window.location.reload();
                } else if (!!response.errors) {
                    for (let key of Object.keys(response.errors)) {
                        let obj = arg_map[key];
                        if (!obj) {
                            console.warn("found unknown key, ignoring: ", key);
                            continue;
                        }
                        let error = response.errors[key];
                        if (Array.isArray(error)) {
                            // django returns a list of strings
                            error = error.join(" ");
                        }
                        // @ts-ignore
                        obj.get("error").set(error);
                    }
                } else if (!!response.data) {
                    console.error("data not yet implemented");
                } else {
                    console.error("found invalid response", response);
                }
            })
            .catch(console.error);
}

window.schedule_meeting = schedule_meeting;
window.UTCDateStringToFormattedString = UTCDateStringToFormattedString;

customElements.define("talk-app", Talk);
defineDyteCustomElements();
