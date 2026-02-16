import { createRouter, createWebHashHistory, type RouteRecordRaw } from 'vue-router'
import HomePage from "../Home.vue";
import Queens from "../Queens.vue";
import StartPage from '../StartPage.vue';
const routes: RouteRecordRaw[] = [
  {path: "/",
    component: StartPage
  },  
  {path: "/home",
    component: HomePage
  },
  {path: "/queens",
    component: Queens
  },
]

export const router = createRouter({
  history: createWebHashHistory(),
  routes,
})

export default router