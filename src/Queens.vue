<script setup lang="ts">
  import { ref, onMounted } from "vue";
  import { invoke } from "@tauri-apps/api/core";


  function getBorders(index:number, grid, size: number) {
    const row = Math.floor(index / size);
    const col = index % size;
    const color = grid[index];

    const borders = {
      top: false,
      right: false,
      bottom: false,
      left: false,
    };

    if (row === 0 || grid[(row - 1) * size + col] !== color) {
      borders.top = true;
    }
    if (col === size - 1 || grid[row * size + col + 1] !== color) {
      borders.right = true;
    }
    if (row === size - 1 || grid[(row + 1) * size + col] !== color) {
      borders.bottom = true;
    }
    if (col === 0 || grid[row * size + col - 1] !== color) {
      borders.left = true;
    }

    return borders;
  }

  function getBorderClasses(i: number) {
    const b = getBorders(i, grid.value, gridSize);
    return {
      top: b.top,
      right: b.right,
      bottom: b.bottom,
      left: b.left,
    };
  }





  const gridSize = 6; 

  const grid = ref([]);

  onMounted(async () => {
    grid.value = await invoke("create_queens_game", { gridSize });
    
  });
  // TODO generate the colours RANDOMLY else you can figure out just from the colours  
  // TODO generate the colours RANDOMLY else you can figure out just from the colours  
  const colourMap = {
    0: "oklch(1 0 0)",
    1: "oklch(0.8266 0.0967 19.33)",
    2: "oklch(0.8995 0.0782 71.55)",
    3: "oklch(0.9816 0.0921 109.3)",
    4: "oklch(0.8411 0.1487 139.76)",
    5: "oklch(0.8412 0.0839 203.78)",
    6: "oklch(0.8156 0.0925 260.13)",
    7: "oklch(0.8133 0.1332 319.76)",
    8: "oklch(0.8088 0.1529 326.36)",
    9: "oklch(0.8647 0.0913 73.06)",
    10: "oklch(0.66 0.1529 267.88)",
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
          :class="getBorderClasses(i)"
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
  border: 4px solid #000000;
  border-radius: 1%;
}
.cell{
  aspect-ratio: 1 / 1;
  border: 1px solid #00000055;
  box-sizing: border-box;
}


.cell.top {
  border-top: 2px solid black;
}
.cell.right {
  border-right: 2px solid black;
}
.cell.bottom {
  border-bottom: 2px solid black;
}
.cell.left {
  border-left: 2px solid black;
}

</style>

