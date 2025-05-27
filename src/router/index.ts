import { createRouter, createWebHistory } from "vue-router";
import Home from "@/views/Home.vue";
import Setting from "@/views/Setting.vue";
import BotCtrl from "@/views/BotCtrl.vue";

// 設定子頁面
import Set_ControlMode from "@/views/setting/ControlMode.vue";
import Set_Connectivity from "@/views/setting/Connectivity.vue";
import Set_Profiles from "@/views/setting/Profiles.vue";
import Set_About from "@/views/setting/About.vue";
import setting_default from "@/views/setting/setting_default.vue";

const routes = [
  { path: "/", name: "Home", component: Home },
  { path: "/ctrl", name: "BotCtrl", component: BotCtrl },
  {
    path: "/setting",
    name: "Setting",
    component: Setting,
    children: [
      {
        path: "",
        name: "setting_default",
        component: setting_default,
      },
      { path: "ControlMode", name: "ControlMode", component: Set_ControlMode },
      {
        path: "Connectivity",
        name: "Connectivity",
        component: Set_Connectivity,
      },
      { path: "Profiles", name: "Profiles", component: Set_Profiles },
      { path: "About", name: "About", component: Set_About },
    ],
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
