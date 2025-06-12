<template>
  <div class="space-y-4">
    <h3 class="text-lg font-semibold">更改课程</h3>

    <div class="flex gap-2">
      <input v-model="courseName" placeholder="输入课程名称" class="p-2 border rounded w-2/3" />
      <button @click="fetchCourse" class="px-4 py-2 bg-blue-600 text-white rounded">加载课程</button>
    </div>

    <div v-if="loaded" class="grid grid-cols-2 gap-4">

      <select v-model.number="course.course_property" required class="p-2 border rounded">
        <option disabled value="">请选择课程性质</option>
        <option :value=1>本科生课程</option>
        <option :value=2>研究生课程</option>
      </select>
    </div>

    <div v-if="loaded">
      <h4 class="font-medium mb-2">编辑主讲教师</h4>
      <div v-for="(t, index) in teachers" :key="index" class="flex gap-2 mb-2 items-center">
        <input :value="t.id" disabled class="p-2 border rounded w-1/4 bg-gray-100" />
        <input v-model.number="t.year" type="number" required class="p-2 border rounded w-1/5" placeholder="课时" />
        <input v-model.number="t.hours" type="number" required class="p-2 border rounded w-1/5" placeholder="课时" />
        <select v-model.number="t.semester" class="p-2 border rounded w-1/5">
          <option :value=1>春季</option>
          <option :value=2>夏季</option>
          <option :value=3>秋季</option>
        </select>
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

const courseName = ref('')
const loaded = ref(false)
const oldname = ref('')
const course = ref({
  name: '',
  hours: 0,
  property: '',
  source: ''
})
const teachers = ref([])

const teacherOptions = ref([])
const newTeacherId = ref('')

const fetchCourse = async () => {
  loaded.value = false
  const res = await fetch('/api/courses/query', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ name: courseName.value })
  })

  if (!res.ok) {
    alert('加载失败')
    return
  }

  const data = await res.json()
  course.value = {
    name: data.course.course_name,
    course_property: data.course.course_property,
  }
  teachers.value = data.teachers.map(t => ({
    id: t.id,
    hours: t.hours,
    year: t.year,
    semester: t.semester
  }))
  oldname.value = courseName.value
  loaded.value = true
}

const loadTeacherOptions = async () => {
  const res = await fetch('/api/teachers', {
    method: 'GET'
  })
  if (res.ok) {
    teacherOptions.value = await res.json()
  }
}

const addTeacher = () => {
  if (!newTeacherId.value) return
  teachers.value.push({ id: newTeacherId.value, year: 1, semester: 1, hours: 0 })
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

  course.value.name = courseName.value
  const payload = {
    old_name: oldname.value,
    new_course: {
      ...course.value,
      teachers: teachers.value
    }
  }

  console.log(JSON.stringify(payload, null, 2));

  const res = await fetch(`/api/courses/update?username=${encodeURIComponent(username)}`, {
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
