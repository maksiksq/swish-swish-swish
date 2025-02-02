<script setup>
import {onMounted, ref} from "vue";
import {
  GestureRecognizer,
  FilesetResolver,
  DrawingUtils
} from "https://cdn.jsdelivr.net/npm/@mediapipe/tasks-vision@0.10.3";

let gestureRecognizer = GestureRecognizer;
let runningMode = "VIDEO";
let isWebcamRunning = true;
const videoHeight = "360px";
const videoWidth = "480px";

let canvasCtx;

const video = ref(null)

const canvasElement = ref(null)
const canvasCtxRef = ref(null);

const gestureOutput = ref(null)

const createGestureRecognizer = async () => {
  const vision = await FilesetResolver.forVisionTasks(
      "https://cdn.jsdelivr.net/npm/@mediapipe/tasks-vision@0.10.3/wasm"
  );
  gestureRecognizer = await GestureRecognizer.createFromOptions(vision, {
    baseOptions: {
      modelAssetPath:
          "https://storage.googleapis.com/mediapipe-models/gesture_recognizer/gesture_recognizer/float16/1/gesture_recognizer.task",
      delegate: "GPU"
    },
    runningMode: runningMode
  });
}

function hasGetUserMedia() {
  return !!(navigator.mediaDevices && navigator.mediaDevices.getUserMedia);
}

const stream = ref(null);

async function startCamera() {
  try {
    stream.value = await navigator.mediaDevices.getUserMedia({ video: true });

    if (video.value) {
      video.value.srcObject = stream.value;
      isWebcamRunning = true;
    }
  } catch (error) {
    console.error("Error accessing the camera:", error);
  }
}

function enableCam(event) {
  if (!gestureRecognizer) {
    alert("Почекай, грузиться.")
    return;
  }


}

let lastVideoTime = -1;
let results = undefined;
async function predictWebcam() {
  const webcamElement = video;
  // Now let's start detecting the stream.
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
    gestureOutput.style.width = videoWidth;
    const categoryName = results.gestures[0][0].categoryName;
    const categoryScore = parseFloat(
        results.gestures[0][0].score * 100
    ).toFixed(2);
    const handedness = results.handednesses[0][0].displayName;
    gestureOutput.innerText = `GestureRecognizer: ${categoryName}\n Confidence: ${categoryScore} %\n Handedness: ${handedness}`;
  } else {
    gestureOutput.style.display = "none";
  }
  // Call this function again to keep predicting when the browser is ready.
  if (isWebcamRunning === true) {
    window.requestAnimationFrame(predictWebcam);
  }
}

onMounted(() => {
  if (hasGetUserMedia()) {
    console.log("cool")
  } else {
    console.warn("getUserMedia() is not supported by your browser");
  }

  canvasCtxRef.value = canvasElement.getContext("2d");
  canvasCtx = canvasElement.getContext("2d")



  createGestureRecognizer();
  enableCam();
  startCamera();

  predictWebcam();

})


</script>

<template>
  <header>
    <h1><span>></span>Swish</h1>
  </header>
  <main class="global-cont">
    <div class="circle"></div>

    <div class="cont">
      <div>
        <video id="webcam" ref="video" autoplay playsinline></video>
        <canvas ref="canvasElement" class="output_canvas" id="output_canvas" width="1280" height="720" style="z-index: 10999; pointer-events: none; position: absolute; left: 0px; top: 0px;"></canvas>
        <p ref="gestureOutput" id='gesture_output' class="output"></p>
      </div>

      <button @click="enableCam">yes</button>
    </div>
  </main>
</template>

<style>
@import url('https://fonts.googleapis.com/css2?family=Roboto+Mono:ital,wght@0,100..700;1,100..700&display=swap');

* {
  margin: 0;
  padding: 0;
}

html {
  overflow-x: hidden;
  overflow-y: hidden;
}

body {
  background-color: #1d1d21;
  overflow-x: hidden;
  overflow-y: hidden;
}

h1 {
  font-family: 'Roboto Mono', sans-serif;
  font-weight: 800;

  font-size: 1.2rem;
}
</style>
<style lang="scss" scoped>
h1 {
  padding-left: 2vw;

  span {
    color: #c6c6c6;
  }
}

:root {
  color: white;
  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.circle { // it's an ellipse but yes
  border-radius: 50%;
  width: 40%;
  height: 40vw;

  position: absolute;
  top: -7vh;
  left: 2vw;
  z-index: -1;

  background-color: #390036;
}

header {
  width: 100%;
  height: 6vh;

  color: white;

  //background-image: linear-gradient(35deg in hsl, #390036 5%, #2d0f36 10%, #241732 15%, #1e1b2b 20%, #1d1d21);
  background-color: rgba(29, 29, 33, 0.6);
  backdrop-filter: blur(300px);
  border-bottom: black 1px solid;

  position: relative;
  z-index: 10099;

  display: flex;
  flex-direction: row;
  //justify-content: flex-end;
  align-items: center;
}

.cont {
  width: 100%;
  height: 94vh;

  position: relative;
  z-index: 10098;

  //background-color: #1d1d21;

  background-color: rgba(29, 29, 33, 0.6);
  backdrop-filter: blur(1000px) grayscale(0.8);

  display: flex;
  flex-direction: column;
  align-items: center;

  button {
    all: unset;

    width: 30vh;
    height: 7vh;
    cursor: pointer;
    border-radius: 4px;
    border: 1px solid rgba(250, 250, 250, 0.3);

    /* From https://css.glass */
    background: rgb(44, 15, 74);
    box-shadow: 0 4px 30px rgba(0, 0, 0, 0.1);
    backdrop-filter: blur(17.3px);
    -webkit-backdrop-filter: blur(17.3px);

  }

  video {
    width: 30vw;
    height: 30vw;
  }
}

.output_canvas {
  transform: rotateY(180deg);
  -webkit-transform: rotateY(180deg);
}
</style>