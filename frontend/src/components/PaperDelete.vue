<template>
  <div class="space-y-4">
    <h3 class="text-lg font-semibold">删除论文</h3>

    <input
      v-model="paperName"
      placeholder="要删除的论文名称"
      class="p-2 border rounded w-full"
    />

    <button
      @click="deletePaper"
      class="px-4 py-2 bg-red-600 text-white rounded"
    >
      删除论文
    </button>
  </div>
</template>

<script setup>
import { ref } from 'vue'

const paperName = ref('')

const deletePaper = async () => {
  const username = localStorage.getItem('username')
  if (!username) {
    alert('未登录')
    return
  }

  if (!paperName.value) {
    alert('请输入项目名称')
    return
  }

  const confirmed = confirm(`确认要删除项目 "${paperName.value}" 吗？`)
  if (!confirmed) return

  const res = await fetch(`/api/papers?username=${encodeURIComponent(username)}`, {
    method: 'DELETE',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ name: paperName.value })
  })

  if (res.ok) {
    alert('删除成功')
    paperName.value = ''
  } else if (res.status === 403) {
    alert('权限不足，无法删除')
  } else if (res.status === 404) {
    alert('论文不存在')
  } else {
    alert('删除失败')
  }
}
</script>
