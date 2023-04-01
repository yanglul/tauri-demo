<script setup lang="ts">
import { ref, reactive, inject  } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from '@tauri-apps/api/event';
import { fetch } from '@tauri-apps/api/http';
import ElementPlus from 'element-plus';
// import zhCn from 'node_modules/element-plus/dist/locale/zh-cn.mjs';
 
import { useRouter } from "vue-router";
const greetMsg = ref("");
const name = ref("");

const form = reactive({
  name: '',
 
  desc: '',
})

const onSubmit = () => {
  console.log('submit!')
}


async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsg.value = await invoke("greet", { name: name.value });
}



const unlisten = async () => {
  await invoke("init_process");
  await listen<String>('event-name', (event) => {
   console.log(event);
});
};

const response = async()=>{
  let data = await fetch('https://v0.yiketianqi.com/api?unescape=1&version=v91&appid=43656176&appsecret=I42og6Lm&ext=&cityid=&city=%E6%9D%AD%E5%B7%9E', {
  method: 'GET',
  timeout: 30,
});
console.log(data);
};


const router = useRouter();
const toPagePath = (url: string) => {
// 这里回调写成对象，方便后面传参 push 写成 replace 不会留下历史记录
console.log(url);
  router.push({
    path: url,
  });
};

</script>

<template>
  <el-form :model="form" label-width="120px">
    <el-form-item label="Activity name">
      <el-input v-model="form.name" />
    </el-form-item>
    <el-form-item label="Activity form">
      <el-input v-model="form.desc" type="textarea" />
    </el-form-item>
    <el-form-item>
      <el-button type="primary" @click="onSubmit">Create</el-button>
      <el-button>Cancel</el-button>
    </el-form-item>
  </el-form>
  <button @click="toPagePath('/login')">login</button>
</template>
