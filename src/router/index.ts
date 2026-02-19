import { createRouter, createWebHashHistory, type RouteRecordRaw } from 'vue-router'
import HomePage from "../Home.vue";
import Queens from "../Queens.vue";
import Shapes from "../Shapes.vue";
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
  {path: "/shapes",
    component: Shapes
  },
]

export const router = createRouter({
  history: createWebHashHistory(),
  routes,
})

export default router