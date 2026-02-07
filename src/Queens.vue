<script setup lang="ts">
  import { ref, onMounted } from "vue";
  import { invoke } from "@tauri-apps/api/core";
  import QueenIcon from "./components/icons/QueenIcon.vue";
  const gridSize = 4; 


  function getBorders(index:number, grid:any, size: number) {
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

  var position = ref(gridSize*gridSize);

  function move(dir: number){
    if(position.value == gridSize*gridSize){
      position.value = 0;
    }
    else if(position.value+dir < gridSize*gridSize && position.value+dir >=0){
      if (dir == -1 && position.value%gridSize == 0){
        return;
      }
      if (dir == 1 && position.value%gridSize == gridSize-1){
        return;
      }
      position.value = position.value + dir;
    }
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

  function findColour(cell: never, i: number){
    if (position.value == i){
      return "#000000";
    }
    else{
      return colourMap[cell];
    }
  }

  var input = ref(new Array(gridSize**2).fill(0));
  const grid = ref([]);

  async function newGrid(){
    grid.value = await invoke("create_queens_game", { gridSize })
    input = ref(new Array(gridSize**2).fill(0));
    position = ref(gridSize*gridSize);
  }

  onMounted(async () => {
    grid.value = await invoke("create_queens_game", { gridSize });
    window.addEventListener("keydown", e => {
      if (e.key === "ArrowUp") {
        e.preventDefault()
        move(-gridSize);
      }
      else if (e.key === "ArrowDown") {
        e.preventDefault()
        move(gridSize);
      }
      else if (e.key === "ArrowLeft") {
        e.preventDefault()
        move(-1);
      }
      else if (e.key === "ArrowRight") {
        e.preventDefault()
        move(1);
      }
      else if (e.key == "q"){
        e.preventDefault()
        toggle(position.value, 1);
      }
      else if (e.key == "x"){
        e.preventDefault()
        toggle(position.value, 2);
      }
      else if (e.key == "r"){
        e.preventDefault()
        newGrid();
      }
    })

  
  });
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
  function toggle(i: number, value: number){
    var j = input.value[i];
    if (j == value){
      input.value[i] = 0;
    }else{
      input.value[i] = value;
    }
  }


</script>

<template>
  <!-- <div tabindex="0" @keydown.left.prevent="move(-1)" @keydown.right.prevent="move(1)" @keydown.up.prevent="move(-gridSize)" @keydown.down.prevent="move(gridSize)"/> -->
  <div class="background">
    <button @click="newGrid"></button>
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
          :style="{ backgroundColor: findColour(cell, i) }"
          @click.shift="position=i; toggle(i, 2)"
          @click.exact="position=i; toggle(i, 1)"
        >
          <svg v-if="input[i] === 1" viewBox="0 0 32 22" height="min(60%, 60%)" width="min(60%, 60%)" y="1em">
            <QueenIcon></QueenIcon>
          </svg>
          <div v-if="input[i] === 2" style="font-size: 100px;">X</div>
        </div>
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

  background-color: #FDD7FF;
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
  display: flex;
  aspect-ratio: 1 / 1;
  border: 1px solid #00000055;
  box-sizing: border-box;
  justify-content: center; 
  align-items: center;
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

