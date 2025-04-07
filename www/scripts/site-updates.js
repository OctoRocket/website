// Fetch code stolen from MDN

const fetch_url = "https://asta.octorocket.dev/api";
const commits = await fetch_commits(fetch_url);

main()

function main() {
    var div = document.getElementById("site-update-block");

    commits.forEach(commit => {
        div.innerHTML += format(commit);
    });

    // Update "last updated" text
    var last_updated = document.getElementById("last-updated");
    last_updated.innerHTML = last_update(commits[0]);
}

function format(commit) {
    function wrap_time(time) {
        return "<span class=\"time-section\">"
            + time.toString().padStart(2, "0")
            + "</span>";
    }
    const timestamp = new Date(commit.date * 1000);

    const name = (commit.message.length > 32)
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
        +"</span>:<br>><span class=\"message\">"+ name
        + "</span>"
        + "</p>\n";
}

function last_update(commit) {
    const fancy_options = {
        month: "long",
        day: "numeric",
        weekday: "long",
        year: "numeric",
    }
    const plain_options = {
        month: "2-digit",
        day: "2-digit",
        year: "numeric",
    }
    const timestamp = new Date(commit.date * 1000);
    return timestamp.toLocaleDateString(undefined, fancy_options)
         + " ∨ " + timestamp.toLocaleDateString(undefined, plain_options);
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
