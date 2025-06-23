window.UTCDateStringToFormattedString = function UTCDateStringToFormattedString(dateString) {
    const date = new Date(dateString.get());
    const formatted = new Intl.DateTimeFormat('en-US', {
        dateStyle: 'medium',
        timeStyle: 'short',
    }).format(date);

    return formatted;
}

/**
 * @typedef {import('@dytesdk/web-core').default} DyteClient
 */

window.joinMeeting = async function joinMeeting() {
    if (!window.meeting) { 
        console.error("Meeting not initialized. Ignoring.");
        return;
    }

    /** @type DyteClient */
    const meeting = window.meeting;
    console.info("Joining meeting");
    await meeting.join();
}

window.leaveMeeting = async function leaveMeeting() {
    if (!window.meeting) { 
        console.error("Meeting not initialized. Ignoring.");
        return;
    }

    /** @type DyteClient */
    const meeting = window.meeting;
    console.info("leaving meeting");
    await meeting.leaveRoom();
}

/** Kick all participants and leave the meeting */
window.endMeeting = async function endMeeting() {
    if (!window.meeting) { 
        console.error("Meeting not initialized. Ignoring.");
        return;
    }

    /** @type DyteClient */
    const meeting = window.meeting;
    console.info("leaving meeting");
    await meeting.participants.kickAll();
    await meeting.leaveRoom();
}

window.disableMic = async function disableMic() {
    if (!window.meeting) { 
        console.error("Meeting not initialized. Ignoring.");
        return;
    }

    /** @type DyteClient */
    const meeting = window.meeting;
    console.info("disable mic");
    const audioEnabled = meeting.self.audioEnabled;
    if (audioEnabled) {
        await meeting.self.disableAudio();
    } else {
        console.info("Mic already disabled");
    }
}

window.enableMic = async function enableMic() {
    if (!window.meeting) { 
        console.error("Meeting not initialized. Ignoring.");
        return;
    }

    /** @type DyteClient */
    const meeting = window.meeting;
    console.info("enable mic");
    const audioEnabled = meeting.self.audioEnabled;
    if (audioEnabled) {
        console.info("Mic already enabled");
    } else {
        await meeting.self.enableAudio();
    }
}

window.disableVideo = async function disableVideo() {
    if (!window.meeting) { 
        console.error("Meeting not initialized. Ignoring.");
        return;
    }

    /** @type DyteClient */
    const meeting = window.meeting;

    console.info("disable video");
    const videoEnabled = meeting.self.videoEnabled;
    if (videoEnabled) {
        await meeting.self.disableVideo();
    } else {
        console.info("Video already disabled");
    }
}

window.enableVideo = async function enableVideo() {
    if (!window.meeting) { 
        console.error("Meeting not initialized. Ignoring.");
        return;
    }

    /** @type DyteClient */
    const meeting = window.meeting;

    console.info("enable video");
    const videoEnabled = meeting.self.videoEnabled;
    if (videoEnabled) {
        console.info("Video already enabled");
    } else {
        await meeting.self.enableVideo();
    }
}


window.disableScreenShare = async function disableScreenShare() {
    if (!window.meeting) { 
        console.error("Meeting not initialized. Ignoring.");
        return;
    }

    /** @type DyteClient */
    const meeting = window.meeting;

    console.info("disable screen share");
    const screenShareEnabled = meeting.self.screenShareEnabled;
    if (screenShareEnabled) {
        await meeting.self.disableScreenShare();
    } else {
        console.info("Screen share already disabled");
    }
}

window.enableScreenShare = async function enableScreenShare() {
    if (!window.meeting) { 
        console.error("Meeting not initialized. Ignoring.");
        return;
    }

    /** @type DyteClient */
    const meeting = window.meeting;

    console.info("enable screen share");
    const screenShareEnabled = meeting.self.screenShareEnabled;
    if (screenShareEnabled) {
        console.info("Screen share already enabled");
    } else {
        await meeting.self.enableScreenShare()
    }
}
