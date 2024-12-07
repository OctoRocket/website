// Fetch code stolen from MDN

const fetch_url = "https://asta.octorocket.dev/api";
const commits = await fetch_commits(fetch_url);

main()

function main() {
    var div = document.getElementById("site-update-block");

    commits.forEach(commit => {
        div.innerHTML += format(commit);
    });
}

function format(commit) {
    function wrap_time(time) {
        return "<span class=\"time-section\">"
            + time.toString().padStart(2, "0")
            + "</span>";
    }
    const timestamp = new Date(commit.date * 1000);

    const name = (commit.message.length > 30)
        ? commit.message.substring(0, 29) + "..."
        : commit.message.substring(0, 32);
    const sha = commit.sha.substring(0, 7);
    const date = "<span class=\"full-time\">"
        + wrap_time(timestamp.getFullYear()) + "-"
        + wrap_time(timestamp.getMonth() + 1) + "-" // JS months start at zero...
        + wrap_time(timestamp.getDate()) + " "
        + wrap_time(timestamp.getHours()) + ":"
        + wrap_time(timestamp.getMinutes()) + ":"
        + wrap_time(timestamp.getSeconds())
        + "</span>";

    return "<p>>("
        + date
        + ")>-<<span class=\"sha\">" + sha
        +"</span>:<br><span class=\"message\">>"+ name
        + "</span>"
        + "</p>\n";
}

async function fetch_commits(url) {
    try {
        const response = await fetch(url);
        if (!response.ok) {
            throw new Error(`Bad response: ${response.status}`);
        }

        return await response.json();
    } catch (error) {
        console.error(error.message)
    }
}
