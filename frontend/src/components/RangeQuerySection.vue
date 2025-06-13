<template>
  <div class="flex justify-center mt-8">
    <div class="space-y-4 w-full max-w-md">
      <h3 class="text-lg font-semibold text-center">按范围查询</h3>

      <select v-model="selectedTeacherId" class="w-full p-2 border rounded" >
        <option disabled value="">请选择教师</option>
        <option v-for="teacher in teacherOptions" :key="teacher.teacher_id" :value="teacher.teacher_id">
          {{ teacher.teacher_name }}（ID: {{ teacher.teacher_id }}）
        </option>
      </select>

      <input
        v-model="startYear"
        type="number"
        min="1900"
        max="2100"
        placeholder="开始年份（可为空）"
        class="w-full p-2 border rounded"
      />

      <input
        v-model="endYear"
        type="number"
        min="1900"
        max="2100"
        placeholder="结束年份（可为空）"
        class="w-full p-2 border rounded"
      />

      <button
        @click="queryRange"
        class="w-full px-4 py-2 bg-blue-600 text-white rounded"
      >
        查询并下载PDF
      </button>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'

const selectedTeacherId = ref('')
const startYear = ref('')
const endYear = ref('')
const teacherOptions = ref([])

async function loadTeacherOptions() {
  try {
    const res = await fetch('/api/teachers', { method: 'GET' })
    if (res.ok) {
      teacherOptions.value = await res.json()
    }
  } catch (e) {
    console.error('加载教师列表失败', e)
  }
}


async function queryRange() {
  if (!selectedTeacherId.value) {
    alert('请选择教师')
    return
  }

  if (startYear.value !== '' && endYear.value !== '' && Number(startYear.value) > Number(endYear.value)) {
    alert('开始年份不能大于结束年份')
    return
  }

  try {
    const payload = {
      teacher_id: selectedTeacherId.value,
      start_year: startYear.value === '' ? null : Number(startYear.value),
      end_year: endYear.value === '' ? null : Number(endYear.value),
    }

    console.log(JSON.stringify(payload, null, 2));
    const username = localStorage.getItem('username')
    const res = await fetch(`/api/range-query?username=${encodeURIComponent(username)}`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(payload),
    })

    if (!res.ok) {
      alert('查询失败')
      return
    }

    const blob = await res.blob()
    const url = window.URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = `range_query_${startYear.value || 'start'}_${endYear.value || 'end'}.pdf`
    document.body.appendChild(a)
    a.click()
    a.remove()
    window.URL.revokeObjectURL(url)
  } catch (e) {
    alert('查询失败')
    console.error(e)
  }
}

onMounted(loadTeacherOptions)
</script>
