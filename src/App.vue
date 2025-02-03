<script lang="ts" setup>
import {
  GestureRecognizer,
  FilesetResolver,
  DrawingUtils
} from "@mediapipe/tasks-vision"
import {
  warn,
  debug,
  trace,
  info,
  error,
  attachConsole,
  attachLogger,
} from '@tauri-apps/plugin-log';
import {onMounted, ref} from "vue";

const pTxt = ref(null);

let gestureRecognizer: GestureRecognizer;
let runningMode = "IMAGE";

// in the codepen they declared some video ones early, why??? Like I know????? I made them further.

// one goal would be to make
// 1) Model from scratch (help)
// 2) Replace the vision locally (easy)
const createGestureRecognizer = async () => {

  const vision = await FilesetResolver.forVisionTasks("https://cdn.jsdelivr.net/npm/@mediapipe/tasks-vision@0.10.3/wasm") // maybe replace locally?

  gestureRecognizer = await GestureRecognizer.createFromOptions(vision, {
    baseOptions: {
      modelAssetPath: "https://storage.googleapis.com/mediapipe-models/gesture_recognizer/gesture_recognizer/float16/1/gesture_recognizer.task",
      delegate: "GPU"
    },
    // typescript being funni here, RunningMode type is basically a string.
    // @ts-ignore
    runningMode: runningMode
  });
  info('gestureRecognizer initialized');
};

// using refs here, turning them into html objects later on

const vidRef = ref(null);
const canvasElementRef = ref(null);
const gestureOutputRef = ref(null);
// notice canvasCtx is declared later

const enableWebcamButtonRef = ref(null)
let webcamRunning: Boolean = false;

const videoHeight = "360px";
const videoWidth = "480px";

function hasGetUserMedia() {
  return !!(navigator.mediaDevices && navigator.mediaDevices.getUserMedia);
}

function sayHello () {
  info('hi');
}

function enableCam() {
  info('webcam enabling')

  const video = vidRef.value;
  const canvasElement = canvasElementRef.value;
  const gestureOutput = gestureOutputRef.value;
  const canvasCtx = canvasElement.getContext("2d");

  const enableWebcamButton = enableWebcamButtonRef.value;


  if
  (!hasGetUserMedia()) {
    warn("your browser doesn't support getUserMedia");
    return;
  }

  if (!gestureRecognizer) {
    warn("hold on, it's loading");
    return;
  }

  if (webcamRunning === true) {
    webcamRunning = false;
    enableWebcamButton.innerText = "ENABLE PREDICTIONS";
  } else {
    webcamRunning = true;
    enableWebcamButton.innerText = "DISABLE PREDICTIONS";
  }

  // getUserMedia parameters idk why they're necessary but ima forget
  const constraints = {
    video: true
  }

  navigator.mediaDevices.getUserMedia(constraints).then(function (stream) {
    video.srcObject = stream;
    video.addEventListener("loadeddata", predictWebcam);
  });
}

let lastVideoTime = -1;
let results = undefined;
async function predictWebcam() {
  const video = vidRef.value;
  const canvasElement = canvasElementRef.value;
  const gestureOutput = gestureOutputRef.value;
  const canvasCtx = canvasElement.getContext("2d");

  const webcamElement = video;

  if (runningMode === "IMAGE") {
    runningMode = "VIDEO";
    await gestureRecognizer.setOptions({ runningMode: "VIDEO" });
  }
  let nowInMs = Date.now();
  if (video.currentTime !== lastVideoTime) {
    lastVideoTime = video.currentTime;
    results = gestureRecognizer.recognizeForVideo(video, nowInMs);
  }

  canvasCtx.save();
  canvasCtx.clearRect(0, 0, canvasElement.width, canvasElement.height);
  const drawingUtils = new DrawingUtils(canvasCtx);

  canvasElement.style.height = videoHeight;
  webcamElement.style.height = videoHeight;
  canvasElement.style.width = videoWidth;
  webcamElement.style.width = videoWidth;

  if (results.landmarks) {
    for (const landmarks of results.landmarks) {
      drawingUtils.drawConnectors(
          landmarks,
          GestureRecognizer.HAND_CONNECTIONS,
          {
            color: "#00FF00",
            lineWidth: 5
          }
      );
      drawingUtils.drawLandmarks(landmarks, {
        color: "#FF0000",
        lineWidth: 2
      });
    }
  }
  canvasCtx.restore();

  if (results.gestures.length > 0) {
    gestureOutput.style.display = "block";
    // added background color cause that was missing smh
    gestureOutput.style.backgroundColor = "purple";
    gestureOutput.style.width = videoWidth;
    let categoryName = results.gestures[0][0].categoryName;

    // replace categoryName with emoji cause of course
    if (categoryName === "Thumb_Up") {
      categoryName = "👍";
    }
    if (categoryName === "Thumb_Down") {
      categoryName = "👎";
    }
    // this is Peace, not Victory, bite me mediapipe ...
    if (categoryName === "Victory") {
      categoryName = "✌️";
    }
    if (categoryName === "Pointing_Up") {
      categoryName = "☝️";
    }
    if (categoryName === "Closed_Fist") {
      categoryName = "✊";
    }
    if (categoryName === "Open_Palm") {
      categoryName = "👋";
    }
    // I think this gesture is Call me and not ILY but sure
    if (categoryName === "ILoveYou") {
      categoryName = "🤟";
    }

    //

    const categoryScore = parseFloat(
        (results.gestures[0][0].score * 100).toString()
    ).toFixed(2);
    const handedness = results.handednesses[0][0].displayName;
    gestureOutput.innerText = `GestureRecognizer: ${categoryName}\n Confidence: ${categoryScore} %\n Handedness: ${handedness}`;
  } else {
    gestureOutput.style.display = "none";
  }
  // Call this function again to keep predicting when the browser is ready.
  if (webcamRunning === true) {
    window.requestAnimationFrame(predictWebcam);
  }
}

onMounted(() => {
  info('alivency')
  createGestureRecognizer();

  info('ha')
})
</script>

<template>
  <main class="global-cont">

    <div id='liveView' class="videoView"></div>
    <button ref="enableWebcamButtonRef" @click="enableCam" id="webcamButton">Enable webcam</button>
    <div style="position: relative">
      <!--      ref different from example ere, watch out-->
      <video ref="vidRef" id="webcam" autoplay playsinline></video>
      <canvas ref="canvasElementRef" class="output_canvas" id="output_canvas" width="1280" height="720"
              style="position: absolute; left: 0; top: 0"></canvas>
      <p ref="gestureOutputRef" id="gesture_output" class="output"></p>
    </div>
  </main>
</template>

<style>
body {
  background-color: rgba(29, 29, 33, 1);
}
</style>
<style lang="scss" scoped>
main {
  display: flex;
  justify-content: center;
  flex-direction: column;
  align-items: center;
}

button {
  position: relative;
  margin-top: 50vh;
  z-index: 999999;
}

// copy pasted styles from the codepen
body {
  font-family: roboto, serif;
  margin: 2em;
  color: #3d3d3d;
  --mdc-theme-primary: #007f8b;
  --mdc-theme-on-primary: #f1f3f4;
}

h1 {
  color: #007f8b;
}

h2 {
  clear: both;
}

video {
  clear: both;
  display: block;
  transform: rotateY(180deg);
  -webkit-transform: rotateY(180deg);
  -moz-transform: rotateY(180deg);
  height: 280px;
}

section {
  opacity: 1;
  transition: opacity 500ms ease-in-out;
}

.removed {
  display: none;
}

.invisible {
  opacity: 0.2;
}

.detectOnClick {
  position: relative;
  float: left;
  width: 48%;
  margin: 2% 1%;
  cursor: pointer;
}

.videoView {
  position: absolute;
  float: left;
  width: 48%;
  margin: 2% 1%;
  cursor: pointer;
  min-height: 500px;
}

.videoView p,
.detectOnClick p {
  padding-top: 5px;
  padding-bottom: 5px;
  background-color: #007f8b;
  color: #fff;
  border: 1px dashed rgba(255, 255, 255, 0.7);
  z-index: 2;
  margin: 0;
}

.highlighter {
  background: rgba(0, 255, 0, 0.25);
  border: 1px dashed #fff;
  z-index: 1;
  position: absolute;
}

.canvas {
  z-index: 1;
  position: absolute;
  pointer-events: none;
}

.output_canvas {
  transform: rotateY(180deg);
  -webkit-transform: rotateY(180deg);
  -moz-transform: rotateY(180deg);
}

.detectOnClick {
  z-index: 0;
  font-size: calc(8px + 1.2vw);
}

.detectOnClick img {
  width: 45vw;
}

.output {
  display: none;
  width: 100%;
  font-size: calc(8px + 1.2vw);
}
</style>