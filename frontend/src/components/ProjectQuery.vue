<template>
  <div class="space-y-4">
    <h3 class="text-lg font-semibold">查询项目</h3>

    <input
      v-model="projectName"
      placeholder="请输入项目名称"
      class="w-full p-2 border rounded"
    />

    <button @click="queryProject" class="px-4 py-2 bg-blue-600 text-white rounded">
      查询
    </button>

    <div v-if="project" class="bg-white p-4 shadow rounded mt-4">
      <div><strong>项目名称:</strong> {{ project.name }}</div>
      <div><strong>来源:</strong> {{ project.source }}</div>
      <div><strong>类型:</strong> {{ project.project_type }}</div>
      <div><strong>保密级别:</strong> {{ project.secret_level }}</div>
      <div><strong>参与老师:</strong>
        <ul class="list-disc ml-6 mt-1">
          <li v-for="teacher in project.teachers" :key="teacher.id">{{ teacher.name }}</li>
        </ul>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'

const projectName = ref('')
const project = ref(null)

const queryProject = async () => {
  const username = localStorage.getItem('username')
  if (!projectName.value || !username) {
    alert('请填写项目名并确保已登录')
    return
  }

  const res = await fetch(`/api/projects/query?username=${encodeURIComponent(username)}`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ name: projectName.value })
  })

  if (res.ok) {
    project.value = await res.json()
  } else {
    alert('查询失败，请检查权限或项目名')
  }
}
</script>
