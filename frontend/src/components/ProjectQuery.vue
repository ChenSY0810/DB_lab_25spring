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

    <div v-if="pwt" class="bg-white p-4 shadow rounded mt-4">
      <div><strong>项目名称:</strong> {{ pwt.project.project_name }}</div>
      <div><strong>来源:</strong> {{ pwt.project.project_src }}</div>
      <div><strong>总经费:</strong> {{ pwt.project.total_fund }}</div>
      <div><strong>类型:</strong> {{ mapplvl(pwt.project.project_type) }}</div>
      <div><strong>保密级别:</strong> {{ mapsecret(pwt.project.secret_level) }}</div>
      <div><strong>时间:</strong> {{ pwt.project.start_year }}{{ pwt.project.end_year ? '-' + pwt.project.end_year : ' 至今' }}</div>
      <div ><strong>参与老师(按照排名):</strong>
        <div class=" flex justify-center">
          <table class="table-auto border mt-2 ml-6">
            <thead>
              <tr class="bg-gray-100">
                <th class="px-4 py-2 border">ID</th>
                <th class="px-4 py-2 border">姓名</th>
                <th class="px-4 py-2 border">经费</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="teacher in pwt.teachers" :key="teacher.teacher_id">
                <td class="px-4 py-2 border">{{ teacher.teacher_id }}</td>
                <td class="px-4 py-2 border">{{ teacher.teacher_name }}</td>
                <td class="px-4 py-2 border">{{ teacher.fund }}</td>
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

const plvlMap = {
  1: '国家级项目',
  2: '省部级项目',
  3: '市厅级项目',
  4: '企业合作项目',
  5: '其他类型项目',
};

const secretMap = {
  1: '不保密',
  2: '保密'
};

function mapplvl(code) {
  return plvlMap[code] || '未知';
}

function mapsecret(code) {
  return secretMap[code] || '未知';
}

const projectName = ref('')
const pwt = ref(null)

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
    pwt.value = await res.json()
  } else {
    alert('查询失败，请检查权限或项目名')
  }
}
</script>
