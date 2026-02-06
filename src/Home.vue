<script setup lang="ts">
  import { ref, onMounted } from "vue";
  import { invoke } from "@tauri-apps/api/core";
  import QueenBox from "./components/icons/QueenBox.vue";
  import { useRouter } from "vue-router";

  const gridSize = 8;
  const hover = ref(false);
  const grid = ref([]);
  const router = useRouter();
  function toGame(game:string){
    router.push(game);
  }
onMounted(async () => {
  grid.value = await invoke("create_queens_game", { gridSize });
});

</script>

<template>
  <div class="background">
      <QueenBox style="height:min(40vh, 40vw);width:min(40vh, 40vw);" @click="toGame('/queens')" @mouseover="hover = true" @mouseleave="hover = false" :style="{ active: hover }"></QueenBox>
  </div>
</template>

<style scoped>

.background {
  display: flex;
  justify-content: center;
  align-items: center;

  width: 100vw;
  height: 100vh;

  /* background-color: white; */
  background: linear-gradient(#223d75,#193e8f)
}
.QueenSquare:hover{
  background-color: #dca8e0;;
}

.grid{
  display: grid;
  width: 100%;
  height: 100%;
}
.cell{
  aspect-ratio: 1 / 1;
}
</style>

