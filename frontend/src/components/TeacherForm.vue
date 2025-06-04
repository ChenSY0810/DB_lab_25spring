<template>
  <form @submit.prevent="submitForm" class="bg-gray-100 p-4 rounded border w-full max-w-md">
    <h3 class="text-lg font-semibold mb-4">新增教师</h3>

    <div class="mb-3">
      <label class="block mb-1">姓名</label>
      <input v-model="form.teacher_name" type="text" class="w-full p-2 border rounded" required />
    </div>

    <div class="mb-3">
      <label class="block mb-1">职称</label>
      <select v-model.number="form.teacher_title" class="w-full p-2 border rounded">
        <option value=1>博士后</option>
        <option value=2>助教</option>
        <option value=3>讲师</option>
        <option value=4>副教授</option>
        <option value=5>特任教授</option>
        <option value=6>教授</option>
        <option value=7>助理研究员</option>
        <option value=8>特任副研究员</option>
        <option value=9>副研究员</option>
        <option value=10>特任研究员</option>
        <option value=11>研究员</option>
      </select>
    </div>

    <div class="mb-4">
      <label class="block mb-1">性别</label>
      <select v-model.number="form.teacher_sex" class="w-full p-2 border rounded">
        <option value=1>男</option>
        <option value=2>女</option>
      </select>
    </div>

    <button type="submit" class=" bg-emerald-300 text-blue-600 px-4 py-2 rounded">
      提交
    </button>
  </form>
</template>

<script setup>
import { ref } from 'vue'

const emit = defineEmits(['teacher-added'])

const form = ref({
  teacher_name: "",
  teacher_sex: 1,
  teacher_title: 1,
})

async function submitForm() {
  const res = await fetch('/api/teachers', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      Authorization: `Bearer ${localStorage.getItem('token')}`
    },
    body: JSON.stringify(form.value)
  })

  if (res.ok) {
    emit('teacher-added')
    form.value = { teacher_name: "", teacher_sex: 1, teacher_title: 1 }
    alert('添加成功')
  } else {
    alert('添加失败')
  }
}
</script>