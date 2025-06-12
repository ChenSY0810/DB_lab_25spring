<template>
  <div class="space-y-4">
    <h3 class="text-lg font-semibold">删除课程</h3>

    <input
      v-model="courseName"
      placeholder="要删除的课程名称"
      class="p-2 border rounded w-full"
    />

    <button
      @click="deleteCourse"
      class="px-4 py-2 bg-red-600 text-white rounded"
    >
      删除课程
    </button>
  </div>
</template>

<script setup>
import { ref } from 'vue'

const courseName = ref('')

const deleteCourse = async () => {
  const username = localStorage.getItem('username')
  if (!username) {
    alert('未登录')
    return
  }

  if (!courseName.value) {
    alert('请输入课程名称')
    return
  }

  const confirmed = confirm(`确认要删除课程 "${courseName.value}" 吗？`)
  if (!confirmed) return

  const res = await fetch(`/api/courses?username=${encodeURIComponent(username)}`, {
    method: 'DELETE',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ name: courseName.value })
  })

  if (res.ok) {
    alert('删除成功')
    courseName.value = ''
  } else if (res.status === 403) {
    alert('权限不足，无法删除')
  } else if (res.status === 404) {
    alert('课程不存在')
  } else {
    alert('删除失败')
  }
}
</script>
