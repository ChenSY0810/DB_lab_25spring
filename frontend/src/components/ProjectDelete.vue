<template>
  <div class="space-y-4">
    <h3 class="text-lg font-semibold">删除项目</h3>

    <input
      v-model="projectName"
      placeholder="要删除的项目名称"
      class="p-2 border rounded w-full"
    />

    <button
      @click="deleteProject"
      class="px-4 py-2 bg-red-600 text-white rounded"
    >
      删除项目
    </button>
  </div>
</template>

<script setup>
import { ref } from 'vue'

const projectName = ref('')

const deleteProject = async () => {
  const username = localStorage.getItem('username')
  if (!username) {
    alert('未登录')
    return
  }

  if (!projectName.value) {
    alert('请输入项目名称')
    return
  }

  const confirmed = confirm(`确认要删除项目 "${projectName.value}" 吗？`)
  if (!confirmed) return

  const res = await fetch(`/api/projects?username=${encodeURIComponent(username)}`, {
    method: 'DELETE',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ name: projectName.value })
  })

  if (res.ok) {
    alert('删除成功')
    projectName.value = ''
  } else if (res.status === 403) {
    alert('权限不足，无法删除')
  } else if (res.status === 404) {
    alert('项目不存在')
  } else {
    alert('删除失败')
  }
}
</script>
