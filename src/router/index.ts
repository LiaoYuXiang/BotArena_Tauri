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
  { path: "/", name: "Home", component: Home, meta: { title: "首頁" } },
  {
    path: "/ctrl",
    name: "BotCtrl",
    component: BotCtrl,
    meta: { title: "機器人控制頁面" },
  },
  {
    path: "/setting",
    name: "Setting",
    component: Setting,
    children: [
      {
        path: "",
        name: "setting_default",
        component: setting_default,
        meta: { title: "設定" },
      },
      {
        path: "ControlMode",
        name: "ControlMode",
        component: Set_ControlMode,
        meta: { title: "控制設定" },
      },
      {
        path: "Connectivity",
        name: "Connectivity",
        component: Set_Connectivity,
        meta: { title: "網路連接設定" },
      },
      {
        path: "Profiles",
        name: "Profiles",
        component: Set_Profiles,
        meta: { title: "設定檔" },
      },
      {
        path: "About",
        name: "About",
        component: Set_About,
        meta: { title: "關於" },
      },
    ],
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
