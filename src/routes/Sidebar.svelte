<script lang="ts">
    import Cross from "$lib/Cross.svelte";

    let {isSidebar: isSidebar = $bindable(false), isAutoLock = $bindable(false), getSetting, setSetting, automaticallyCloseLock} = $props();

    $inspect(isSidebar)

    const clickOnBurger = () => {
        isSidebar = false;
    };

    const enableAutoLock = async () => {
        automaticallyCloseLock = !automaticallyCloseLock;

        await setSetting("autoLock", automaticallyCloseLock);
    }

    const resetPasswordMode = () => {return true};

    const isOpen = $derived(isSidebar);

    const handleLightboxClick = () => {
        clickOnBurger();
    }

    // i18n
    import { m } from '$lib/paraglide/messages.js';
    import { setLocale } from '$lib/paraglide/runtime';


</script>

<div class={`light-box ${isOpen ? 'open-opacity' : 'close-opacity'}`} role="button" tabindex="0" onclick={handleLightboxClick} onkeydown={(e) => {if (e.key==="Enter") {enableAutoLock()}}}></div>
<div style={`transform: translateX(${isOpen ? '-23.28vw' : '1vw'})`} class="sidebar-wrap">
    <ul class={`sidebar ${isOpen ? 'show-up' : ''}`}>
        <li class="cross-slice">
            <button onclick={clickOnBurger}>
                <Cross></Cross>
            </button>
        </li>
        <li class="slice one-slice">
            <div>
                <button onclick={resetPasswordMode}>
                    {m.sidebar_1()}
                </button>
            </div>
        </li>
        <li class="slice one-slice">
            <div class="slice-cont" onclick={enableAutoLock} role="button" tabindex="0" onkeydown={(e) => {if (e.key==="Enter") {enableAutoLock()}}}>
                <div class="check-box" aria-checked={isAutoLock} role="checkbox">
                    <img src={isAutoLock ? "/img/checkmarkOn.png" : "/img/checkmark.png"} alt="checkmark" class="check-box-img"/>
                </div>
                <p>{m.sidebar_2()}</p>
            </div>
        </li>
        <li class="slice bottom-slice">
            <div style="position: fixed">
                <button onclick={() => setLocale('en')}>en</button>
                <button onclick={() => setLocale('uk')}>uk</button>
            </div>
        </li>
        <li class="slice">
            <div class="slice-cont">
                <p>{m.sidebar_99()}<br>☝️ 👍 👎 ✌️ ✊ 👋 🤟</p>
            </div>
        </li>
    </ul>
</div>

<style>
    .light-box {
        background: #141417;
        transition: opacity 0.3s ease;

        position: fixed;
        z-index: 9999;
        top: 0;
        left: 0;

        width: 100vw;
        height: 100vh;
    }

    .close-opacity {
        pointer-events: none;

        opacity: 0;
    }

    .open-opacity {
        opacity: 0.6;
    }

    .sidebar-wrap {
        transition: transform 200ms cubic-bezier(0.78, 0, 0.22, 1);

        display: flex;
        justify-content: right;

        position: absolute;
        top: 0;
        right: 0;

        z-index: 99999;

        ul, li {
            margin-top: 0;
            margin-left: 0;
            padding: 0;
            user-select: none;
        }

    .sidebar {
        overflow-x: hidden;

        background: #1d1d21;
        border-left: #ffffff 1px solid;


        z-index: 10000;

        position: fixed;
        width: 18vw;
        height: 100vh;

        transform: translateX(23.28vw);

        transition: transform linear(0, 0.409 5.7%, 0.68 10.7%, 0.838 15.5%, 0.877 17.9%, 0.89 20.3%, 0.883 22.1%, 0.862 24%, 0.775 28.1%, 0.103 46.8%, -0.03 51.9%, -0.11 56.9%, -0.142 61.2%, -0.143 65.9%, -0.124 70.2%, -0.021 86%, -0.004 92.1%, 0);

        display: flex;
        flex-direction: column;

        li {
            list-style: none;

            color: white;
            font-family: Montserrat, sans-serif;
            font-weight: 600;
            font-size: 1.2rem;

            padding: 1vw;

            a {
                color: black;
                text-decoration: none;
            }

            transition: 0.3s;
        }

        .slice {
            .slice-cont {
                display: flex;
                flex-direction: row;
            }
        }

        .one-slice {
            cursor: pointer;
            div {
                p {
                    margin: 0;
                    transition: 0.3s;

                }
                p:hover {
                    color: #9e04da;
                }
            }
        }

        .bottom-slice {
            margin-top: auto;

            div {
                p {
                    user-select: text;
                }
            }
        }

        .cross-slice {
            display: flex;
            padding: 4%;
            height: 3vh;


            & button {
                all: unset;
                cursor: pointer;
                & svg {
                    cursor: pointer;
                }
            }
        }

        .sideSlice {
            border-bottom: 3px solid black;

            display: flex;
            align-items: center;
            height: 10.81vh;

            a {
                margin-left: 2vw;
            }
        }
    }

        .show-up {
            margin-right: 0;
        }
    }

    .check-box {
        display: flex;
        justify-content: center;
        align-items: center;

        margin-top: -0.5vw;

        padding-right: 0.4vw;

        aspect-ratio: 1;
        width: 3vw;
        height: 3vw;
        background: #1d1d21;


        .check-box-img {
            width: 100%;
            height: 100%;
            object-fit: cover;
        }
    }


</style>