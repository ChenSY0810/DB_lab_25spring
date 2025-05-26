<template>
  <div>
    <h3>范围查询操作</h3>
    <label>字段：
      <select v-model="field">
        <option value="id">ID</option>
        <option value="status">状态</option>
      </select>
    </label>

    <label>开始值：
      <input v-model="start" />
    </label>

    <label>结束值：
      <input v-model="end" />
    </label>

    <button @click="submit">提交范围查询</button>
    <p v-if="result">结果：{{ result }}</p>
  </div>
</template>

<script setup>
import { ref } from 'vue'

const field = ref('id')
const start = ref('')
const end = ref('')
const result = ref('')

async function submit() {
  const res = await fetch(`/api/range?field=${field.value}&start=${start.value}&end=${end.value}`)
  result.value = await res.text()
}
</script>
