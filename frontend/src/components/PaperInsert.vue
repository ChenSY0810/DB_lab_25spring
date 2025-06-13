<template>
  <div class="space-y-4">
    <h3 class="text-lg font-semibold">插入新论文</h3>

    <div class="grid grid-cols-2 gap-4">
      <input v-model="paper.name" placeholder="论文名称" required class="p-2 border rounded" />
      <input v-model="paper.source" placeholder="论文源" required class="p-2 border rounded" />
      <input v-model="paper.pub_year" placeholder="发表年份" required type="number" class="p-2 border rounded" />

      <select v-model.number="paper.paper_level" required class="p-2 border rounded">
        <option disabled value=null>请选择论文级别</option>
        <option :value=1>CCF-A</option>
        <option :value=2>CCF-B</option>
        <option :value=3>CCF-C</option>
        <option :value=4>中文CCF-A</option>
        <option :value=5>中文CCF-B</option>
        <option :value=6>无级别</option>
      </select>

      <select v-model.number="paper.paper_type" class="p-2 border rounded">
        <option disabled value=null>请选择论文类型</option>
        <option :value=1>full paper</option>
        <option :value=2>short paper</option>
        <option :value=3>poster paper</option>
        <option :value=4>demo paper</option>
      </select>
    </div>

    <div>
     <h4 class="font-medium mb-2"><b>添加教师参与顺位</b></h4>
      <p class="text-sm text-gray-500 mb-2">只能选择一位教师作为通讯作者</p>

      <div v-for="(t, index) in teachers" :key="index" class="flex gap-2 mb-2 items-center">
        <select v-model="t.id" required class="p-2 border rounded w-1/2">
          <option disabled value="">选择教师</option>
          <option
            v-for="option in teacherOptions"
            :key="option.teacher_id"
            :value="option.teacher_id"
          >
            {{ option.teacher_name }} ({{ option.teacher_id }})
          </option>
        </select>

        <label class="flex items-center space-x-1 w-1/3">
          <input
            type="radio"
            name="comm"
            :checked="t.comm"
            @change="setComm(index)"
          />
          <span>通讯作者</span>
        </label>

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

const paper = ref({
  name: '',
  source: '',
  pub_year: '',
  paper_type: null,
  paper_level: null
})

const teachers = ref([
  { id: '', comm: false }
])

const teacherOptions = ref([])

const addTeacher = () => {
  teachers.value.push({ id: '', comm: false })
}

const removeTeacher = (index) => {
  teachers.value.splice(index, 1)
}

function setComm(index) {
  teachers.value.forEach((t, i) => {
    t.comm = i === index;
  });
}

const submit = async () => {
  const p = paper.value;

  if (
    p.name === '' ||
    p.source === '' ||
    p.pub_year == null ||
    p.paper_type == null ||
    p.paper_level == null
  ) {
    alert("请填写完整论文信息。")
    return false;
  }

  const payload = {
    ...paper.value,
    teachers: teachers.value
  }

  console.log(JSON.stringify(payload, null, 2));

  const res = await fetch('/api/papers', {
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
      pub_year: '',
      paper_type: null,
      paper_level: null
    }
    teachers.value = [{ id: '', comm: false }]
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
