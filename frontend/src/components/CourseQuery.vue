<template>
  <div class="space-y-4">
    <h3 class="text-lg font-semibold">查询课程</h3>

    <input
      v-model="courseName"
      placeholder="请输入课程名称"
      class="w-full p-2 border rounded"
    />

    <button @click="queryCourse" class="px-4 py-2 bg-blue-600 text-white rounded">
      查询
    </button>

    <div v-if="cwt" class="bg-white p-4 shadow rounded mt-4">
      <div><strong>课程名称:</strong> {{ cwt.course.course_name }}</div>
      <div><strong>课程性质:</strong> {{ mapProperty(cwt.course.course_property) }}</div>
      <div><strong>总学时:</strong> {{ cwt.course.hours }}</div>

      <div><strong>主讲教师信息:</strong>
        <div class="flex justify-center">
          <table class="table-auto border mt-2 ml-6">
            <thead>
              <tr class="bg-gray-100">
                <th class="px-4 py-2 border">ID</th>
                <th class="px-4 py-2 border">年份</th>
                <th class="px-4 py-2 border">学期</th>
                <th class="px-4 py-2 border">学时</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="teacher in cwt.teachers" :key="teacher.id">
                <td class="px-4 py-2 border">{{ teacher.id }}</td>
                <td class="px-4 py-2 border">{{ teacher.year }}</td>
                <td class="px-4 py-2 border">{{ mapSemester(teacher.semester) }}</td>
                <td class="px-4 py-2 border">{{ teacher.hours }}</td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'

const courseName = ref('')
const cwt = ref(null)

const propertyMap = {
  1: '本科生课程',
  2: '研究生课程',
};

const semesterMap = {
  1: '春季学期',
  2: '夏季学期',
  3: '秋季学期',
};

function mapProperty(code) {
  return propertyMap[code] || '未知';
}

function mapSemester(code) {
  return semesterMap[code] || '未知';
}

const queryCourse = async () => {
  const username = localStorage.getItem('username')
  if (!courseName.value || !username) {
    alert('请填写课程名并确保已登录')
    return
  }

  const res = await fetch('/api/courses/query', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ name: courseName.value })
  })

  if (res.ok) {
    cwt.value = await res.json()
  } else {
    alert('查询失败，请检查权限或课程名')
  }
}
</script>