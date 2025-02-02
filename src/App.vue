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
    runningMode: runningMode
  });
  info('gestureRecognizer initialized');
};

async function handleClick(e) {
  if (!gestureRecognizer) {
    warn("It's loading, hold on a little");
    return;
  }

  if (runningMode === "VIDEO") {
    runningMode = "IMAGE";
    await gestureRecognizer.setOptions({ runningMode: "IMAGE" });
  }

  const allCanvas = e.target.parentNode.getElementsByClassName("canvas");
  for (var i = allCanvas.length - 1; i >= 0; i--) {
    const n = allCanvas[i];
    n.parentNode.removeChild(n);
  }

  const results = gestureRecognizer.recognize(e.target);
  // info(`gestureRecognizer ${JSON.stringify(results)}`);

  if (results.gestures.length > 0) {
    const p = pTxt.value // reassigned this to a vue ref cause Typescript and Vue

    info(p)

    p.setAttribute("class", "info");

    const categoryName = results.gestures[0][0].categoryName;
    // has to be stringified, idk why haven't they done it in the example codepen
    const categoryScore = parseFloat((results.gestures[0][0].score * 100).toString()).toFixed(2);
    const handedness = results.handednesses[0][0].displayName;

    p.innerText = `GestureRecognizer: ${categoryName}\n Confidence: ${categoryScore}%\n Handedness: ${handedness}`;
    p.style =
        "left: 0px;" +
        "top: " +
        e.target.height +
        "px; " +
        "width: " +
        (e.target.width - 10) +
        "px;";

    const canvas = document.createElement("canvas");
    canvas.setAttribute("class", "canvas");
    canvas.setAttribute("width", e.target.naturalWidth + "px");
    canvas.setAttribute("height", e.target.naturalHeight + "px");

    // added .cssText cause Typescript doesn't have .style on elems for some reason?
    canvas.style.cssText =
        "left: 0px;" +
        "top: 0px;" +
        "width: " +
        e.target.width +
        "px;" +
        "height: " +
        e.target.height +
        "px;";

    e.target.parentNode.appendChild(canvas);
    const canvasCtx = canvas.getContext("2d");
    const drawingUtils = new DrawingUtils(canvasCtx);
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
        lineWidth: 1
      });
    }
  }
}



onMounted(() => {
  info('alivency')
  createGestureRecognizer();
})

function sayHello() {
  info('hello0')
}
</script>

<template>
  <main class="global-cont">
    <div class="detectOnClick">
      <img @click="handleClick" src="https://assets.codepen.io/9177687/idea-gcbe74dc69_1920.jpg" crossorigin="anonymous" title="Click to get recognize!"  alt="ide stop bugging me"/>
      <p ref="pTxt" class="classification removed"></p>
    </div>
    <div class="detectOnClick">
      <img @click="handleClick" src="https://assets.codepen.io/9177687/thumbs-up-ga409ddbd6_1.png" crossorigin="anonymous" title="Click to get recognize!" alt="ide stop bugging me" />
      <p ref="pTxt" class="classification removed"></p>
    </div>

    <div id='liveView' class="videoView"></div>
    <button @click="sayHello">Run</button>
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

  img {
    width: 32vw;
    height: 18vw; // that's how you do math (16:9)

    cursor: pointer;
  }
}

// copy pastad
body {
  font-family: roboto,serif;
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