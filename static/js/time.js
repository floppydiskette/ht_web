let website_url = document.URL;
let clock = document.getElementById("clock");

// e.g. 1 -> "st", 2 -> "nd", etc.
function getDayEnding(day) {
    if (day % 10 == 1) {
        return "st";
    } else if (day % 10 == 2) {
        return "nd";
    } else if (day % 10 == 3) {
        return "rd";
    } else {
        return "th";
    }
}

// to update clock, send "time" to /ws websocket endpoint and
// update the clock element with the response
function updateClock() {
    let ws = new WebSocket("ws://" + website_url.split("/")[2] + "/ws");
    ws.onmessage = function (event) {
        if (event.data === "unknown command") {
            console.log("unknown command");
        } else {
            let time = JSON.parse(event.data);
            let year = time.year;
            let month = "the " + time.month[0].toLowerCase() + " month of " + time.month[1].toLowerCase();
            let day = time.day;
            let second = time.second;
            let sks = second / 6000;
            let rem = second % 6000;
            sks = Math.floor(sks);

            let clock_text = "year " + year + "<br>" + day + getDayEnding(day) + " day of " + month + "<br>" + sks + " sks " + rem + " rem";
            clock.innerHTML = clock_text;
        }
    };
    ws.onopen = function () {
        ws.send("time");
    }
}

// update clock every second
setInterval(updateClock, 100);