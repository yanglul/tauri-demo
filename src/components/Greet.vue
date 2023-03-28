<script setup lang="ts">
import { ref, reactive, inject  } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from '@tauri-apps/api/event';
import { fetch } from '@tauri-apps/api/http';
import ElementPlus from 'element-plus';
// import zhCn from 'node_modules/element-plus/dist/locale/zh-cn.mjs';
 


const greetMsg = ref("");
const name = ref("");

const v3layer = inject('v3layer');
const utils = inject('utils');

const formObj = reactive({
  tel:"",
  pwd:""


  });
 
const handleSubmit = () => {
    
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




</script>

<template>
  <div class="card">
    <input id="greet-input" v-model="name" placeholder="Enter a name..." />
    <button type="button" @click="greet()">Greet</button>
    <button type="button" @click="unlisten()">event</button>
    <button type="button" @click="response()">request</button>
  </div>
  <p>{{ greetMsg }}</p>
  <!-- 登录模板 -->
<template>
    <div>
        <div class="ntMain__cont flex1 flexbox flex-col">
            <div class="nt__lgregWrapper flex1 flexbox flex-col">
                <NavBar transparent />

                <div class="nt__lgregBox flex1">
                    <div class="banner">
                        <h2 class="tit flexbox flex-alignc"><img src="@assets/logo.png" />TAURI-VUE3-CAHT</h2>
                        <img class="bg" src="/static/skin/bg-banner.jpg" />
                    </div>
                    <div class="forms">
                        <form @submit.prevent="handleSubmit">
                            <ul class="clearfix">
                                <li><input class="iptxt flex1" type="text" v-model="formObj.tel" placeholder="请输入手机号" /></li>
                                <li><input class="iptxt flex1" type="password" v-model="formObj.pwd" placeholder="请输入密码"/></li>
                            </ul>
                            <div class="btns">
                                <el-button type="primary" native-type="submit" size="default" auto-insert-space>登录</el-button>
                            </div>
                            <div class="lgregLink align-c clearfix">
                                <router-link class="navigator" to="#">忘记密码</router-link>
                                <router-link class="navigator" to="/register">注册账号</router-link>
                            </div>
                            <!-- ... -->
                        </form>
                    </div>
                </div>
                <!-- ... -->
            </div>
        </div>
    </div>
</template>

</template>
