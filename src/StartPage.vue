<script setup lang="ts">
  import { ref, onMounted } from "vue";
  import { invoke } from "@tauri-apps/api/core";
  import QueenIcon from "./components/icons/QueenIcon.vue";
  import { useRouter } from "vue-router";

  const router = useRouter();
  function startUp(){
    router.push("/home");
  }

</script>

<template>
  <button style="padding:0;border:0;" @click="startUp">
    <div class="background">
      <div class="QueenSquare">

      <!-- style="position:absolute; top:-34px; left: 3px;"> -->
        <svg viewBox="0 0 32 22" height="min(6vw, 6vh)" width="min(6vw, 6vh)" y="1em">
        <QueenIcon></QueenIcon>
        </svg>
      </div>
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


.QueenSquare{
  display: flex;
  justify-content: center;
  align-items: center;
  background-color: #FDD7FF;
  width: min(10vh,10vw);
  height: min(10vh,10vw);
  border: 4px solid #000000;
  border-radius: 20%;
  top:4px; 
  left:30px;
  animation-duration: 6s;
  animation-name: appear-and-dissappear;
  animation-iteration-count: infinite;
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

