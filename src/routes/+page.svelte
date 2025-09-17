<script lang="ts">
    import Sidebar from "./Sidebar.svelte";
    import type {
        GestureRecognizerResult
    } from "@mediapipe/tasks-vision";
    import {
        GestureRecognizer,
        FilesetResolver,
        DrawingUtils
    } from "@mediapipe/tasks-vision"
    import {
        warn,
        info,
    } from '@tauri-apps/plugin-log';
    // @ts-ignore (it exists)
    import {load} from '@tauri-apps/plugin-store';
    import type {
        BleDevice
    } from '@mnlphlp/plugin-blec'
    import {
        getConnectionUpdates,
        startScan,
        sendString,
        connect,
        disconnect,
        getScanningUpdates
    } from '@mnlphlp/plugin-blec'
    import {onMount} from "svelte";

    //
    // Powered by obscene amounts of caffeine
    //
    // ⢀⡐⡀⠀⠀⠀⣠⣿⣿⣿⣿⣿⡀⠀⠀⠀⠀⠀⠀⠀
    // ⣾⣄⡀⠀⠀⣿⣿⣿⣀⣿⣿⣿⢐⢀⠀⠀⠀⠀⠀⠀
    // ⠸⣿⣿⣿⣿⣿⣿⣿⣾⣿⣿⣿⣬⣡⠀⠀⠀⠀⠀⠀
    // ⠀⠘⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣧⣦⠀⠀⠀
    // ⠀⠈⣤⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡀
    // ⣤⣤⣿⣿⣿⣿⣿⣿⣿⣧⣷⢺⣿⣿⣿⣿⣿⣿⡀⢷
    // ⠀⠀⠂⢀⠿⢬⣿⣿⣿⠃⣄⣇⣴⣧⢿⣦⠀⠈⠀⠀
    // ⠀⠠⣿⣿⣿⣿⣿⣿⣿⢠⣸⣿⣿⣿⣿⣀⠀⠀⠀⠀
    // ⠀⠀⢷⣷⣿⣿⣿⣿⣿⣾⢈⡀⡏⣿⣿⣿⣿⢸⠀⠀
    // ⠀⠀⠀⠀⣷⣿⣿⣿⣿⢄⣷⢀⣧⣿⣿⣿⣿⠐⠀⠀
    // ⠀⠀⠀⠀⠀⠀⢿⣿⠀⠂⢹⢁⣿⠤⣿⣿⢁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀

    // --------------------------------------
    // === CONFIG ===
    // --------------------------------------

    // Password length. Let's imagine for a second this is all encrypted and there's a key generated for every time a password is made
    // 7^6 is already 117649 combinations which is basically unpickable by hand if it were used for a real door. In the future it could be
    // secured by just securing the password and moving all the security logic to obfuscated rust and sending the ble requests
    // with ECDH. Which would be incredibly secure.
    const PASSWORD_LENGTH = 6;

    // Delay between each gesture recognition, to give the user a little bit of time to breathe
    // Can be offset by GestureRecognizer taking a while to do its thing on a low-end machine I imagine
    const DELAY_BETWEEN_INPUTS = 550;

    // Yes.
    // TODO: fix
    const DO_RICKROLL = false;

    // Actual pairing is planned for the future. For now just this option.
    const CHARACTERISTIC_UUID = "beb5483e-36e1-4688-b7f5-ea07361b26a8";


    // --------------------------------------
    // === CONFIG END ===
    // --------------------------------------

    // --------------------------------------
    // === OPTIONS (available in-app) ===
    // --------------------------------------

    // string because it becomes 'nothingYet' if it was undeclared because clarity
    let automaticallyCloseLock = $state<boolean | string | undefined>(false);

    // --------------------------------------
    // === OPTIONS END ===
    // --------------------------------------

    // --------------------------------------
    // === HERE'S WHERE THE MAGIC HAPPENS ===
    // --------------------------------------

    // if you want to plug in something here, this is the place

    let scanning = $state<Boolean>(false);
    let devices = $state<BleDevice[]>([]);
    let connectedToLock = $state<Boolean>(false);
    let connected = $state<Boolean>(false);

    // This thing makes sure the "currently nothing found in the vicinity, is Bluetooth on? 🔗"
    // does not show up in initialization, only the first time tho because lazy
    let avoidConfusion = $state(true);

    // !!! may break 1
    $effect(() => {
        (async () => {
            // enables the lock button
            isLockButtonGreyedOut = false;

            if (devices.length === 0) {
                if (avoidConfusion) {
                    !avoidConfusion;
                } else {
                    success = m.success_nothing();
                }
            }


            let j = $state(0);
            for (const device of devices) {
                if (device.name === "ESP32_LED_Control") {
                    await connect(device.address, () => info('disconnected'));
                    connectedToLock = true;
                }
                if (j === devices.length - 1) {
                    success = m.success_no_lock();
                }
                j += 1;
            }

            // can also send from js
            await sendString(CHARACTERISTIC_UUID, "on");

            // open sesame
            success = m.success_hurray();

            // if autolock, lock in 30 seconds
            if (automaticallyCloseLock) {
                setTimeout(async () => {
                    await sendString(CHARACTERISTIC_UUID, "off");
                }, 30000)
            }
        })
    })

    async function disconnectFromLock() {
        // disconnecting past connection in case it was connected
        if (connected) {
            await disconnect()
        }
    }

    let session = $state(false);

    async function unlock() {
        success = m.success_looking();
        session = true;

        devices = [];

        success = m.success_attempting();


        setTimeout(async () => {
            if (!connected) {
                success = m.success_no_device();
            }
        }, 10000)
        await startScan((dv: BleDevice[]) => {
            devices = dv;
        }, 10000);
    }

    async function block() {
        await info('wrong, go to jail!')

        // you shall not pass (closes the lock)
        await sendString(CHARACTERISTIC_UUID, "off");

        if (DO_RICKROLL) {
            window.open("https://www.youtube.com/watch?v=xvFZjo5PgG0&ab_channel=Duran", "_blank");
        }

        success = m.success();
    }

    // --------------------------------------
    // === HERE'S WHERE THE MAGIC ENDS ===
    // --------------------------------------


    let store: any = null;

    const setSetting = async (key: string, value: any) => {
        await store.set(key, {value: value});

        await store.save();
    }

    const getSetting: any = async (key: string) => {
        const setting = await store.get(key) ?? 'nothingYet';

        return setting.value;
    };

    let isLockButtonGreyedOut = $state(true);
    let isWebcamButtonGreyedOut = $state(false);

    let connectionTxt = $state(m.process_4());

    let gestureRecognizer: GestureRecognizer;
    let runningMode = "IMAGE";

    const createGestureRecognizer = async () => {
        const vision = await FilesetResolver.forVisionTasks("https://cdn.jsdelivr.net/npm/@mediapipe/tasks-vision@0.10.3/wasm") // maybe replace locally?

        gestureRecognizer = await GestureRecognizer.createFromOptions(vision, {
            baseOptions: {
                modelAssetPath: "https://storage.googleapis.com/mediapipe-models/gesture_recognizer/gesture_recognizer/float16/1/gesture_recognizer.task",
                delegate: "GPU"
            },
            // typescript being funny here, RunningMode type is basically a string.
            // @ts-ignore
            runningMode: runningMode
        });
    };

    // using refs here, turning them into html objects later on

    let vidRef = $state<HTMLVideoElement | null>(null);
    let canvasElementRef = $state<HTMLCanvasElement | null>(null);
    let gestureOutputRef = $state<HTMLElement | null>(null);
    let isDoor = $state<boolean>(false);

    let enableWebcamButtonRef = $state<HTMLElement | null>(null)
    let webcamRunning: Boolean = $state(false);
    let webcamRanOnce: Boolean = $state(false);

    let webcamOnceIx = 0;
    $effect(() => {
        webcamRunning = webcamRunning;

        if (webcamOnceIx === 1) {
            webcamRanOnce = true;
        }
        webcamOnceIx++;
    })

    const videoHeight = "360px";
    const videoWidth = "480px";

    function hasGetUserMedia() {
        return !!(navigator.mediaDevices && navigator.mediaDevices.getUserMedia);
    }

    function enableCam() {
        info('enabling webcam')
        isWebCamOn = true;
        isDoor = true;

        const video: HTMLVideoElement | null = vidRef;
        if (!video) {
            console.warn("Video not found! Something went very wrong (?). Try a restart.");
            return;
        }

        const enableWebcamButton: HTMLElement | null = enableWebcamButtonRef;

        if
        (!hasGetUserMedia()) {
            warn("your browser probably doesn't support getUserMedia");
            return;
        }

        if (!gestureRecognizer) {
            warn("hold on, it's loading");
            return;
        }

        if (!enableWebcamButton) {
            console.warn("Webcam button didn't load. Something went very wrong (?). Try a restart.");
            return;
        }

        if (webcamRunning === true) {
            webcamRunning = false;
            enableWebcamButton.innerText = m.button_1_enable();
        } else {
            webcamRunning = true;
            enableWebcamButton.innerText = m.button_1_disable();
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

    let ifRun = $state(true)

    // MAKE SURE TO INCLUDE A SPACE AT THE END 3 AM ME
    let password = $state("Thumb_Up Thumb_Down Victory Closed_Fist Thumb_Up Victory ")
    // const password = $state("Thumb_Up Thumb_Down Victory ")
    let success = $state(m.success() + " 👍 👎 ✌️ ✊ 👍 ✌️ ")

    let currentPassword = $state("")

    // this is the global password iterator
    let passIter = $state(0)
    let oldCategoryName = $state("")
    let lastTime = 0;

    let isResetPasswordMode = $state(false);
    let h3txt1 = $state(m.main1())
    let h3txt2 = $state(m.main2())

    let isWebCamOn = $state(false)
    let currentCombo = $state("🧈")


    function throttle(func: any, delay: number, a: any) {
        const now = Date.now();
        if (now - lastTime >= delay) {
            func(a);
            lastTime = now;
        }
    }

    let emoji = $state("");

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
        // I think this gesture is Metal and not ILY but sure
        if (categoryName === "ILoveYou") {
            categoryName = "🤟";
        }
        return categoryName;
    }

    async function reEnable() {
        setTimeout(() => {
            ifRun = true;
            if (webcamRunning) {
                predictWebcam();
            }
        }, 1500)
    }

    const clearCurrentLockCombo = async () => {
        passIter = 0;
        oldCategoryName = "";
        currentPassword = "";
        currentCombo = "";
    }

    async function matchPassword(categoryName: string) {
        if (passIter === 0) {
            currentPassword = "";
            currentCombo = "";
        }

        if (categoryName === oldCategoryName || categoryName === "None") {
            return
        }

        // might as well add the emoji to the current thing with the same function cause why not
        currentCombo += emoji;

        oldCategoryName = categoryName;

        currentPassword += categoryName;
        currentPassword += " ";

        if (passIter === PASSWORD_LENGTH - 1 && password === "blank") {
            password = currentPassword;

            h3txt1 = m.main1();
            h3txt2 = m.main2();

            await setSetting('password', password)

            success = m.success() + convertToEmoji(password);
        }

        if (passIter === PASSWORD_LENGTH - 1 && currentPassword === password) {
            await clearCurrentLockCombo();
            await disconnectFromLock();
            await unlock();

            if (isResetPasswordMode) {
                password = "blank";
                isResetPasswordMode = false;
                h3txt1 = m.main1_now_input();
                h3txt2 = m.main2_from_the_sidebar();
            }

            return;
        } else if ((passIter === PASSWORD_LENGTH - 1 && currentPassword !== password) || passIter === PASSWORD_LENGTH) {
            await clearCurrentLockCombo();
            await block();

            ifRun = false;
            await reEnable()

            return;
        }

        passIter++;
    }

    let lastVideoTime = -1;

    let results: GestureRecognizerResult | undefined = undefined;

    async function predictWebcam() {
        if (ifRun == false) {
            return;
        }

        const video: HTMLVideoElement | null = vidRef;
        if (!video) {
            console.warn("Video not found! Something went very wrong (?). Try a restart.");
            return;
        }

        const canvasElement: HTMLCanvasElement | null = canvasElementRef;
        if (!canvasElement) {
            console.warn("Canvas not found! Something went very wrong (?). Try a restart.");
            return;
        }
        const gestureOutput: HTMLElement | null = gestureOutputRef;
        const canvasCtx = canvasElement.getContext("2d");
        if (!canvasCtx) {
            console.warn("Canvas context not found! Something went very wrong (?). Try a restart.");
            return;
        }


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

        if (!results) {
            console.warn("Prediction result is undefined as of right now. Trying again.");
            canvasCtx.restore();
            if (webcamRunning === true) {
                window.requestAnimationFrame(predictWebcam);
            }

            return;
        }
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
            gestureOutput!.style.display = "block";
            gestureOutput!.style.width = videoWidth;
            let categoryName = results.gestures[0][0].categoryName;
            const categoryNameStored = categoryName;

            emoji = matchEmoji(categoryName);
            throttle(matchPassword, DELAY_BETWEEN_INPUTS, categoryNameStored);

            // @ts-ignore useful debug
            const categoryScore = parseFloat(
                (results.gestures[0][0].score * 100).toString()
            ).toFixed(2);

            // @ts-ignore useful debug
            const handedness = results.handedness[0][0].displayName;
            gestureOutput!.innerText = `${emoji}`;
            // gestureOutput.innerText = `GestureRecognizer: ${categoryName}\n Confidence: ${categoryScore} %\n Handedness: ${handedness}` ;
        } else {
            gestureOutput!.style.display = "block";
        }
        // Call this function again to keep predicting whenever the browser is ready.
        if (webcamRunning === true) {
            window.requestAnimationFrame(predictWebcam);
        }
    }

    onMount(async () => {
        console.log("wait");
        await createGestureRecognizer();

        store = await load('store.json', { autoSave: false });
        console.log("stoishing", store);

        // Getters for settings on load
        automaticallyCloseLock = await getSetting("autoLock");
        if (automaticallyCloseLock === "nothingYet") {
            console.warn('Setting default value for autoLock');
            automaticallyCloseLock = true;
        }


        const passwordFromStorage = await getSetting('password');
        if (passwordFromStorage !== undefined && passwordFromStorage !== "nothingYet") {
            password = passwordFromStorage;
        }

        await getConnectionUpdates((state) => connected = state)
        await getScanningUpdates((state) => {
            scanning = state
        })

        setInterval(() => {
            if (connected) {
                connectionTxt = m.process_5();
            } else {
                connectionTxt = m.process_3();
            }
        }, 1000)
    })


    let isSideBar = $state(false);

    function openSidebar() {
        isSideBar = true;
    }

    $effect(() => {
        if (isResetPasswordMode) {
            h3txt1 = m.main1_now_input();
            h3txt2 = m.main2_from_the_sidebar();
        }
    })

    // i18n
    import {m} from '$lib/paraglide/messages.js';
    import {setLocale} from '$lib/paraglide/runtime';
</script>
<main class="global-cont">
    <div onclick={openSidebar} onkeydown={(e: KeyboardEvent) => {if (e.key === "Enter") {openSidebar()}}} role="button"
         tabindex="0" class="settings-button">
        <svg class="settings-svg" xmlns="http://www.w3.org/2000/svg" fill="white" viewBox="0 0 448 512">
            <!--!Font Awesome Free 6.7.2 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2025 Fonticons, Inc.-->
            <path
                    d="M0 96C0 78.3 14.3 64 32 64l384 0c17.7 0 32 14.3 32 32s-14.3 32-32 32L32 128C14.3 128 0 113.7 0 96zM0 256c0-17.7 14.3-32 32-32l384 0c17.7 0 32 14.3 32 32s-14.3 32-32 32L32 288c-17.7 0-32-14.3-32-32zM448 416c0 17.7-14.3 32-32 32L32 448c-17.7 0-32-14.3-32-32s14.3-32 32-32l384 0c17.7 0 32 14.3 32 32z"/>
        </svg>
    </div>
    <Sidebar bind:isSidebar={isSideBar} {getSetting} {setSetting} {session}
             bind:automaticallyCloseLock={automaticallyCloseLock} bind:isResetPasswordMode={isResetPasswordMode}></Sidebar>
    <h3> {h3txt1} <br> {h3txt2}</h3>
        <div class="front-button-wrap">
            <button bind:this={enableWebcamButtonRef} onclick={enableCam} id="webcam-button"
                    class={`web-cam-btn ${isWebcamButtonGreyedOut ? "button-greyed-out" : "button-active"}`}>{m.button_1()}</button>
            <button onclick={() => {isLockButtonGreyedOut ? '' : sendString(CHARACTERISTIC_UUID, 'off')}}
                    class={isLockButtonGreyedOut ? "button-greyed-out" : "button-active"}>{m.button_2()}
            </button>
            <button onclick={() => {if(isLockButtonGreyedOut){sendString(CHARACTERISTIC_UUID, 'on')}}}
                    class={isLockButtonGreyedOut ? "button-greyed-out" : "button-active"}>{m.button_3()}
            </button>
        </div>
        <div class={`canvas-cont ${webcamRanOnce ? "" : "d-none"}`}>
            <video bind:this={vidRef} id="webcam" class={`vid ${webcamRanOnce ? "webcam-shadow" : ''}`} muted autoplay
                   playsinline>
                Video loading, hold on a little ...
            </video>
            <canvas bind:this={canvasElementRef} class="output-canvas" id="output-canvas" width="1280" height="720"
                    style="position: absolute; left: 0; top: 0"></canvas>
            <div class="right-webcam-cont">
                <p>
                    <span bind:this={gestureOutputRef} id="gesture-output" class="output">{m.process_1()} <br></span>
                    <span class="success-transition"
                          style={`display: block, position: relative, transform: translateY(${isDoor ? '0' : '1500px'})`}>{m.process_2()} <span>{
                        currentCombo
                    }</span>
          </span>
                </p>
                <button onclick={clearCurrentLockCombo} class="button-active clean-input-button success-transition"
                        style={`display: block, position: relative, transform: translateY(${isDoor ? '0' : '1500px'})`}>
                    {m.clear()}
                </button>
                <p class="right-connection-txt success-transition"
                   style={`display: block, position: relative, transform: translateY(${isDoor ? '0' : '1500px'})`}>
                    {connectionTxt}</p>
            </div>
        </div>
        <p class={`success-transition success-txt ${webcamRanOnce ? '' : "d-none"}`} style={`transform: translateY(${isDoor ? '0' : '500px'})}`}>{
            success
        }</p>
</main>

<style>
    @import url('https://fonts.googleapis.com/css2?family=Comfortaa:wght@300..700&display=swap');

    :global {
        body, html {
            overflow-x: hidden;
            overflow-y: hidden;
            margin: 0;
            background-color: rgba(29, 29, 33, 1);
        }

        /*selection*/

        ::selection {
            background: #8a02b5;
        }

        /*//*/

        .d-none {
            display: none !important;
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

            /*// #5301bf looks so good, but a little not in theme*/
            /*//mix-blend-mode: hard-light;*/

            transition: all 0.2s ease-in-out;
        }

        button::after {
            content: "";
            pointer-events: none;

            /*//background: repeating-linear-gradient(75deg,*/
            /*//    rgb(0, 0, 0, 0.2) 0%,*/
            /*//    rgb(0, 0, 0, 0.2) 0.001px,*/
            /*//    rgb(0, 0, 0, 0) 5px,*/
            /*//    rgb(0, 0, 0, 0) 10px*/
            /*//);*/
            opacity: 0.8;

            position: absolute;
            z-index: -1;
            inset: 0;
            width: 100%;
            height: 100%;
        }

        .button-active:hover {
            transform: translate(3px, 3px);
            box-shadow: 1px 1px 0 black;
            background-color: #9e04da;
            color: #000000;
        }

        .button-active {
            background-color: #8a00bf;
            box-shadow: 4px 4px 0 black, 0 1px 0 black, 1px 2px 0 black, 2px 3px 0 black;
        }

        .button-greyed-out {
            background-color: #717171;
            box-shadow: 4px 4px 0 black, 0 1px 0 black, 1px 2px 0 black, 2px 3px 0 black;

        }

        .button-greyed-out:hover {
            background-color: #717171;

        }
    }

    main {
        display: flex;
        justify-content: center;
        flex-direction: column;
        align-items: center;

        padding-top: 3vw;

        .success-txt {
            margin-top: 4vh;
        }

        .success-transition {
            /*webstorm wake up*/
            /*noinspection CssInvalidFunction*/
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

        .web-cam-btn {
            cursor: pointer;
        }

        .canvas-cont {
            display: flex;
            justify-content: left;
            position: relative;

            flex-wrap: wrap;

            margin-top: 4vw;

            .right-webcam-cont {
                display: flex;
                flex-direction: column;

                align-items: center;

                p {
                    width: 480px;
                }

                .right-connection-txt {
                    margin-top: 11vw;
                }

            }
        }
    }

    .front-button-wrap {
        display: flex;
        flex-direction: row;

        flex-wrap: wrap;

        justify-content: center;
        width: 100%;

        gap: 5vw;
        padding-bottom: 1rem;
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

    :global {
        body {
            font-family: roboto, serif;
            margin: 2em;
            color: #3d3d3d;
        }
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
        /*//width: 48%;*/
        margin: 2% 1%;
        cursor: pointer;
        /*//min-height: 500px;*/
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

    .output-canvas {
        padding: 0;
        margin: 0;
        /*// video width and height*/
        width: 480px;
        height: 360px;
        transform: rotateY(180deg);
    }

    .webcam-shadow {
        /*// 0 idea why the x axis is inverted here unlike on the buttons*/
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