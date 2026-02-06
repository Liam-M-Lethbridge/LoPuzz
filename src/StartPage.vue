<script setup lang="ts">
  import QueenBox from "./components/icons/QueenBox.vue";
  import { useRouter } from "vue-router";

  const router = useRouter();
  function startUp(){
    router.push("/home");
  }

</script>

<template>
  <button style="padding:0;border:0;" @click="startUp">
    <div class="background">
      <QueenBox style="width: min(10vh,10vw); height: min(10vh,10vw);"></QueenBox>
    </div>
  </button>
</template>

<style scoped>

.background {
  display: flex;
  justify-content: center;
  align-items: center;

  width: 100vw;
  height: 100vh;
  animation-duration: 60s;
  animation-name: rotate-hue;
  animation-iteration-count: infinite;
  margin:0px;
}



@keyframes appear-and-dissappear {
  0%{
    opacity: 0;
  }
  50%{
    opacity: 100;
  }
  75%{
    opacity: 0;
  }
  100%{
    opacity: 0;
  }
}


@keyframes rotate-hue {
  0% {
    background-color: oklch(0.7 0.9 0);
  }
  10% {
    background-color: oklch(0.8 0.5 36);
  }
  20% {
    background-color: oklch(0.9 0.5 72);
  }
  30% {
    background-color: oklch(0.7 0.9 108);
  }
  40% {
    background-color: oklch(0.7 0.9 144);
  }
  50% {
    background-color: oklch(0.7 0.9 180);
  }
  60%{
    background-color: oklch(0.7 0.9 216);
  }
  70% {
    background-color: oklch(0.7 0.9 252);
  }
  80% {
    background-color: oklch(0.7 0.9 288);
  }
  90% {
    background-color: oklch(0.7 0.9 324);
  }
  100%{
    background-color: oklch(0.7 0.9 360);
  }
}
</style>

