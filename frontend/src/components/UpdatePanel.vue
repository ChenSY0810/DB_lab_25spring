<template>
  <div>
    <h3>更改操作</h3>
    <label>字段：
      <select v-model="field">
        <option value="name">名字</option>
        <option value="status">状态</option>
      </select>
    </label>

    <label>主键 ID:
      <input v-model="id" />
    </label>

    <label>新值：
      <input v-model="newValue" />
    </label>

    <button @click="submit">提交更改</button>
    <p v-if="result">结果：{{ result }}</p>
  </div>
</template>

<script setup>
import { ref } from 'vue'

const field = ref('name')
const id = ref('')
const newValue = ref('')
const result = ref('')

async function submit() {
  const res = await fetch(`/api/update`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ id: id.value, field: field.value, value: newValue.value })
  })
  result.value = await res.text()
}
</script>
