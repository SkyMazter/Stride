import { createRouter, createWebHashHistory, RouteRecordRaw } from "vue-router";
import Home from "../components/Home.vue";
import Submit from "../components/Submit.vue";

const routes: Array<RouteRecordRaw> = [
  {
    path: "/",
    name: "Home",
    component: Home,
    children: [{ path: "submit", name: "Submit", component: Submit }],
  },
  { path: "/submit", name: "Submit", component: Submit },
];

const router = createRouter({
  history: createWebHashHistory(),
  routes,
});

export default router;
