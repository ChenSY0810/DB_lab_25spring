<template>
  <div class="space-y-4">
    <h3 class="text-lg font-semibold">插入新项目</h3>

    <div class="grid grid-cols-2 gap-4">
      <input v-model="project.name" placeholder="项目名称" required class="p-2 border rounded" />
      <input v-model="project.source" placeholder="项目来源" required class="p-2 border rounded" />
      <input v-model="project.start_year" placeholder="开始年份" required type="number" class="p-2 border rounded" />
      <input v-model="project.end_year" placeholder="结束年份（可空）" type="number" class="p-2 border rounded" />

      <select v-model.number="project.project_type" required class="p-2 border rounded">
        <option disabled value="">请选择项目类型</option>
        <option :value=1>国家级项目</option>
        <option :value=2>省部级项目</option>
        <option :value=3>市厅级项目</option>
        <option :value=4>企业合作项目</option>
        <option :value=5>其它类型项目</option>
      </select>

      <select v-model.number="project.secret_level" class="p-2 border rounded">
        <option disabled value="">请选择保密级别</option>
        <option :value=1>不保密</option>
        <option :value=2>保密</option>
      </select>
    </div>

    <div>
      <h4 class="font-medium mb-2"><b>添加教师参与顺位</b></h4>
      <div v-for="(t, index) in teachers" :key="index" class="flex gap-2 mb-2 items-center">
        <select v-model="t.id" required class="p-2 border rounded w-1/2">
          <option disabled value="">选择教师</option>
          <option v-for="option in teacherOptions" :key="option.teacher_id" :value="option.teacher_id">
            {{ option.teacher_name }}({{ option.teacher_id }})
          </option>
        </select>
        <input v-model="t.fund" placeholder="经费" type="number" required class="p-2 border rounded w-1/3" />
        <button @click="removeTeacher(index)" class="bg-red-500 text-white">移除</button>
      </div>
      <button @click="addTeacher" class="px-3 py-1 bg-green-500 text-white rounded">添加教师</button>
    </div>

    <button @click="submit" class="px-4 py-2 bg-blue-600 text-white rounded mt-4">提交</button>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'

const project = ref({
  name: '',
  source: '',
  start_year: '',
  end_year: null,
  project_type: '',
  secret_level: ''
})

const teachers = ref([
  { id: '', fund: '' }
])

const teacherOptions = ref([])

const addTeacher = () => {
  teachers.value.push({ id: '', fund: '' })
}

const removeTeacher = (index) => {
  teachers.value.splice(index, 1)
}

const submit = async () => {
  if (
    project.value.name === '' ||
    project.value.source === '' ||
    project.value.start_year === '' ||
    project.value.project_type === '' ||
    project.value.secret_level === ''
  ) {
    alert("请填写完整项目信息。")
    return false
  }
  
  if (project.value.end_year === '') {
    project.value.end_year = null
  }

  const payload = {
    ...project.value,
    teachers: teachers.value
  }

console.log(JSON.stringify(payload, null, 2));

  const res = await fetch('/api/projects', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(payload)
  })

  if (res.ok) {
    alert('插入成功')
    project.value = {
      name: '',
      source: '',
      start_year: '',
      end_year: null,
      project_type: '',
      secret_level: ''
    }
    teachers.value = [{ id: '', fund: '' }]
  } else {
    const errorMsg = await res.text()
    alert(errorMsg)
  }
}

const loadTeacherOptions = async () => {
  const res = await fetch('/api/teachers', {
    method: 'GET',
  })
  if (res.ok) {
    teacherOptions.value = await res.json()
  }
}

onMounted(loadTeacherOptions)
</script>
