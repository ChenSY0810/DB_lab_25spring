<template>
  <div class="space-y-4">
    <h3 class="text-lg font-semibold">查询论文</h3>

    <input
      v-model="paperName"
      placeholder="请输入论文名称"
      class="w-full p-2 border rounded"
    />

    <button @click="queryPaper" class="px-4 py-2 bg-blue-600 text-white rounded">
      查询
    </button>

    <div v-if="pwt" class="bg-white p-4 shadow rounded mt-4">
      <div><strong>论文名称:</strong> {{ pwt.paper.paper_name }}</div>
      <div><strong>来源:</strong> {{ pwt.paper.paper_src }}</div>
      <div><strong>发表年份:</strong> {{ pwt.paper.pub_year }}</div>
      <div><strong>类型:</strong> {{ mapType(pwt.paper.paper_type) }}</div>
      <div><strong>等级:</strong> {{ mapLvl(pwt.paper.paper_level) }}</div>
      <div ><strong>参与老师(按照排名):</strong>
        <div class=" flex justify-center">
          <table class="table-auto border mt-2 ml-6">
            <thead>
              <tr class="bg-gray-100">
                <th class="px-4 py-2 border">ID</th>
                <th class="px-4 py-2 border">姓名</th>
                <th class="px-4 py-2 border">通讯作者</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="teacher in pwt.teachers" :key="teacher.teacher_id">
                <td class="px-4 py-2 border">{{ teacher.teacher_id }}</td>
                <td class="px-4 py-2 border">{{ teacher.teacher_name }}</td>
                <td class="px-4 py-2 border">{{ teacher.comm ? '✔' : 'x' }}</td>
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

const paperName = ref('')
const pwt = ref(null)

const lvlMap = {
  1: 'CCF-A',
  2: 'CCF-B',
  3: 'CCF-C',
  4: '中文CCF-A',
  5: '中文CCF-B',
  6: '无级别',
};

const typeMap = {
  1: 'full paper',
  2: 'short paper',
  3: 'poster paper',
  4: 'demo paper',
};

function mapLvl(code) {
  return lvlMap[code] || '未知';
}

function mapType(code) {
  return typeMap[code] || '未知';
}

const queryPaper = async () => {
  const username = localStorage.getItem('username')
  if (!paperName.value || !username) {
    alert('请填写项目名并确保已登录')
    return
  }

  const res = await fetch('/api/papers/query', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ name: paperName.value })
  })

  if (res.ok) {
    pwt.value = await res.json()
  } else {
    alert('查询失败，请检查权限或论文名')
  }
}
</script>
