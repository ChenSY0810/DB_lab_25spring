<template>
  <div>
    <h2 class="text-xl font-bold mb-4">新增教师</h2>
    <TeacherForm @teacher-added="loadTeachers" />

    <div class="mt-6">
      <h3 class="text-lg font-semibold mb-2">教师列表</h3>
      <table class="w-full border text-left">
        <thead class="bg-gray-100">
          <tr>
            <th class="p-2">姓名</th>
            <th class="p-2">职称</th>
            <th class="p-2">性别</th>
            <th class="p-2">操作</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="teacher in teachers" :key="teacher.id" class="border-t">
            <td class="p-2">{{ teacher.name }}</td>
            <td class="p-2">{{ teacher.title }}</td>
            <td class="p-2">{{ teacher.gender }}</td>
            <td class="p-2">
              <button class="text-red-600 hover:underline" @click="deleteTeacher(teacher.id)">
                删除
              </button>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
    
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import TeacherForm from './TeacherForm.vue'

const teachers = ref([])

async function loadTeachers() {
  const res = await fetch('/api/teachers', {
    headers: {
      Authorization: `Bearer ${localStorage.getItem('token')}`
    }
  })
  teachers.value = await res.json()
}

async function deleteTeacher(id) {
  if (!confirm('确定要删除这个教师吗？')) return
  await fetch(`/api/teachers/${id}`, {
    method: 'DELETE',
    headers: {
      Authorization: `Bearer ${localStorage.getItem('token')}`
    }
  })
  await loadTeachers()
}

onMounted(() => {
  loadTeachers()
})
</script>
