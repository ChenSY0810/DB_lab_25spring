<template>
  <div class="flex justify-center">
    <form @submit.prevent="submitForm" class="bg-gray-100 p-4 rounded border w-full max-w-md ">
      <h3 class="text-lg font-semibold mb-4">绑定账号</h3>

      <div class="mb-3">
        <label class="block mb-1">教师工号</label>
        <input v-model="form.teacher_id" type="text" pattern="^[0-9a-fA-F]{1,5}$" title="请输入5位以内十六进制数字（如 1aF）" class="w-full p-2 border rounded" required />
      </div>

      <div class="mb-3">
        <label class="block mb-1">用户名</label>
        <input v-model="form.user_name" type="text" class="w-full p-2 border rounded" required />
      </div>

      <button type="submit" class=" bg-emerald-300 text-blue-600 px-4 py-2 rounded">
        提交
      </button>
    </form>
  </div>
</template>

<script setup>
import { ref } from 'vue'

const emit = defineEmits(['linked'])

const form = ref({
  teacher_id: "",
  user_name: "",
})

async function submitForm() {
  const res = await fetch('/api/link', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(form.value)
  })

  if (res.ok) {
    emit('linked')
    form.value = { teacher_id: "", user_name: "" }
    alert('关联成功')
  } else {
    alert('关联失败')
  }
}
</script>