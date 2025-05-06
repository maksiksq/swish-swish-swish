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
import {onMounted, ref, watch} from "vue";
import Sidebar from "./Sidebar.vue";
import {invoke} from "@tauri-apps/api/core";
import { load, set, save } from '@tauri-apps/plugin-store';
import {
  BleDevice,
  getConnectionUpdates,
  startScan,
  sendString,
  readString,
  unsubscribe,
  subscribeString,
  stopScan,
  connect,
  disconnect,
  getScanningUpdates
} from '@mnlphlp/plugin-blec'


// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⣤⣶⣷⣦⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠄⠠⡐⠀⠊⠍⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣤⣾⣿⣿⣿⣿⣿⣿⣿⣦⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⡄⢀⡀⠔⠖⠒⠐⠅⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣠⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣄⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠠⣷⣾⡾⠥⢔⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣴⣿⣿⣿⡿⡛⠛⠛⡻⣿⣿⣿⣿⣿⣿⣿⣿⣿⠿⠷⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⢳⠁⠀⠀⠈⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣶⣿⣿⣿⣿⡏⠐⠠⠠⢠⠐⠘⣿⣿⣿⣿⣿⣿⣿⠡⢂⢀⠀⢀⢀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠈⣷⣄⣀⣀⡀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣿⣿⣿⣿⣿⣧⡐⠈⣀⠀⠕⣰⣿⣿⣿⣿⣿⣿⣿⡐⡈⠐⢐⠄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠸⣿⣿⣿⣿⣿⣆⠀⠀⠀⠀⠀⢀⣴⣿⣿⣿⣿⣿⣿⣿⣶⣶⣶⣾⣿⣿⣿⣿⣿⣿⣿⣿⣷⣬⣀⣀⣡⡌⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠹⣿⣿⣿⣿⣿⣦⠀⠀⢀⣠⣾⣿⣿⣿⠏⠄⠀⡙⠛⠋⡔⠦⡀⣠⠤⡛⠛⢃⣀⢠⠲⡀⡹⠿⠟⠉⡁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠈⢿⣿⣿⣿⣿⣷⣾⣿⣿⣿⣿⣿⣿⣬⣌⠉⠀⠑⠜⣠⣤⣤⡁⠀⠱⠰⠁⢈⣤⣤⠐⠒⠁⠱⠜⡀⠆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠘⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⢹⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣧⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠉⠛⠛⠛⠻⠿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠿⠿⠛⠛⠋⠉⠉⠉⠁⠀⠀⠀⠀⢀⣀⣀⣀⠀⠀⠉⠙⠛⠻⢷⣦⣄⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣠⣿⣿⣿⣿⣿⡿⠟⠛⠉⠁⠀⠀⠀⠀⣀⡀⣤⣤⡶⣶⣾⣿⣿⣯⢿⣿⣿⣯⢷⡿⣿⣶⣶⣦⣤⣄⣈⠉⠒⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⣀⣤⣴⣾⣿⣿⣿⠿⠛⠉⠀⠀⠀⢀⢀⣤⣴⣶⣎⢯⢿⣾⣿⠏⣸⣿⣿⣿⣿⣿⣏⢻⣿⡽⡈⣿⣾⣿⣿⣿⣿⣿⣿⣿⣶⣦⣄⡀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠈⠛⠛⠛⠛⠋⠉⠁⠀⠀⠀⣀⡤⣢⣿⡟⡟⣿⣿⣿⡜⢯⣿⢯⠋⠀⣿⡟⣿⢿⢿⣿⣿⡄⠻⡇⠁⠸⣿⡽⣎⣛⣿⣿⣿⣿⣿⣿⣿⣿⣷⣦⡀⠀⠀
// ⠀⢬⣤⣤⣤⣤⣤⣤⣀⣤⣤⣤⣤⣶⣾⣿⠟⡼⢣⣿⣹⠇⠉⣿⣧⣧⣿⣋⠃⠀⠀⡟⠀⠛⢸⢸⢻⣿⣧⠀⠀⣇⠀⢿⣿⡽⣿⣿⣿⣿⣿⣿⠟⠛⠉⠉⠉⠙⢷⡀
// ⠀⠀⠙⠛⠿⠿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣯⠺⡽⣾⠃⣿⠀⠀⠏⣿⠹⣯⣧⣤⣈⠐⣷⠀⠀⢺⢈⡎⡿⣻⠀⣤⣿⠀⡾⣿⢷⠿⠿⠟⠛⠋⠁⠀⠀⠀⠀⠀⠀⠀⡃
// ⠀⠀⠀⠈⠢⠀⠀⠉⠉⠛⠿⣿⣿⣿⣿⣾⣸⢻⢯⡁⡇⢰⢰⠀⠳⡴⠸⢏⠟⡉⠁⠀⠙⠂⠘⡆⠑⡞⠛⠃⡟⡿⠀⡇⣿⡎⡇⠀⠀⠀⠀⠀⠀⠀⡀⠄⠐⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠂⠀⠀⠀⠀⠀⠉⠛⠿⠿⠿⢬⠎⢡⠁⠈⠸⡆⡄⢷⠀⠂⠃⠀⠀⠀⠀⠀⠀⣇⠉⡇⢸⡀⠁⡇⢀⡁⢿⡇⠀⠀⠀⠀⠀⠄⠀⠈⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠄⢀⠀⠀⠀⠀⢀⠔⠅⣰⢠⣆⢸⣷⣟⠷⠠⠑⠀⠀⠀⠀⠀⠀⠀⠀⠈⠑⠒⣱⠇⣧⡁⢸⡁⡞⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣦⡼⢿⣿⣿⣿⣷⡇⠀⠀⠀⢀⣀⣄⠀⠀⠀⠀⣴⡿⣄⡟⢱⡿⡏⠿⣦⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠠⠠⢀⣀⣤⣀⣀⣀⣀⣀⣴⣾⣿⣶⡺⣿⣿⣦⡹⣿⣿⡿⢀⠁⢀⠀⠀⠀⠘⠀⣀⡴⡾⣹⣿⢧⠀⣼⣷⡀⠀⠘⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠠⡀⠻⣿⣷⣻⣿⣿⣿⣿⣿⣿⣿⣿⣮⣻⣿⣷⡹⣿⠃⣼⠀⠀⢀⠀⢀⣴⣾⣿⢿⢱⣷⡿⡆⣸⣿⣿⣧⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠘⣄⠈⢿⣷⣻⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣧⠉⣰⣿⠀⢠⢠⣄⣸⣷⠉⠛⠷⡼⣿⣼⢠⣛⣻⠿⢿⣇⢆⣀⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠈⢷⣄⠙⢷⡻⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣦⣿⣿⡇⠘⠿⠿⠿⠏⠀⠀⡀⠊⠻⡏⣸⣿⣿⣿⣿⣶⣶⣿⣟⢿⣿⣆⠁⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠹⣷⣄⠑⠽⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠃⠀⠀⠀⠁⠀⠀⠁⠀⠀⠠⠃⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡄⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢶⣷⣄⠙⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠀⣾⢦⡄⢈⡉⠭⠄⠀⠨⠁⠰⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⢸⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⢿⣷⣦⡈⠛⢿⣿⣿⣿⣿⣿⣿⠟⠁⠈⠙⢄⠈⠒⢄⡀⢀⠔⠁⠀⠀⠛⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠇⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠻⢿⣿⣷⣦⣄⣉⠉⠉⠉⠀⠀⠀⠀⠀⠀⠀⢢⣀⠈⠉⠀⡎⠀⠀⠀⠘⣿⣿⣿⣿⣿⣿⣿⣿⣿⠟⠐⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⢿⣿⣿⣿⣧⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣷⠄⢀⠀⣧⠀⠀⠀⠀⡙⣿⣿⣿⣿⣿⣿⣿⠋⠀⠁⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠻⢿⣿⣧⠀⠀⠀⠀⠀⠀⠀⠀⠀⢹⠄⢁⠘⣿⠀⠀⠀⠀⠐⡈⠻⣿⣿⣿⠟⢁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠛⠆⠀⠀⠀⠀⠀⠀⠀⠀⢸⡆⡀⠀⣿⠀⠀⠀⠀⠀⢸⣶⣤⡉⠄⠂⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠂⠀⠀⠤⠤⠼⠦⠤⠤⠿⠀⠠⠤⠀⠀⠊⠉⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀

// --------------------------------------
// === CONFIG ===
// --------------------------------------

// Delay between each gesture recognition, to give the user a little bit of time to breathe
// Can be offset by GestureRecognizer taking a while to do its thing on a low-end machine I imagine
const DELAY_BETWEEN_INPUTS = 550;

// Yes.
const DO_RICKROLL = false;

// --------------------------------------
// === CONFIG END ===
// --------------------------------------

// --------------------------------------
// === OPTIONS (available in-app) ===
// --------------------------------------

const automaticallyCloseLock = ref(false);

// --------------------------------------
// === OPTIONS END ===
// --------------------------------------

// --------------------------------------
// === HERE'S WHERE THE MAGIC HAPPENS ===
// --------------------------------------

// if you want to plug in something here, this is the place

// let's imagine for a second this is all encrypted and there's a key generated for every time a password is made
// 7^6 is already 117649 combinations which is basically unpickable by hand if it were used for a real door unless you edit the code which you can (currently)
// because it's a web app. In the future it could be secured by just securing the password and moving all the security logic to obfuscated rust and sending the
// ble requests with ECDH. Which I think would be very safe.

const CHARACTERISTIC_UUID = "beb5483e-36e1-4688-b7f5-ea07361b26a8";

const scanning = ref<Boolean>(false)
const devices = ref<BleDevice[]>([])

// This thing makes sure the "currently nothing found in the vicinity, is Bluetooth on? 🔗"
// does not show up in initialization, only the first time tho because lazy
const avoidConfusion = ref(true);

watch(devices, async () => {
  // enables the lock button
  isLockButtonGreyedOut.value = false;

  await info('i got this far');
  console.log("unlocking with:");
  console.log(devices);
  console.log(devices.value);

  if (devices.value.length === 0) {
    if (avoidConfusion.value) {
      !avoidConfusion.value;
    } else {
      success.value = 'currently nothing found in the vicinity, is Bluetooth on? 🔗';
    }
  }
  console.log("extra test")

  const j = ref(0);
  for (const device of devices.value) {
    console.log("trying to connect");
    console.log(j.value);
    console.log(device);
    if (device.name === "ESP32_LED_Control") {
      await connect(device.address, () => info('disconnected'));
      connectedToLock.value = true;
      console.log("Look:");
      console.log(Date.now());
    }
    console.log("Current j:");
    console.log(j.value);
    console.log("is last");
    console.log(j.value === devices.value.length - 1);
    console.log("device lenght");
    console.log(devices.value.length);
    if (j.value === devices.value.length - 1) {
      success.value = 'no lock found in the vicinity which is about 5-15 meters (oh no) 📡';
    }
    j.value += 1;
  }


  console.log("minefield")
  await sendBleCommand("on");


  await sendString(CHARACTERISTIC_UUID, "on");

  // open sesame
  success.value = 'hurray you got in 🍪, now it`s time to rename yourself to Rob Banks! Now you get your well deserved 🍪 personal cookie stash 🍪 🍪🍪'

  // if autolock, lock in 30 seconds
  if (automaticallyCloseLock.value) {
    setTimeout(async () => {
      await sendBleCommand("off");
    }, 30000)
  }
})

async function disconnectFromLock() {
  // disconnecting past connection in case it was connected
  if (connected.value) {
    // await disconnect()
  }
}

async function unlock() {
  await info('hurray you got in, now it`s time to rename yourself to Rob Banks');
  await info(devices.value.toString());

  await info("hello??????!");

  await info("hello??????");


  success.value = 'looking for devices 📡'

  devices.value = [];

  console.log("hello????")
  await info("hello");
  success.value = 'attempting to send the command to lock ⌛'

  // setTimeout(async () => {
  //   if(!connected.value) {
  //     success.value = 'no device connected within 10 seconds, is Bluetooth on? 🔗';
  //   }
  // }, 10000)
  await startScan((dv: BleDevice[]) => {
    console.log("testtsetestestssssssssssss")
    info(dv.toString());
    devices.value = dv;
  }, 10000);

}

async function block() {
  await info('wrong, go to jail!')

  // you shall not pass (closes the lock)
  await sendBleCommand("off");

  if (DO_RICKROLL) {
    window.open("https://www.youtube.com/watch?v=xvFZjo5PgG0&ab_channel=Duran", "_blank");
  }

  success.value = 'I wonder what\'s behind this door 🔒🚪 ; 👍 👎 ✌️ ✊ 👍 ✌️ '
}

// --------------------------------------
// === HERE'S WHERE THE MAGIC ENDS ===
// --------------------------------------

let store = null;

const setSetting = async (key: string, value: any) => {
  console.log("trying to get from:")
  console.log(store)
  console.log("setting:")
  console.log(value)
  await store.set(key, { value: value });

  await store.save();
}

const getSetting = async (key: string) => {
  console.log("attempt key");
  console.log(key);
  console.log("attempt key");
  console.log(store);


  const setting = await store.get(key) ?? 'nothingYet';

  console.log("attempt");
  console.log(setting);

  if (setting === undefined && key === "autoLock") {
    console.log("no setting");
    await setSetting(key, true);
    return true;
  }


  console.log("attempt pt2");
  console.log(setting);

  console.log("Persistent value:");
  console.log(setting.value);

  return setting.value;
};

const connected = ref<Boolean>(false)
const connectedToLock = ref<Boolean>(false)

const isLockButtonGreyedOut = ref(true);
const isWebcamButtonGreyedOut = ref(false);

const buttonGreyedOutClass = ref('buttonGreyedOut');
const buttonActiveClass = ref('buttonActive');

const connectionTxt = ref("Awaiting unlock...");

const pTxt = ref(null);

let gestureRecognizer: GestureRecognizer;
let runningMode = "IMAGE";

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
const isDoor = ref(false);
// notice canvasCtx is declared later

const enableWebcamButtonRef = ref(null)
let webcamRunning: Boolean = false;

const videoHeight = "360px";
const videoWidth = "480px";

const webcamShadowClass = ref('webcamShadow');

function hasGetUserMedia() {
  return !!(navigator.mediaDevices && navigator.mediaDevices.getUserMedia);
}

function enableCam() {
  info('webcam enabling')
  isWebCamOn.value = true;
  isDoor.value = true;

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
    enableWebcamButton.innerText = "Enable Predictions";
  } else {
    webcamRunning = true;
    enableWebcamButton.innerText = "Disable Predictions";
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

const enableAutoLock = async () => {
  console.log("enabling auto lock")
  automaticallyCloseLock.value = await getSetting("autoLock");
  console.log("acquired setting:")
  console.log(automaticallyCloseLock.value)
  if (automaticallyCloseLock.value === undefined || automaticallyCloseLock.value === null) {
    automaticallyCloseLock.value = true;
  }

  await setSetting('autoLock', !automaticallyCloseLock.value)
}

const ifRun = ref(true)

const passwordLength = ref(6)
// MAKE SURE TO INCLUDE A SPACE AT THE END 3 AM ME
const password = ref("Thumb_Up Thumb_Down Victory Closed_Fist Thumb_Up Victory ")
const success = ref("I wonder what's behind this door 🔒🚪 ; 👍 👎 ✌️ ✊ 👍 ✌️ ")

const currentPassword = ref("")

// this is the global password iterator
const i = ref(0)
const oldCategoryName = ref("")
let lastTime = 0;

const isResetPasswordMode = ref(false);
const h3txt1 = ref("Press the button to start entering the lock combination.")
const h3txt2 = ref("If you fail, you get punished.")

const isWebCamOn = ref(false)
const currentCombo = ref("🧈")


function throttle(func, delay, a) {

  const now = Date.now();
  if (now - lastTime >= delay) {
    func(a);
    lastTime = now;
  }
}

const emoji = ref("");

function convertToEmoji(s: string) {
  const allEmoji = s.split(" ")

  allEmoji.forEach((elem) => {
    if (elem === "Thumb_Up") {
      s += "👍";
    }
    if (elem === "Thumb_Down") {
      s += "👎";
    }
    if (elem === "Victory") {
      s += "✌️";
    }
    if (elem === "Pointing_Up") {
      s += "☝️";
    }
    if (elem === "Closed_Fist") {
      s += "✊";
    }
    if (elem === "Open_Palm") {
      s += "👋";
    }
    if (elem === "ILoveYou") {
      s += "🤟";
    }


  })
  return s;
}

function matchEmoji(categoryName: string) {
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
  return categoryName;
}

async function reEnable() {
  setTimeout(() => {
    ifRun.value = true;
    if (webcamRunning) {
      predictWebcam();
    }
  }, 1500)
}

const clearCurrentLockCombo = async () => {
  i.value = 0;
  oldCategoryName.value = "";
  currentPassword.value = "";
  currentCombo.value = "";
}

async function matchPassword(categoryName: string) {
  if (i.value === 0) {
    currentPassword.value = "";
    currentCombo.value = "";
  }

  if (categoryName === oldCategoryName.value || categoryName === "None") {
    return
  }

  // might as well add the emoji to the current thing with the same function cause why not
  currentCombo.value += emoji.value;

  oldCategoryName.value = categoryName;

  currentPassword.value += categoryName;
  currentPassword.value += " ";

  await info("password:")
  await info(password.value);
  await info("current password:")
  await info(currentPassword.value);
  await info((currentPassword.value === password.value).toString());

  if (i.value === 5 && password.value === "blank") {
    password.value = currentPassword.value;
    h3txt1.value = "Press the button to start entering the lock combination."
    h3txt2.value = "If you fail, you get punished."

    await setSetting('password', password.value)

    success.value = "I wonder what's behind this door 🔒🚪 ; " + convertToEmoji(password.value);
  }


  // console.log("------------------");
  // console.log("the i is:");
  // console.log(i.value);
  // console.log("the current password is:");
  // console.log(currentPassword.value);
  // console.log("the current real password is:");
  // console.log(password.value);
  // console.log("does it match");
  // console.log(currentPassword.value === password.value);
  // console.log("is my condition just wrong?? (yes)");
  // console.log(i.value === 5 && currentPassword.value === password.value);
  // console.log("------------------");


  if (i.value === 5 && currentPassword.value === password.value) {
    console.log("testetstessetstesetset");

    await clearCurrentLockCombo();
    await disconnectFromLock();
    await unlock();

    if (isResetPasswordMode.value) {
      console.log("RESETTING")
      password.value = "blank";
      isResetPasswordMode.value = false;
      h3txt1.value = "Now input the new password. You can always reset it";
      h3txt2.value = "from the sidebar, provided you remember the password.";
    }

    return;
  } else if ((i.value === 5 && currentPassword.value !== password.value) || i === 6) {
    await clearCurrentLockCombo();
    await block();

    ifRun.value = false;
    await reEnable()

    return;
  }

  i.value++;
}

let lastVideoTime = -1;
let results = undefined;

async function predictWebcam() {
  if (ifRun.value == false) {
    return;
  }

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

    emoji.value = matchEmoji(categoryName);
    throttle(matchPassword, DELAY_BETWEEN_INPUTS, categoryNameStored);

    const categoryScore = parseFloat(
        (results.gestures[0][0].score * 100).toString()
    ).toFixed(2);
    const handedness = results.handednesses[0][0].displayName;
    gestureOutput.innerText = `${emoji.value}`;
    // gestureOutput.innerText = `GestureRecognizer: ${categoryName}\n Confidence: ${categoryScore} %\n Handedness: ${handedness}` ;
  } else {
    gestureOutput.style.display = "block";
  }
  // Call this function again to keep predicting whenever the browser is ready.
  if (webcamRunning === true) {
    window.requestAnimationFrame(predictWebcam);
  }
}

onMounted(async () => {
  await createGestureRecognizer();

  store = await load('store.json', { autoSave: false });

  const passwordFromStorage = await getSetting('password');
  console.log("password from storage:");
  console.log(passwordFromStorage);
  if (passwordFromStorage !== undefined && passwordFromStorage !== "nothingYet") {
    console.log("hai")
    password.value = passwordFromStorage;

  }

  await getConnectionUpdates((state) => connected.value = state)
  await getScanningUpdates((state) => {
    console.log('Scanning:', state)
    scanning.value = state
  })

  setInterval(() => {
    console.log("is thy boi scanin?:")
    console.log(scanning.value)
    console.log("is thy boi connectin?:")
    console.log(connected.value)
    console.log("is thy boi scanin?:")
    console.log(scanning.value)

    if (connected.value) {
      connectionTxt.value = "Connected to the lock successfully ✅";
    } else {
      connectionTxt.value = "Awaiting connection... 👀";
    }
  }, 1000)
})


const isSideBar = ref(false);

function openSidebar() {
  isSideBar.value = !isSideBar.value;
}

function resetPasswordStart() {
  h3txt1.value = "Password reset mode, first enter the current password correctly.";
  h3txt2.value = "If else, access denied.";

  isResetPasswordMode.value = true;
}

//
// Talking to the ESP32
// via BLE ofccc
//

async function sendBleCommand(flipper: string) {
  await info("hello");
  console.log("hello");
  try {
    const result = await invoke("send_ble_command", {cmd: flipper});
    await info(result);
  } catch (err) {
    await info(`BLE command failed ${err.toString()}`);
  }
}

</script>

<template>
  <main class="global-cont">
    <div v-on:click="openSidebar" class="settings-button">
      <svg class="settings-svg" xmlns="http://www.w3.org/2000/svg" fill="white" viewBox="0 0 448 512">
        <!--!Font Awesome Free 6.7.2 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2025 Fonticons, Inc.-->
        <path
            d="M0 96C0 78.3 14.3 64 32 64l384 0c17.7 0 32 14.3 32 32s-14.3 32-32 32L32 128C14.3 128 0 113.7 0 96zM0 256c0-17.7 14.3-32 32-32l384 0c17.7 0 32 14.3 32 32s-14.3 32-32 32L32 288c-17.7 0-32-14.3-32-32zM448 416c0 17.7-14.3 32-32 32L32 448c-17.7 0-32-14.3-32-32s14.3-32 32-32l384 0c17.7 0 32 14.3 32 32z"/>
      </svg>
    </div>
    <Sidebar @click-on-burger="openSidebar" @reset-password-mode="resetPasswordStart" @enable-auto-lock="enableAutoLock"
             :is-sidebar-open=isSideBar :is-auto-lock="automaticallyCloseLock"></Sidebar>
    <h3> {{ h3txt1 }} <br> {{ h3txt2 }}
    </h3>
    <div class="frontButtonWrap">
      <button ref="enableWebcamButtonRef" class="webCamBtn" @click="enableCam" id="webcamButton"
              :class="isWebcamButtonGreyedOut ? buttonGreyedOutClass : buttonActiveClass">Start unlock
      </button>
      <button @click="isLockButtonGreyedOut ? '' : sendBleCommand('off')"
              :class="isLockButtonGreyedOut ? buttonGreyedOutClass : buttonActiveClass">Lock
      </button>
      <button class="buttonActive" @click="unlock">
        are you sure?
      </button>
    </div>
    <div class="canvasCont">
      <video ref="vidRef" id="webcam" :class="webcamRunning ? webcamShadowClass : ''" class="vid" autoplay playsinline>Video loading, hold on a little ...</video>
      <canvas ref="canvasElementRef" class="output_canvas" id="output_canvas" width="1280" height="720"
              style="position: absolute; left: 0; top: 0"></canvas>
      <div class="rightWebcamCont">
        <p>
          <span ref="gestureOutputRef" id="gesture_output" class="output">None <br></span>
          <span class="successTransition"
                :style="{display: 'block', position: 'relative', transform: `translateY(${isDoor ? '0' : '1500px'})`}">Current combination: <span>{{
              currentCombo
            }}</span>
          </span>
        </p>
        <button @click="clearCurrentLockCombo()" class="buttonActive cleanInputButton successTransition"
                :style="{display: 'block', position: 'relative', transform: `translateY(${isDoor ? '0' : '1500px'})`}">
          Clear
        </button>
        <p class="rightConnectionTxt successTransition" :style="{display: 'block', position: 'relative', transform: `translateY(${isDoor ? '0' : '1500px'})`}">{{connectionTxt}}</p>
      </div>
    </div>
    <p class="successTransition successTxt" :style="{transform: `translateY(${isDoor ? '0' : '500px'})`}">{{ success }}</p>
  </main>
</template>

<style lang="scss">
body, html {
  overflow-x: hidden;
  overflow-y: hidden;
  background-color: rgba(29, 29, 33, 1);
}

// selection

::selection {
  background: #8a02b5;
}

</style>
<style lang="scss" scoped>
.dNone {
  display: none;
}

main {
  display: flex;
  justify-content: center;
  flex-direction: column;
  align-items: center;

  padding-top: 5vw;

  .successTxt {
    margin-top: 4vh;
  }

  .successTransition {
    transition: transform 1146ms 1s linear(0.00, -0.00624, 0.0254, 0.0642, 0.103, 0.140, 0.176, 0.211, 0.243, 0.274, 0.305, 0.334, 0.361, 0.387, 0.413, 0.438, 0.461, 0.483, 0.505, 0.526, 0.545, 0.564, 0.582, 0.600, 0.617, 0.633, 0.648, 0.662, 0.676, 0.690, 0.703, 0.715, 0.727, 0.738, 0.749, 0.760, 0.769, 0.779, 0.788, 0.797, 0.805, 0.814, 0.821, 0.829, 0.836, 0.843, 0.849, 0.856, 0.861, 0.867, 0.873, 0.878, 0.883, 0.888, 0.893, 0.897, 0.901, 0.905, 0.909, 0.913, 0.917, 0.920, 0.924, 0.927, 0.930, 0.933, 0.936, 0.938, 0.941, 0.943, 0.946, 0.948, 0.950, 0.952, 0.954, 0.956, 0.958, 0.960, 0.961, 0.963, 0.964, 0.966, 0.967, 0.969, 0.970, 0.971, 0.972, 0.974, 0.975, 0.976, 0.977, 0.978, 0.979, 0.979, 0.980, 0.981, 0.982, 0.983, 0.983, 0.984, 0.985, 0.985, 0.986, 0.987, 0.987, 0.988, 0.988, 0.989, 0.989, 0.990, 0.990, 0.990, 0.991, 0.991, 0.992, 0.992, 0.992, 0.993, 0.993, 0.993, 0.993, 0.994, 0.994, 0.994, 0.995, 0.995, 0.995, 0.995, 0.995, 0.996, 0.996, 0.996, 0.996, 0.996, 0.996, 0.997, 0.997, 0.997, 0.997, 0.997, 0.997, 0.997, 0.997, 0.998, 0.998, 0.998, 0.998, 0.998, 0.998, 0.998, 0.998, 0.998, 0.998, 0.998, 0.998, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 1.00);
  }

  h3, p {
    font-size: 1rem;
    color: white;
    font-family: Comfortaa, sans-serif;
    font-weight: normal;
    text-align: center;

    hyphens: auto;

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

    .rightWebcamCont {
      display: flex;
      flex-direction: column;

      align-items: center;

      p {
        width: 480px;
      }

      .rightConnectionTxt {
        margin-top: 11vw;
      }

    }
  }
}

.frontButtonWrap {
  display: flex;
  flex-direction: row;
  justify-content: center;
  width: 100%;

  gap: 5vw;
}

button {
  position: relative;
  margin-top: 2vh;
  z-index: 999;
  padding: 0.6vw 1vw;

  color: white;
  font-family: 'Comfortaa', sans-serif;
  font-size: 1rem;
  border: 3px solid #000000;

  // #5301bf looks so good, but a little not in theme
  //mix-blend-mode: hard-light;

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

.buttonActive:hover {
  transform: translate(3px, 3px);
  box-shadow: 1px 1px 0 black;
  background-color: #9e04da;
  color: #000000;
}

.buttonActive {
  background-color: #8a00bf;
  box-shadow: 4px 4px 0 black, 0 1px 0 black, 1px 2px 0 black, 2px 3px 0 black;
}

.buttonGreyedOut {
  background-color: #717171;
  box-shadow: 4px 4px 0 black, 0 1px 0 black, 1px 2px 0 black, 2px 3px 0 black;

}

.buttonGreyedOut:hover {
  background-color: #717171;

}

.settings-button {
  all: unset;
  cursor: pointer;

  right: 1.5vw;
  top: 3vw;
  position: absolute;
  z-index: 3;
}

.settings-button::after {
  all: unset;
}

.settings-svg {
  position: relative;
  width: 2vw;
  height: 2vw;
  z-index: 1;

}

.settings-svg::after {
  all: unset;
}

body {
  font-family: roboto, serif;
  margin: 2em;
  color: #3d3d3d;
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
  height: 360px;
  width: 480px;
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
  padding: 0;
  margin: 0;
  // video width and height
  width: 480px;
  height: 360px;
  transform: rotateY(180deg);
}

.webcamShadow {
  // 0 idea why the x axis is inverted here unlike on the buttons
  border: 3px solid #000000;
  box-shadow: -6px 4px 0 black, 0 2px 0 black, -3px 2px 0 black, -2px 3px 0 black;
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