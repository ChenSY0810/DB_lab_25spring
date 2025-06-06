<template>
  <div>
    <h2 class="text-xl font-bold mb-4">教师</h2>
    <TeacherForm @teacher-added="loadTeachers" />

    <div class="mt-6">
      <h3 class="text-lg font-semibold mb-2">教师列表</h3>
      <table class="w-full border text-left">
        <thead class="bg-gray-100">
          <tr>
            <th class="p-2">工号</th>
            <th class="p-2">姓名</th>
            <th class="p-2">职称</th>
            <th class="p-2">性别</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="teacher in teachers" :key="teacher.id" class="border-t">
            <td class="p-2">{{ teacher.teacher_id }}</td>
            <td class="p-2">{{ teacher.teacher_name }}</td>
            <td class="p-2">{{ mapTitle(teacher.teacher_title) }}</td>
            <td class="p-2">{{ mapSex(teacher.teacher_sex) }}</td>
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

const titleMap = {
  1: '博士后',
  2: '助教',
  3: '讲师',
  4: '副教授',
  5: '特任教授',
  6: '教授',
  7: '助理研究员',
  8: '特任副研究员',
  9: '副研究员',
  10: '特任研究员',
  11: '研究员'
};

const sexMap = {
  1: '男',
  2: '女'
};

function mapTitle(code) {
  return titleMap[code] || '未知';
}

function mapSex(code) {
  return sexMap[code] || '未知';
}


async function loadTeachers() {
  const res = await fetch('/api/teachers', {
    headers: {
      method: 'GET',
    }
  })
  teachers.value = await res.json()
}

onMounted(() => {
  loadTeachers()
})
</script>
