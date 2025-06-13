<template>
  <div class="space-y-4">
    <h3 class="text-lg font-semibold">更改项目</h3>

    <div class="flex gap-2">
      <input v-model="projectName" placeholder="输入项目名称" class="p-2 border rounded w-2/3" />
      <button @click="fetchProject" class="px-4 py-2 bg-blue-600 text-white rounded">加载项目</button>
    </div>

    <div v-if="loaded" class="grid grid-cols-2 gap-4">
      <input v-model="project.source" placeholder="源" required class="p-2 border rounded" />
      <input v-model="project.start_year" placeholder="开始年份" required type="number" class="p-2 border rounded" />
      <input v-model="project.end_year" placeholder="结束年份（可空）" type="number" class="p-2 border rounded" />

      <select v-model="project.project_type" required class="p-2 border rounded">
        <option disabled value="">请选择项目类型</option>
        <option :value="1">国家级项目</option>
        <option :value="2">省部级项目</option>
        <option :value="3">市厅级项目</option>
        <option :value="4">企业合作项目</option>
        <option :value="5">其它类型项目</option>
      </select>

      <select v-model="project.secret_level" required class="p-2 border rounded">
        <option disabled value="">请选择保密级别</option>
        <option :value="1">不保密</option>
        <option :value="2">保密</option>
      </select>
    </div>

    <div v-if="loaded">
      <h4 class="font-medium mb-2">编辑教师顺位与经费</h4>
      <div v-for="(t, index) in teachers" :key="index" class="flex gap-2 mb-2 items-center">
        <input :value="t.id" disabled class="p-2 border rounded w-1/3 bg-gray-100" />
        <input v-model="t.fund" placeholder="经费" type="number" class="p-2 border rounded w-1/3" />
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

const projectName = ref('')
const loaded = ref(false)
const project = ref({
  start_year: '',
  end_year: '',
  project_type: '',
  secret_level: '',
  name: '',
  source: '',
})
const teachers = ref([])

const teacherOptions = ref([])
const newTeacherId = ref('')

var oldname = ''

const fetchProject = async () => {
  const username = localStorage.getItem('username')
  loaded.value = false
  const res = await fetch(`/api/projects/query?username=${encodeURIComponent(username)}`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ name: projectName.value })
  })

  if (!res.ok) {
    alert('加载失败')
    return
  }

  const data = await res.json()
  project.value = {
    start_year: data.project.start_year,
    end_year: data.project.end_year,
    project_type: data.project.project_type,
    secret_level: data.project.secret_level,
    name: data.project.project_name,
    source: data.project.project_src,
  }
  teachers.value = data.teachers.map(t => ({id: t.teacher_id, fund: t.fund})) // [{ id: 1001, fund: 20000 }, ...]
  oldname = projectName.value
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
  teachers.value.push({ id: newTeacherId.value, fund: '' })
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

  if (project.value.end_year != null && project.value.end_year < project.value.start_year) {
    alert('开始年份应小于等于结束年份')
    return
  }

  project.value.name = projectName.value;
  const payload_1 = {
    ...project.value,
    teachers: teachers.value
  }

  const payload = {
    old_name: oldname,
    new_project: payload_1
  }
  
  console.log(JSON.stringify(payload, null, 2));

  const res = await fetch(`/api/projects/update?username=${encodeURIComponent(username)}`, {
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
