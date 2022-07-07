<template>
  <div class="content">
    <ul>
      <li
        @click="change(item.id, index)"
        v-for="(item, index) in data.categories"
        :key="index"
        :style="
          data.currentCategoryIndex == index ? 'background: #1bc3d2;font-weight: bold;color:#fff' : ''
        "
      >
        {{ item.name }}
      </li>
    </ul>
    <div class="list">
      <div>
        <img
          class="cover"
          src="https://img1.kuwo.cn/wmvpic/324/78/3/4093527818.jpg"
          alt=""
        />
        <div>
          <span class="title">The Lighthouse Keeper</span>
        </div>
        <div>
          <span class="author">Sam Smith</span>
        </div>
      </div>
      <div>
        <img
          class="cover"
          src="https://img1.kuwo.cn/wmvpic/324/78/3/4093527818.jpg"
          alt=""
        />
        <div>
          <span class="title">The Lighthouse Keeper</span>
        </div>
        <div>
          <span class="author">Sam Smith</span>
        </div>
      </div>
      <div>
        <img
          class="cover"
          src="https://img1.kuwo.cn/wmvpic/324/78/3/4093527818.jpg"
          alt=""
        />
        <div>
          <span class="title">The Lighthouse Keeper</span>
        </div>
        <div>
          <span class="author">Sam Smith</span>
        </div>
      </div>
      <div>
        <img
          class="cover"
          src="https://img1.kuwo.cn/wmvpic/324/78/3/4093527818.jpg"
          alt=""
        />
        <div>
          <span class="title">The Lighthouse Keeper</span>
        </div>
        <div>
          <span class="author">Sam Smith</span>
        </div>
      </div>
      <div>
        <img
          class="cover"
          src="https://img1.kuwo.cn/wmvpic/324/78/3/4093527818.jpg"
          alt=""
        />
        <div>
          <span class="title">The Lighthouse Keeper</span>
        </div>
        <div>
          <span class="author">Sam Smith</span>
        </div>
      </div>
    </div>
    <div class="list">aa</div>
  </div>
</template>

<script setup>
import { reactive, onMounted } from "vue";
import { fetch } from "@tauri-apps/api/http";
const data = reactive({
  mvList: [],
  categories: [],
  currentCategoryIndex: 0,
});

const change = (id, index) => {
  data.currentCategoryIndex = index;
};

// const getMVList = () => {
//   fetch("http://localhost:8000/get_mv_list", {
//     method: "GET",
//     timeout: 30,
//   }).then((res) => {
//     data.categories = res.data;
//   });
// };

onMounted(() => {
  fetch("http://localhost:8000/get_categories", {
    method: "GET",
    timeout: 30,
  }).then((res) => {
    data.categories = res.data;
  });
});
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style lang="less" scoped>
.content {
  background: #191927;
}

ul {
  list-style-type: none;
  margin: 40px;
  display: flex;
  font-size: 15px;
  li {
    padding: 0 14px;
    height: 28px;
    border-radius: 28px;
    line-height: 28px;
    cursor: pointer;
    color: #a8a5ab;
  }
}

.list {
  margin: 40px;
  display: flex;
  flex-wrap: wrap;
  > div {
    display: flex;
    flex-direction: column;
    width: 22%;
    margin-right: 4%;
    &:nth-child(4n) {
      margin-right: 0;
    }
    span {
      cursor: pointer;
      display: inline-block;
    }

    .title {
      width: 100%;
      font-size: 18px;
      font-weight: 400;
      line-height: 25px;
      margin-top: 12px;
      text-overflow: ellipsis;
      overflow: hidden;
      white-space: nowrap;
      color: #fff;
      &:hover {
        font-weight: 600;
      }
    }
    .author {
      font-size: 14px;
      font-family: "Microsoft YaHei", "Helvetica Neue", Helvetica, STHeiTi,
        sans-serif;
      color: #999;
      font-weight: 300;
      margin-top: 8px;
      margin-bottom: 26px;
      text-overflow: ellipsis;
      overflow: hidden;
      white-space: nowrap;
      &:hover {
        color: rgb(182, 182, 182);
      }
    }
  }
}

.cover {
  cursor: pointer;
  width: 100%;
  height: 150px;
  border-radius: 4px;
}
</style>
