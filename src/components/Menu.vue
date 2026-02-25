<script setup lang="ts">
import { ref } from 'vue';
import MenuButton from './MenuButton.vue';
import { useRouter } from "vue-router";
import HomeButton from './HomeButton.vue';
const router = useRouter();

let active = ref(false);
function toggle(){
    active.value = !active.value;
}
</script>

<template>
  <div class="menu-wrapper" :class="{ open: active }">
    <div class="menu">
      <HomeButton @click="router.push('/home')" />
    </div>

    <MenuButton @click="toggle" />
  </div>
</template>


<style scoped>
.menu-wrapper {
  position: fixed;
  top: 0;
  left: 0;
  height: 100vh;

  display: flex;
  align-items: flex-start;

  transform: translateX(-40vw);
  transition: transform 0.3s ease;
  z-index: 4;
}

.menu-wrapper.open {
  transform: translateX(0);
}

.menu {
    display: flex;
  width: 40vw;
  height: 100vh;
  background-color: #DDDDDD;
  border-right: 4px solid black;
  justify-content: center;

}

</style>