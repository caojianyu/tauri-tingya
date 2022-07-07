<template>
  <div class="panel">
    <div class="main">
      <div class="left">
        <div class="info">
          <img class="avatar" src="../assets/img/avatar.jpeg" alt="" />
          <div>
            <span class="nickname">{{ data.username }}</span>
            <!-- other -->
          </div>
        </div>
        <div class="menu">
          <div
            :class="data.currentIndex == index ? 'active' : ''"
            v-for="(item, index) in data.menus"
            :key="index"
          >
            <router-link @click="data.currentIndex = index" :to="item.path">
              <i class="iconfont icon" :class="item.icon"></i>{{ item.name }}
            </router-link>
          </div>
        </div>
      </div>
      <!-- view -->
      <router-view class="view" v-slot="{ Component }">
        <transition name="scale-slide">
          <component @change="chooseMusic" :is="Component" />
        </transition>
      </router-view>
    </div>
    <div class="footer">
      <div class="left">
        <img
          v-if="data.currentMusic.cover"
          class="cover"
          :src="data.currentMusic.cover"
          alt=""
        />
        <img v-else class="cover" src="../assets/img/music.webp" alt="" />
        <div>
          <span
            >{{ data.currentMusic.music_name }} -
            {{ data.currentMusic.author }}</span
          >
          <span>{{ data.currentMusic.describe }}</span>
        </div>
      </div>
      <div class="right">
        <div></div>
        <div class="control">
          <div class="function">
            <i class="iconfont icon-shangyishou"></i>
            <i
              v-if="data.isPlaying"
              @click="pause()"
              class="iconfont icon-07zanting click"
            ></i>
            <i
              v-else
              @click="play()"
              class="iconfont icon-shipinbofangshibofang click"
            ></i>
            <i class="iconfont icon-xiayishou"></i>
            <i
              @click="loop()"
              class="iconfont icon-repeat"
              :class="data.loop ? 'click' : ''"
            ></i>
          </div>

          <div class="progress-bar">
            <span>{{ format(data.currentTime) }}</span>
            <div ref="line" class="line" @click="skip($event)">
              <div
                class="progress"
                :style="'width:' + data.width + 'px;'"
              ></div>
            </div>
            <span>{{ format(data.duration) }}</span>
          </div>
        </div>
        <div class="other"></div>
      </div>
    </div>
    <audio ref="audio" :src="data.currentMusic.src"></audio>
  </div>
</template>
<script setup>
import { ref, reactive, onMounted, onUnmounted } from "vue";
import { toRaw } from "@vue/reactivity";
import { invoke } from "@tauri-apps/api";

const data = reactive({
  currentIndex: 0,
  menus: [
    {
      icon: "icon-shouyetianchong",
      name: "音乐台",
      path: "/",
    },
    {
      icon: "icon-paixingbang",
      name: "排行榜",
      path: "/rankingList",
    },
    {
      icon: "icon-boshiweb_bofangliang",
      name: "MV",
      path: "/musicVideo",
    },
    {
      icon: "icon-24gf-headphones",
      name: "个人电台",
      path: "/personalStation",
    },
    {
      icon: "icon-xiai",
      name: "我喜欢",
      path: "/ilike",
    },
  ],
  width: 0,
  duration: 0,
  currentTime: 0,
  isPlaying: false,
  loop: false,
  currentMusic: {},
  playTimer: null,
  username: "",
});

const skip = (e) => {
  data.width = e.offsetX;
  data.currentTime = audio.value.currentTime =
    (data.width / 400) * data.duration;
};

const audio = ref();

const play = () => {
  if (!data.currentMusic.src || !audio.value.duration) {
    return;
  }
  data.duration = audio.value.duration;
  data.isPlaying = true;
  audio.value.play();
  clear();
  data.playTimer = setInterval(() => {
    data.currentTime = audio.value.currentTime;
    data.width = (data.currentTime / data.duration) * 400;
    if (audio.value.ended || data.currentTime >= data.duration) {
      clear();
      data.isPlaying = false;
    }
  }, 1000);
};

const pause = () => {
  audio.value.pause();
  data.isPlaying = false;
  clear();
};

const loop = () => {
  data.loop = !data.loop;
  audio.value.loop = data.loop;
};

const format = (duration) => {
  let m = parseInt((duration / 60) % 60);
  m = m < 10 ? "0" + m : m;
  let s = parseInt(duration % 60);
  s = s < 10 ? "0" + s : s;
  return m + ":" + s;
};

const chooseMusic = (music) => {
  data.currentMusic = toRaw(music);
  pause();
  data.currentTime = 0;
  data.duration = 0;
  data.width = 0;
  audio.value.addEventListener("canplay", () => {
    play();
  });
};

const clear = () => {
  if (data.playTimer) {
    clearInterval(data.playTimer);
  }
};
onMounted(() => {
  invoke("get_username").then((username) => (data.username = username));
});

onUnmounted(() => {
  clear();
});
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style lang="less" scoped>
.panel {
  height: 100%;
}
.main {
  display: flex;
  height: calc(100% - 80px);
  .left {
    width: 260px;
    background: #1d1034;
    padding: 0 50px;
    flex-shrink: 0;
    .info {
      margin-top: 40px;
      display: flex;
      align-items: center;
      > div {
        .nickname {
          font-size: 22px;
          font-weight: 400;
          color: #fff;
          margin-left: 10px;
        }
      }
    }

    .menu {
      color: #a8a5ab;
      margin-top: 40px;
      font-size: 15px;
      font-family: SimHei;
      > div {
        height: 52px;
        display: flex;
        align-items: center;
        a {
          text-decoration: none;
          color: #a8a5ab;
          .icon {
            margin-right: 20px;
          }
        }
        .mv {
          font-family: "微软雅黑";
          font-size: 14px;
        }
      }

      .active {
        color: #fff;
        a {
          color: #fff;
        }
        font-weight: bold;
      }
    }
  }
}

.footer {
  height: 80px;
  background: #1e1e37;
  display: flex;
  align-items: center;
  .left {
    width: 260px;
    height: 100%;
    display: flex;
    align-items: center;
    font-family: SimHei;
    background: #211c3e;
    flex-shrink: 0;
    position: relative;
    padding: 0 20px;
    > div {
      display: flex;
      flex-direction: column;
      align-items: center;
      justify-content: space-around;
      margin-left: 20px;
      > span {
        width: 150px;
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
        &:first-of-type {
          color: #fff;
        }

        &:last-of-type {
          margin-top: 6px;
          font-size: 12px;
          color: #8f8e99;
        }
      }
    }
  }

  .right {
    width: 100%;
    height: 100%;
    display: flex;
    justify-content: space-between;
    background: #1e1e37;
    .control {
      display: flex;
      flex-direction: column;
      align-items: center;
      justify-content: space-evenly;
      .function {
        i {
          font-size: 24px;
          margin: 0 20px;
          color: #8f8f9b;
        }
        .click {
          color: #d2d2d7;
        }
      }
      .progress-bar {
        display: flex;
        align-items: center;
        font-size: 14px;
        color: #a9a9ae;
        .line {
          width: 400px;
          height: 6px;
          background: #1c1c31;
          border-radius: 6px;
          margin: 0 10px;

          .progress {
            height: 100%;
            background: #1bc3d2;
            border-radius: 6px;
          }
        }
      }
    }

    .other {
      background: #2a2e6d;
    }
  }
}

// center
.view {
  width: 100%;
  height: 100%;
}

// common
.avatar {
  width: 70px;
  height: 70px;
  border-radius: 50%;
}

.cover {
  width: 50px;
  height: 50px;
  border-radius: 4px;
}

// animation
.scale-slide-enter-active,
.scale-slide-leave-active {
  position: absolute;
  width: calc(100% - 260px);
  z-index: -1;
  transition: all 0.85s ease;
}

.scale-slide-enter-from {
  left: -100%;
}
.scale-slide-enter-to {
  left: 260px;
}
.scale-slide-leave-from {
  left: 260px;
  transform: scale(1);
}
.scale-slide-leave-to {
  left: 100%;
  transform: scale(0.8);
}
</style>
