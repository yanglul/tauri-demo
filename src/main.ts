import { createApp } from "vue";
import "./styles.css";
import App from "./App.vue";
// import ElementPlus from 'element-plus'
// import 'element-plus/dist/index.css'
// import 'element-plus/es/components/message/style/css'
import router from './router/index'
const app = createApp(App)
 
//还有这里
// app.use(ElementPlus)


createApp(App).use(router).mount("#app");
