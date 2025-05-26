<template>
  <div>
    <h3>插入操作</h3>
    <label>名字：
      <input v-model="name" />
    </label>

    <label>状态：
      <input v-model="status" />
    </label>

    <button @click="submit">提交插入</button>
    <p v-if="result">结果：{{ result }}</p>
  </div>
</template>

<script setup>
import { ref } from 'vue'

const name = ref('')
const status = ref('')
const result = ref('')

async function submit() {
  const res = await fetch(`/api/insert`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ name: name.value, status: status.value })
  })
  result.value = await res.text()
}
</script>
