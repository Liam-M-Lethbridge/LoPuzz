<script setup lang="ts">
    import CircleIcon from './components/icons/CircleIcon.vue';
    import StarIcon from './components/icons/StarIcon.vue';
    import SquareIcon from './components/icons/SquareIcon.vue';
    import TriangleIcon from './components/icons/TriangleIcon.vue';
    import { ref, onMounted } from 'vue';
    import { invoke } from '@tauri-apps/api/core';
import HexagonIcon from './components/icons/HexagonIcon.vue';
    var gridSize = 5;
    var grid = ref<number[]>([]);
    var input = ref<number[]>([]);
    var invalids = ref<number[]>([]);
    var position = ref<number>(0);

    async function newGrid(){
        grid.value = await invoke("create_shapes_game", { "gridSize":gridSize, "difficulty":0 })
        input = ref<number[]>(new Array(gridSize**2).fill(0));
        position = ref<number>(gridSize*gridSize);
        invalids = ref<number[]>(new Array(gridSize**2).fill(0));
    }

    onMounted(async () => {
        newGrid();
    });



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
          class="cell">

          <!-- :style="{ backgroundColor: findColour(cell) }" -->
          <!-- @click.shift="position=i; toggle(i, 2)" -->
          <!-- @click.exact="position=i; toggle(i, 1)" -->
        <svg v-if="grid[i] === 1" viewBox="0 0 100 100" style="height:60%; width:60%; z-index: 2;">
            <CircleIcon/>
        </svg>
        <svg v-if="grid[i] === 2" viewBox="0 0 100 100" style="height:60%; width:60%; z-index: 2;">
            <TriangleIcon/>
        </svg>
          <svg v-if="grid[i] === 3" viewBox="0 0 100 100" style="height:60%; width:60%; z-index: 2;">
            <SquareIcon/>
        </svg>
          <svg v-if="grid[i] === 4" viewBox="0 0 100 100" style="height:60%; width:60%; z-index: 2;">
            <StarIcon/>
        </svg>
                  <svg v-if="grid[i] === 5" viewBox="0 0 100 100" style="height:60%; width:60%; z-index: 2;">
            <HexagonIcon/>
        </svg>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>

.grid{
  display: grid;
  width: 100%;
  height: 100%;
  border: 4px solid #000000;
  border-radius: 1%;
}
.square {
  display: block;
  justify-content: center;
  align-items: center;

  width: min(90vw, 90vh);
  aspect-ratio: 1 / 1;
}

.background{
    display: flex;
    justify-content: center;
    align-items: center;

    width: 100vw;
    height: 100vh;
    background-color: #d7e6ff;

}
.cell{
  display: flex;
  position: relative;
  aspect-ratio: 1 / 1;
  border: 1px solid #00000055;
  box-sizing: border-box;
  justify-content: center; 
  align-items: center;
}

</style>