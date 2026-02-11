<script setup lang="ts">
  import { ref, onMounted } from "vue";
  import { invoke } from "@tauri-apps/api/core";
  import QueenIcon from "./components/icons/QueenIcon.vue";
  import ErrorBox from "./components/icons/ErrorBox.vue";
  import XBox from "./components/icons/XBox.vue";
  import SelectBoxx from "./components/icons/SelectBoxx.vue";

  const gridSize = 5; 

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

  function findColour(cell: never){
    return colourMap[cell];
    
  }

  var input = ref(new Array(gridSize**2).fill(0));
  var invalids = ref(new Array(gridSize**2).fill(0));
  const grid = ref([]);

  function findInvalids(newQueenIndex: number){

    const column = newQueenIndex%gridSize;
    const row = Math.floor(newQueenIndex/gridSize);
    console.log(row, column)
    var counter = 0;
    // check column:
    for(let i = 0; i < gridSize; i++){
      if(input.value[column+gridSize*i] == 1){
        counter+=1;
      }
    }
    if(counter >1){
      console.log("column issue");
      for(let i = 0; i < gridSize; i++){
        invalids.value[column+gridSize*i] +=1;
      }
    }

    // check row
    counter = 0;
    for(let i = 0; i < gridSize; i++){
      if(input.value[row*gridSize+i] == 1){
        counter+=1;
      }
    }
    if(counter >1){
      console.log("row issue");
      for(let i = 0; i < gridSize; i++){
        invalids.value[row*gridSize+i] +=1;
      }
    }
    // check proximities
    const coordsAround = [ -gridSize, gridSize]

    // ADD CONSTRAINTS FOR PROXIMITIES
    if(column >0){
      coordsAround.push( -1)
      if(row > 0){
        coordsAround.push(-1-gridSize)
      }
      if(row < gridSize-1){
        coordsAround.push(-1+gridSize)
      }
    }
    if(column < gridSize-1){
      coordsAround.push( 1)
      if(row > 0){
        coordsAround.push(1-gridSize)
      }
      if(row < gridSize-1){
        coordsAround.push(1+gridSize)
      }
    }
    for (const x of coordsAround){
      // if(newQueenIndex+x >=0 && newQueenIndex+x <gridSize**2){
        if(input.value[newQueenIndex+x] == 1){
          console.log("nearby issue")
          invalids.value[newQueenIndex+x]+=1;
          invalids.value[newQueenIndex]+=1;
        // }
      }
    }
    // check colours
    counter = 0;
    const colour = grid.value[newQueenIndex];
    var colouredCells = [];
    for(let i = 0; i < gridSize**2; i++){
      if(grid.value[i] == colour){
        colouredCells.push(i)
        if(input.value[i] == 1){
          counter +=1;
      }

      }
    }
    if(counter >1){
      console.log("colour issue")
      for(const colouredCell of colouredCells){
        invalids.value[colouredCell] +=1;
      }
    }
  }

  function removeInvalids(newQueenIndex: number){

    const column = newQueenIndex%gridSize;
    const row = Math.floor(newQueenIndex/gridSize);
    var counter = 0;
    // check column:
    for(let i = 0; i < gridSize; i++){
      if(input.value[column+gridSize*i] == 1){
        counter+=1;
      }
    }
    if(counter >0){
      console.log("column fix");

      for(let i = 0; i < gridSize; i++){
        invalids.value[column+gridSize*i] -=1;
      }
    }

    // check row
    counter = 0;
    for(let i = 0; i < gridSize; i++){
      if(input.value[row*gridSize+i] == 1){
        counter+=1;
      }
    }
    if(counter >0){
      console.log("row fix");
      for(let i = 0; i < gridSize; i++){
        invalids.value[row*gridSize+i] -=1;
      }
    }
    // check proximities
    const coordsAround = []

    // ADD CONSTRAINTS FOR PROXIMITIES
    if(column >0){
      coordsAround.push( -1)
      if(row > 0){
        coordsAround.push(-1-gridSize)
      }
      if(row < gridSize-1){
        coordsAround.push(-1+gridSize)
      }
    }
    if(column <gridSize-1){
      coordsAround.push( 1)
      if(row > 0){
        coordsAround.push(1-gridSize);
      }
      if(row < gridSize-1){
        coordsAround.push(1+gridSize);
      }
    }
    if(row > 0){
      coordsAround.push(-gridSize);
    }
    if(row < gridSize-1){
      coordsAround.push(gridSize);
    }
    for (const x of coordsAround){
      
      if(input.value[newQueenIndex+x] == 1){
        console.log("nearby fix");
        invalids.value[newQueenIndex+x]-=1;
        invalids.value[newQueenIndex]-=1;
        
      }
    }
    // check colours
    counter = 0;
    const colour = grid.value[newQueenIndex];
    var colouredCells = [];
    for(let i = 0; i < gridSize**2; i++){
      if(grid.value[i] == colour){
        colouredCells.push(i)
        if(input.value[i] == 1){
        counter +=1;
      }
      }
    }
    if(counter >0){
      console.log("colour fix");
      for(const colouredCell of colouredCells){
        invalids.value[colouredCell] -=1;
      }
    }
  }

  async function newGrid(){
    grid.value = await invoke("create_queens_game", { gridSize })
    input = ref(new Array(gridSize**2).fill(0));
    position = ref(gridSize*gridSize);
    invalids = ref(new Array(gridSize**2).fill(0));
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
    0: "oklch(0 0 0)",
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
      if(value == 1){
        removeInvalids(i);
      }
    }else{
      input.value[i] = value;
      if(value == 1){
        findInvalids(i);
      }else if(j == 1){removeInvalids(i)}
    }
    console.log(invalids.value)
  }

</script>

<template>
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
          :style="{ backgroundColor: findColour(cell) }"
          @click.shift="position=i; toggle(i, 2)"
          @click.exact="position=i; toggle(i, 1)"
        >
          <svg v-if="input[i] === 1" viewBox="0 0 32 22" style="height:60%; width:60%; z-index: 2;">
            <QueenIcon/>
          </svg>
          <svg v-if="position===i" viewBox="0 0 20 20" style="height:90%; width:90%; z-index: 3;">
            <select-boxx/>
          </svg>
          <svg v-if="invalids[i] >0" viewBox="0 0 20 20" style="height:90%; width:90%; z-index: 1;">
            <ErrorBox/>
          </svg>
          <XBox v-if="input[i] === 2" style="font-size: 100px;"/>
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
  /* z-index: -2; */
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
.cell svg {
  position: absolute;
  inset: 0;
  margin: auto;
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

