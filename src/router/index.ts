import { createRouter, createWebHistory } from "vue-router";
import Home from "@/views/Home.vue";
import About from "@/views/About.vue";
import Setting from "@/views/Setting.vue";
import BotCtrl from "@/views/BotCtrl.vue";

const routes = [
  { path: "/", name: "Home", component: Home },
  { path: "/ctrl", name: "BotCtrl", component: BotCtrl },
  { path: "/setting", name: "Setting", component: Setting },
  { path: "/about", name: "About", component: About },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
