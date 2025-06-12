<template>
  <div class="space-y-4">
    <h3 class="text-lg font-semibold">插入新课程</h3>

    <div class="grid grid-cols-2 gap-4">
      <input v-model="course.name" placeholder="课程名称" required class="p-2 border rounded" />

      <select v-model.number="course.course_property" required class="p-2 border rounded">
        <option disabled value=null>请选择课程性质</option>
        <option :value=1>本科生课程</option>
        <option :value=2>研究生课程</option>
      </select>
    </div>

    <div>
      <h4 class="font-medium mb-2"><b>添加主讲教师信息</b></h4>

      <div v-for="(t, index) in teachers" :key="index" class="flex gap-2 mb-2 items-center">
        <select v-model="t.id" required class="p-2 border rounded w-1/3">
          <option disabled value="">选择教师</option>
          <option
            v-for="option in teacherOptions"
            :key="option.teacher_id"
            :value="option.teacher_id"
          >
            {{ option.teacher_name }} ({{ option.teacher_id }})
          </option>
        </select>

        <input v-model.number="t.year" placeholder="年份" type="number" required class="p-2 border rounded w-1/5" />

        <select v-model.number="t.semester" required class="p-2 border rounded w-1/5">
          <option disabled value=null>学期</option>
          <option :value=1>春季</option>
          <option :value=2>夏季</option>
          <option :value=3>秋季</option>
        </select>

        <input v-model.number="t.hours" placeholder="学时" type="number" required class="p-2 border rounded w-1/5" />

        <button @click="removeTeacher(index)" class="bg-red-500 text-white px-2 py-1 rounded">
          移除
        </button>
      </div>
      <button @click="addTeacher" class="px-3 py-1 bg-green-500 text-white rounded">添加教师</button>
    </div>

    <button @click="submit" class="px-4 py-2 bg-blue-600 text-white rounded mt-4">提交</button>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'

const course = ref({
  name: '',
  course_property: null,
})

const teachers = ref([
  { id: '', year: new Date().getFullYear(), semester: null, hours: null }
])

const teacherOptions = ref([])

const addTeacher = () => {
  teachers.value.push({ id: '', year: new Date().getFullYear(), semester: null, hours: null })
}

const removeTeacher = (index) => {
  teachers.value.splice(index, 1)
}

const submit = async () => {
  const c = course.value;
  if (!c.name || !c.course_property) {
    alert("请填写完整课程信息。");
    return;
  }
  for (const t of teachers.value) {
    if (!t.id || !t.year || !t.semester || !t.hours) {
      alert("请填写完整教师课程信息。");
      return;
    }
  }

  const payload = {
    ...course.value,
    teachers: teachers.value
  }

  console.log(JSON.stringify(payload, null, 2));

  const res = await fetch('/api/courses', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(payload)
  })

  if (res.ok) {
    alert('插入成功')
    course.value = { name: '', source: '', course_property: null }
    teachers.value = [{ id: '', year: new Date().getFullYear(), semester: null, hours: null }]
  } else {
    const errorMsg = await res.text()
    alert(errorMsg)
  }
}

const loadTeacherOptions = async () => {
  const res = await fetch('/api/teachers')
  if (res.ok) {
    teacherOptions.value = await res.json()
  }
}

onMounted(loadTeacherOptions)
</script>
