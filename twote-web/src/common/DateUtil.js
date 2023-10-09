export function date_from_seconds(seconds) {
    return new Date(seconds * 1000)
}

export function timeSince(date) {
    const now = new Date();
    const secondsPast = (now.getTime() - date.getTime()) / 1000;

    if (secondsPast < 60) {
        return parseInt(secondsPast) + ' seconds ago';
    }
    if (secondsPast < 3600) {
        return parseInt(secondsPast / 60) + ' minutes ago';
    }
    if (secondsPast <= 86400) {
        return parseInt(secondsPast / 3600) + ' hours ago';
    }
    if (secondsPast > 86400) {
        const daysPast = parseInt(secondsPast / 86400);
        if (daysPast < 7) {
            return daysPast + ' days ago';
        } else if (daysPast < 30) {
            return parseInt(daysPast / 7) + ' weeks ago';
        } else if (daysPast < 365) {
            return parseInt(daysPast / 30) + ' months ago';
        } else {
            return parseInt(daysPast / 365) + ' years ago';
        }
    }
}