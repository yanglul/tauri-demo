<template>
    <div>
        主页
        <el-button type="primary" @click="toLogin()">登录</el-button>
    </div>
</template>

<script lang="ts" setup>
import { defineComponent } from 'vue'
import { useRouter } from 'vue-router';
let router = useRouter();
const toLogin = ()=>{
    router.push({
        path: "/login"
    })
};
</script>

<style scoped>

</style>