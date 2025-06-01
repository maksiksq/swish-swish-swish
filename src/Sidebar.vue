<script setup>
import {ref, computed, onMounted, watch} from "vue";
import Cross from "./Cross.vue";

import checkmark from './assets/checkmark.png'
import checkmarkOn from './assets/checkmark-on.png'

// mostly copied this from one my previous projects because lazy so the code is a bit sloppy

const showUpClass = ref('show-up');

const props = defineProps({
  isSidebarOpen: Boolean,
  isAutoLock: Boolean
})

const isOpen = computed(() => props.isSidebarOpen);

</script>

<template>
  <div :class="[isOpen ? 'open-opacity' : 'close-opacity']" class="light-box" v-on:click="$emit('clickOnBurger'); setSetting('autoLock', !getSetting('autoLock'))"></div>
  <div :style="{transform: `translateX(${isOpen ? '-23.28vw' : '1vw'})`}" class="sidebar-wrap">
    <ul :class="[isOpen ? showUpClass : '']" class="sidebar">
      <li class="crossSlice">
        <Cross v-on:click="$emit('clickOnBurger')"></Cross>
      </li>
      <li class="slice oneSlice">
        <div>
          <p v-on:click="$emit('resetPasswordMode')">
            Reset password
          </p>
        </div>
      </li>
      <li class="slice oneSlice">
        <div class="sliceCont" @click="$emit('enableAutoLock');">
          <div class="checkbox" role="checkbox">
            <img :src="props.isAutoLock ? checkmarkOn : checkmark" alt="checkmark" class="checkboxImg"/>
          </div>
          <p>Auto lock</p>
        </div>
      </li>
      <li class="slice oneSlice bottomSlice">
        <div class="sliceCont">
          <p>Available gestures:<br>☝️ 👍 👎 ✌️ ✊ 👋 🤟</p>
        </div>
      </li>
    </ul>
  </div>
</template>

<style scoped lang="scss">

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
  opacity: 0.3;
}

.sidebar-wrap {
  transition: transform 500ms ease-in-out;

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
//
  .sidebar {
    overflow-x: hidden;

    background: #1d1d21;
    border-left: #ffffff 1px solid;


    z-index: 10000;

    position: fixed;
    width: 23.28vw;
    height: 100vh;

    transform: translateX(23.28vw);

    transition: transform 1500ms linear(0.00, -0.00624, 0.0254, 0.0642, 0.103, 0.140, 0.176, 0.211, 0.243, 0.274, 0.305, 0.334, 0.361, 0.387, 0.413, 0.438, 0.461, 0.483, 0.505, 0.526, 0.545, 0.564, 0.582, 0.600, 0.617, 0.633, 0.648, 0.662, 0.676, 0.690, 0.703, 0.715, 0.727, 0.738, 0.749, 0.760, 0.769, 0.779, 0.788, 0.797, 0.805, 0.814, 0.821, 0.829, 0.836, 0.843, 0.849, 0.856, 0.861, 0.867, 0.873, 0.878, 0.883, 0.888, 0.893, 0.897, 0.901, 0.905, 0.909, 0.913, 0.917, 0.920, 0.924, 0.927, 0.930, 0.933, 0.936, 0.938, 0.941, 0.943, 0.946, 0.948, 0.950, 0.952, 0.954, 0.956, 0.958, 0.960, 0.961, 0.963, 0.964, 0.966, 0.967, 0.969, 0.970, 0.971, 0.972, 0.974, 0.975, 0.976, 0.977, 0.978, 0.979, 0.979, 0.980, 0.981, 0.982, 0.983, 0.983, 0.984, 0.985, 0.985, 0.986, 0.987, 0.987, 0.988, 0.988, 0.989, 0.989, 0.990, 0.990, 0.990, 0.991, 0.991, 0.992, 0.992, 0.992, 0.993, 0.993, 0.993, 0.993, 0.994, 0.994, 0.994, 0.995, 0.995, 0.995, 0.995, 0.995, 0.996, 0.996, 0.996, 0.996, 0.996, 0.996, 0.997, 0.997, 0.997, 0.997, 0.997, 0.997, 0.997, 0.997, 0.998, 0.998, 0.998, 0.998, 0.998, 0.998, 0.998, 0.998, 0.998, 0.998, 0.998, 0.998, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 0.999, 1.00);

    display: flex;
    flex-direction: column;

    li {
      list-style: none;

      color: white;
      font-family: Montserrat, sans-serif;
      font-weight: 600;
      font-size: 1.5rem;

      padding: 1vw;

      a {
        color: black;
        text-decoration: none;
      }

      transition: 0.3s;
    }

    .slice {
      .sliceCont {
        display: flex;
        flex-direction: row;
      }
    }

    .oneSlice {
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

    .bottomSlice {
      margin-top: auto;
    }

    .crossSlice {
      display: flex;
      padding: 4%;
      height: 3vh;

      svg {
        cursor: pointer;
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

.checkbox {
  display: flex;
  justify-content: center;
  align-items: center;

  margin-top: -0.5vw;

  padding-right: 0.4vw;

  aspect-ratio: 1;
  width: 3vw;
  height: 3vw;
  background: #1d1d21;

  .checkboxImg {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }
}

</style>