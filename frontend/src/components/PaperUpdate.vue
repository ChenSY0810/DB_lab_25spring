<template>
  <div class="space-y-4">
    <h3 class="text-lg font-semibold">更改论文</h3>

    <div class="flex gap-2">
      <input v-model="paperName" placeholder="输入论文名称" class="p-2 border rounded w-2/3" />
      <button @click="fetchPaper" class="px-4 py-2 bg-blue-600 text-white rounded">加载论文</button>
    </div>

    <div v-if="loaded" class="grid grid-cols-2 gap-4">
      <input v-model.number="paper.source" placeholder="源" required class="p-2 border rounded" />
      <input v-model.number="paper.pub_year" placeholder="发表年份" required class="p-2 border rounded" />

      <select v-model.number="paper.paper_type" required class="p-2 border rounded">
        <option disabled value="">请选择论文类型</option>
        <option :value=1>full paper</option>
        <option :value=2>short paper</option>
        <option :value=3>poster paper</option>
        <option :value=4>demo paper</option>
      </select>

      <select v-model.number="paper.paper_level" required class="p-2 border rounded">
        <option disabled value="">请选择论文等级</option>
        <option :value=1>CCF-A</option>
        <option :value=2>CCF-B</option>
        <option :value=3>CCF-C</option>
        <option :value=4>中文CCF-A</option>
        <option :value=5>中文CCF-B</option>
        <option :value=6>无级别</option>
      </select>
    </div>

    <div v-if="loaded">
      <h4 class="font-medium mb-2">编辑教师顺位与通信作者</h4>
        <div v-for="(t, index) in teachers" :key="index" class="flex gap-2 mb-2 items-center">
          <input :value="t.id" disabled class="p-2 border rounded w-1/3 bg-gray-100" />

          <input
            type="checkbox"
            :checked="t.comm"
            @change="setCommAuthor(index)"
            class="w-5 h-5"
          />
          <span>通信作者</span>

          <button @click="removeTeacher(index)" class="text-red-500">移除</button>
        </div>

        <div class="flex items-center gap-2">
          <select v-model="newTeacherId" class="p-2 border rounded w-1/2">
            <option disabled value="">选择新教师</option>
            <option v-for="t in teacherOptions" :key="t.teacher_id" :value="t.teacher_id">
              {{ t.teacher_name }}（{{ t.teacher_id }}）
            </option>
          </select>
          <button @click="addTeacher" class="px-3 py-1 bg-green-500 text-white rounded">添加教师</button>
        </div>
    </div>

    <button v-if="loaded" @click="submit" class="px-4 py-2 bg-blue-600 text-white rounded mt-4">提交更改</button>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'

function setCommAuthor(index) {
  teachers.value.forEach((t, i) => {
    t.comm = (i === index);
  });
}

const paperName = ref('')
const loaded = ref(false)
const paper = ref({
  pub_year: '',
  paper_type: '',
  paper_level: '',
  name: '',
  source: '',
})
const teachers = ref([])

const teacherOptions = ref([])
const newTeacherId = ref('')

var oldname = ''

const fetchPaper = async () => {
  loaded.value = false
  const res = await fetch('/api/papers/query', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ name: paperName.value })
  })

  if (!res.ok) {
    alert('加载失败')
    return
  }

  const data = await res.json()
  paper.value = {
    pub_year: Number(data.paper.pub_year),
    paper_type: data.paper.paper_type,
    paper_level: data.paper.paper_level,
    name: data.paper.paper_name,
    source: data.paper.paper_src,
  }
  teachers.value = data.teachers.map(t => ({id: t.teacher_id, comm: t.comm}))
  oldname = paperName.value
  loaded.value = true
}

const loadTeacherOptions = async () => {
  const res = await fetch('/api/teachers', {
    headers: {
      method: 'GET'
    }
  })
  if (res.ok) {
    teacherOptions.value = await res.json()
  }
}

const addTeacher = () => {
  if (!newTeacherId.value) return
  if (teachers.value.find(t => t.id === newTeacherId.value)) {
    alert('该教师已存在')
    return
  }
  teachers.value.push({ id: newTeacherId.value, comm: false })
  newTeacherId.value = ''
}

const removeTeacher = (index) => {
  teachers.value.splice(index, 1)
}

const submit = async () => {

  const username = localStorage.getItem('username')
  if (!username) {
    alert('未登录')
    return
  }

  paper.value.name = paperName.value;
  const payload_1 = {
    ...paper.value,
    teachers: teachers.value
  }

  const payload = {
    old_name: oldname,
    new_paper: payload_1
  }
  
  console.log(JSON.stringify(payload, null, 2));

  const res = await fetch(`/api/papers/update?username=${encodeURIComponent(username)}`, {
    method: 'PUT',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(payload)
  })

  if (res.ok) {
    alert('更改成功')
    loaded.value = false
  } else {
    alert('更改失败')
  }
}


onMounted(loadTeacherOptions)
</script>
