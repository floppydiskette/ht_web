let website_url = document.URL;
let clock = document.getElementById("clock");
let month = document.getElementById("month");
let today = -1;

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

// called if day changes on updateClock()
function updateCalendarDay(day) {
    // get class "day[day]" and set it to .today
    let days = document.getElementsByClassName("day" + day);
    let previous_day = document.getElementsByClassName("today");
    for (let i = 0; i < previous_day.length; i++) {
        previous_day[i].classList.remove("today");
    }
    for (let i = 0; i < days.length; i++) {
        days[i].classList.add("today");
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
            let month_text = "the " + time.month[0].toLowerCase() + " month of " + time.month[1].toLowerCase();
            let day = time.day;
            let second = time.second;
            let sks = second / 6000;
            let rem = second % 6000;
            sks = Math.floor(sks);

            let clock_text = "year " + year + "<br>" + day + getDayEnding(day) + " day of " + month_text + "<br>" + sks + " sks " + rem + " rem";
            clock.innerHTML = clock_text;
            month.innerHTML = month_text;

            // todo! tell jetbrains to shut up about type safety, i WANT type coercion
            if (day != today) {
                updateCalendarDay(day);
                today = day;
            }
        }

        // close websocket connection
        ws.close();

        // prevent memory leaks
        ws = null;
    };
    ws.onopen = function () {
        ws.send("time");
    }
}

// update clock every second
setInterval(updateClock, 100);