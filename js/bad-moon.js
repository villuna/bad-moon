const videoWidth = 480;
const videoHeight = 360;

const emojis = ["🌕", "🌖", "🌗", "🌘", "🌑", "🌒", "🌓", "🌔"];
const blocks = [
    [1, 1, 1, 1],
    [1, 1, 1, 0],
    [1, 1, 0, 0],
    [1, 0, 0, 0],
    [0, 0, 0, 0],
    [0, 0, 0, 1],
    [0, 0, 1, 1],
    [0, 1, 1, 1],
];

// Returns the "difference" between two rows of pixel values
// (actually it's the squared distance, treating them as vectors)
function difference(a, b) {
    const diffs = a.map((e, i) => {
        return (e - b[i]) * (e - b[i]);
    });

    return diffs.reduce((a, b) => a + b, 0);
}

// Takes in a row of pixels and returns the closest moon emoji
function matchingChar(row) {
    let min = null;
    let matching = null;

    for (let i = 0; i < blocks.length; i++) {
        let diff = difference(blocks[i], row);

        if (min == null || min < diff) {
            min = diff;
            matching = i;
        }
    }

    if (matching == null) {
        return "?";
    } else {
        return emojis[matching];
    }
}

const video = document.getElementById("video");
// For complicated reasons this is necessary so we can get image data from the video
video.crossOrigin = "anonymous";

// Setup a button to let the user play the video
// most browsers are cool asf and don't let unmuted videos autoplay
document.getElementById("play").onclick = (e) => {
    video.play()
    video.requestVideoFrameCallback(drawLoop);
    document.getElementById("play").style.display = "none";
};

// Dummy canvas which we can use to transfer image data from the GPU to the CPU
const canvas = document.createElement("canvas");
canvas.height = videoHeight;
canvas.width = videoWidth;
canvas.style.display = "none";
document.body.append(canvas);
const ctx = canvas.getContext("2d");

const frame = document.getElementById("frame");

function drawFrame(data) {
    frame.textContent = "";
    for (let y = 0; y < videoHeight / 8; y++) {
        if (y != 0) {
            frame.appendChild(document.createElement("br"));            
        }

        let str = "";

        for (let x = 0; x < videoWidth / 8; x++) {
            let baseIndex = 4 * (videoWidth * (y * 8 + 4) + x * 8);
            let row = [];

            for (let i = 0; i < 4; i++) {
                row.push(data.data[baseIndex + i * 8] / 255);
            }

            let char = matchingChar(row);
            str += char + " ";
        }

        var t = document.createTextNode(str);
        frame.appendChild(t);
    }
}

// Every frame we draw said frame to the canvas and then load that image data into memory
// then we can process it and ideally display each frame as text!
async function drawLoop(_t, _f) {
    const bitmap = await createImageBitmap(video);
    ctx.drawImage(bitmap, 0, 0);
    const data = ctx.getImageData(0, 0, videoWidth, videoHeight);

    drawFrame(data);

    video.requestVideoFrameCallback(drawLoop);
}