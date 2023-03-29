import {createRouter,createWebHistory,RouteRecordRaw} from 'vue-router'

const routes: Array<RouteRecordRaw> = [
    {
        path:'/',
        name:'home',
        component:()=> import(/*  webpackChunkName:"login"   */'../components/Greet.vue'),
    },
    {
        path:'/login',
        name:'login',
        component:()=> import(/*  webpackChunkName:"login"   */'../views/LoginView.vue'),
    },

]
const router = createRouter({
    history: createWebHistory(),
    routes,
  });
   
// 3导出路由   然后去 main.ts 注册 router.ts
export default router