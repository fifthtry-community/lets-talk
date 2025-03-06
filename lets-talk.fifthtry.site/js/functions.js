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
    /** @type DyteClient */
    const meeting = window.meeting;
    if (meeting) {
        console.info("Joining meeting");
        await meeting.join();
    } else {
        console.error("Meeting not initialized. Can't join!");
    }
}

window.leaveMeeting = async function leaveMeeting() {
    /** @type DyteClient */
    const meeting = window.meeting;
    if (meeting) {
        console.info("leaving meeting");
        await meeting.leaveRoom();
    } else {
        console.error("Meeting not initialized. Can't join!");
    }
}

/** Kick all participants and leave the meeting */
window.endMeeting = async function endMeeting() {
    /** @type DyteClient */
    const meeting = window.meeting;
    if (meeting) {
        console.info("leaving meeting");
        await meeting.participants.kickAll();
        await meeting.leaveRoom();
    } else {
        console.error("Meeting not initialized. Can't join!");
    }
}

window.toggleMic = async function toggleMic() {
    /** @type DyteClient */
    const meeting = window.meeting;
    if (meeting) {
        console.info("toggle mic");
        const audioEnabled = meeting.self.audioEnabled;
        if (audioEnabled) {
            await meeting.self.disableAudio();
        } else {
            await meeting.self.enableAudio();
        }
    } else {
        console.error("Meeting not initialized. Can't join!");
    }
}

window.toggleVideo = async function toggleVideo() {
    /** @type DyteClient */
    const meeting = window.meeting;
    if (meeting) {
        console.info("toggle mic");
        const videoEnabled = meeting.self.videoEnabled;
        if (videoEnabled) {
            await meeting.self.disableVideo();
        } else {
            await meeting.self.enableVideo();
        }
    } else {
        console.error("Meeting not initialized. Can't join!");
    }
}
