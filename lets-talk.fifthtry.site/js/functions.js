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

window.toggleMic = async function toggleMic() {
    if (!window.meeting) { 
        console.error("Meeting not initialized. Ignoring.");
        return;
    }

    /** @type DyteClient */
    const meeting = window.meeting;
    console.info("toggle mic");
    const audioEnabled = meeting.self.audioEnabled;
    if (audioEnabled) {
        await meeting.self.disableAudio();
    } else {
        await meeting.self.enableAudio();
    }
}

window.toggleVideo = async function toggleVideo() {
    if (!window.meeting) { 
        console.error("Meeting not initialized. Ignoring.");
        return;
    }

    /** @type DyteClient */
    const meeting = window.meeting;

    console.info("toggle mic");
    const videoEnabled = meeting.self.videoEnabled;
    if (videoEnabled) {
        await meeting.self.disableVideo();
    } else {
        await meeting.self.enableVideo();
    }
}
