<script setup lang="ts">
  import { ref, onMounted } from "vue";
  import { invoke } from "@tauri-apps/api/core";


  const gridSize = 8;

const grid = ref([]);

onMounted(async () => {
  grid.value = await invoke("create_queens_game", { gridSize });
});

  const colourMap = {
  0: "#ffffff",
  1: "#ffadad",
  2: "#ffd6a5",
  3: "#fdffb6",
  4: "#caffbf",
  5: "#9bf6ff",
  6: "#a0c4ff",
  7: "#bdb2ff",
  8: "#ffc6ff"
};

</script>

<template>
  <div class="background">
    <div class="square">
      <div
        class="grid"
        :style="{ gridTemplateColumns: `repeat(${gridSize}, 1fr)` }"
      >
        <div
          v-for="(cell, i) in grid"
          :key="i"
          class="cell"
          :style="{ backgroundColor: colourMap[cell] }"
        />
      </div>
    </div>
  </div>
</template>

<style scoped>

.background {
  display: flex;
  justify-content: center;
  align-items: center;

  width: 100vw;
  height: 100vh;

  background-color: white;
}

.square {
  display: block;
  justify-content: center;
  align-items: center;
  background-color: #FDD7FF;

  width: min(90vw, 90vh);
  aspect-ratio: 1 / 1;
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

