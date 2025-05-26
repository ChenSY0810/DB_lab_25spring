<template>
  <div class="container">
    <h1>数据操作面板</h1>

    <div class="buttons">
      <button @click="setAction('query')">查询</button>
      <button @click="setAction('update')">更改</button>
      <button @click="setAction('insert')">插入</button>
      <button @click="setAction('range')">范围查询</button>
    </div>

    <div class="action-panel">
      <QueryPanel v-if="action === 'query'" />
      <UpdatePanel v-if="action === 'update'" />
      <InsertPanel v-if="action === 'insert'" />
      <RangePanel v-if="action === 'range'" />
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'

import QueryPanel from './components/QueryPanel.vue'
import UpdatePanel from './components/UpdatePanel.vue'
import InsertPanel from './components/InsertPanel.vue'
import RangePanel from './components/RangePanel.vue'

const action = ref(null)

function setAction(act) {
  action.value = act
}
</script>

<style scoped>
.container {
  max-width: 600px;
  margin: 0 auto;
}
.buttons button {
  margin: 5px;
}
.action-panel {
  margin-top: 20px;
  padding: 10px;
  border: 1px solid #ccc;
}
</style>
