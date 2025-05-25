<template>
  <div>
    <h3>查询操作</h3>
    <label>字段：
      <select v-model="field">
        <option value="id">ID</option>
        <option value="name">名字</option>
        <option value="status">状态</option>
      </select>
    </label>

    <label>值：
      <input v-model="value" placeholder="输入查询值" />
    </label>

    <button @click="submit">提交查询</button>
  </div>
</template>

<script setup>
import { ref } from 'vue'

const field = ref('id')
const value = ref('')

function submit() {
  // 示例请求
  fetch(`/api/query?field=${field.value}&value=${value.value}`)
    .then(res => res.json())
    .then(data => {
      alert("查询结果: " + JSON.stringify(data))
    })
}
</script>
