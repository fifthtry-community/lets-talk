function UTCDateStringToFormattedString(dateString) {
    const date = new Date(dateString.get());
    const formatted = new Intl.DateTimeFormat('en-US', {
        dateStyle: 'medium',
        timeStyle: 'short',
    }).format(date);

    return formatted;
}

async function joinMeeting() {
    /** @type {DyteClient} */
    const meeting = window.meeting;
    if (meeting) {
        console.info("Joining meeting");
        await meeting.join();
    } else {
        console.error("Meeting not initialized. Can't join!");
    }
}

window.joinMeeting = joinMeeting;
window.UTCDateStringToFormattedString = UTCDateStringToFormattedString;
