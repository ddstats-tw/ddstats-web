// Javascript functions are used instead of nunjucks macros due to performance and readability.

function ordinal(n) {
    var s = ["th", "st", "nd", "rd"]
    var v = n%100
    return (s[(v-20)%10] || s[v] || s[0])
}

function plural(value, unit) {
    return value + " " + unit + (value === 1 ? "" : "s")
}

function timeFormat(seconds) {
    let remaining = seconds
    let years = Math.floor(remaining / 31536000)
    remaining %= 31536000
    let hours = Math.floor(remaining / 3600)
    remaining %= 3600
    let minutes = Math.floor(remaining / 60)
    remaining %= 60

    if(years > 0) {
        return plural(years, "year")
    } else if (hours > 0) {
        return plural(hours, "hour")
    } else if (minutes > 0) {
        return plural(minutes, "minute")
    } else {
        return plural(remaining, "second")
    }
}

function fancyTime(inputSecs) {
    let seconds = Math.floor(inputSecs)
    let hourSecs = seconds % 3600
    let hours = (seconds - hourSecs) / 3600
    let minSecs = (seconds - hours * 3600) % 60
    let mins = (seconds - hours * 3600 - minSecs) / 60
    let secsMillis = (inputSecs - hours * 3600 - mins * 60).toFixed(2)
    let secs = Math.floor(secsMillis)
    let millis = Math.round((secsMillis - secs) * 100)

    let timeStr = ""
    if (hours) {
        timeStr += hours + ":"
    }

    timeStr += (mins < 10 ? "0" : "") + mins + ":"
    timeStr += (secs < 10 ? "0" : "") + secs + "."
    timeStr += (millis < 10 ? "0" : "") + millis

    return timeStr
}

function getCurrentDate() {
    return new Date().toISOString().split("T")[0]
}

function truncateString(string) {
    if(string.length > 32) {
        return string.substring(0, 32) + "…"
    }
    return string
}

export default {
    ordinal,
    timeFormat,
    fancyTime,
    getCurrentDate,
    truncateString
}