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

// let's imagine for a second this is all encrypted and there's a key generated for every time a password is made
// 7^6 is already 117649 combinations which is basically unpickable by hand if it were used for a real door unless you edit the code which you can (currently)
// cause it's a web app. In the future it could be secured by just securing the password and moving all the security logic to obfuscated rust (maybe checksum tooo?)
// which I think would be very safe. Or move it to a server but if i can keep it internetless why move it to a server.

function unlock() {
  info('hurray you got in, now it`s time to rename yourself to Rob Banks')
}

function block() {
  info('wrong, go to jail!')
}

const passwordLength = ref(6)
// MAKE SURE TO INCLUDE A SPACE AT THE END
const password = ref("Thumb_Up Thumb_Down Victory Closed_Fist Thumb_Up Victory ")
const currentPassword = ref("")
const i = ref(0)
const oldCategoryName = ref("")
let lastTime = 0;


function throttle(func, delay, a) {

  const now = Date.now();
  if (now - lastTime >= delay) {
    func(a);
    lastTime = now;
  }
}


async function matchPassword(categoryName: string) {
  if (i.value === 6) {
    i.value = 0;
    oldCategoryName.value = "";
    currentPassword.value = "";
    block();
    return;
  }

  if (categoryName === oldCategoryName.value || categoryName === "None") {
    return
  }


  i.value++;

  oldCategoryName.value = categoryName;

  currentPassword.value += categoryName;
  currentPassword.value += " ";

  await info("current password:")
  await info(currentPassword.value);
  await info((currentPassword.value === password.value).toString());

  if (currentPassword.value === password.value) {
    i.value = 0;
    oldCategoryName.value = "";
    currentPassword.value = "";
    unlock();
  }
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
    await gestureRecognizer.setOptions({runningMode: "VIDEO"});
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
            color: "#500077",
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
    gestureOutput.style.width = videoWidth;
    let categoryName = results.gestures[0][0].categoryName;
    const categoryNameStored = categoryName;

    throttle(matchPassword, 500, categoryNameStored);

    // replaced categoryName with emoji cause of course
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
    gestureOutput.innerText = `${categoryName}`;
    // gestureOutput.innerText = `GestureRecognizer: ${categoryName}\n Confidence: ${categoryScore} %\n Handedness: ${handedness}`;
  } else {
    gestureOutput.style.display = "block";
  }
  // Call this function again to keep predicting when the browser is ready.
  if (webcamRunning === true) {
    window.requestAnimationFrame(predictWebcam);
  }
}

onMounted(() => {
  createGestureRecognizer();
})
</script>

<template>
  <main class="global-cont">
    <h3>Press the button to start entering the lock combination.<br>
      If you fail, you get punished.
    </h3>
    <button ref="enableWebcamButtonRef" class="webCamBtn" @click="enableCam" id="webcamButton">Enable webcam</button>
    <div class="canvasCont">
      <!--      ref different from example ere, watch out-->
      <video ref="vidRef" id="webcam" class="vid" autoplay playsinline></video>
      <canvas ref="canvasElementRef" class="output_canvas" id="output_canvas" width="1280" height="720"
              style="position: absolute; left: 0; top: 0"></canvas>
      <p ref="gestureOutputRef" id="gesture_output" class="output">GestureRecognizer: <br>Confidence: <br>Handedness:
      </p>
    </div>
  </main>
</template>

<style>
body, html {
  overflow-x: hidden;
  overflow-y: hidden;
  background-color: rgba(29, 29, 33, 1);
}
</style>
<style lang="scss" scoped>
main {
  display: flex;
  justify-content: center;
  flex-direction: column;
  align-items: center;

  padding-top: 10vw;

  h3 {
    font-size: 1rem;
    color: white;
    font-family: Comfortaa, sans-serif;
    font-weight: normal;
    text-align: center;

    line-height: 2rem;
  }

  .videoView {
    cursor: initial;
  }

  .webCamBtn {
    cursor: pointer;
  }

  .canvasCont {
    display: flex;
    justify-content: left;
    position: relative;

    margin-top: 4vw;

    p {

      height: 15vh;
      width: 480px;
    }

  }
}

button {
  position: relative;
  margin-top: 2vh;
  z-index: 999999;
  padding: 0.6vw 1vw;

  color: white;
  font-family: 'Comfortaa', sans-serif;
  font-size: 1rem;
  border: 3px solid #000000;
  background-color: #8a00bf;
  //mix-blend-mode: hard-light; color

  box-shadow: 4px 4px 0 black, 0 1px 0 black, 1px 2px 0 black, 2px 3px 0 black;

  transition: all 0.2s ease-in-out;
}

button::after {
  content: "";
  pointer-events: none;

  //background: repeating-linear-gradient(75deg,
  //    rgb(0, 0, 0, 0.2) 0%,
  //    rgb(0, 0, 0, 0.2) 0.001px,
  //    rgb(0, 0, 0, 0) 5px,
  //    rgb(0, 0, 0, 0) 10px
  //);
  opacity: 0.8;

  position: absolute;
  z-index: -1;
  inset: 0;
  width: 100%;
  height: 100%;
}

button:hover {
  transform: translate(3px, 3px);
  box-shadow: 1px 1px 0 black;
  background-color: #9e04da;
  color: #000000;
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
  //width: 48%;
  margin: 2% 1%;
  cursor: pointer;
  //min-height: 500px;
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