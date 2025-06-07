<template>
  <div>
    <h2 class="text-xl font-bold mb-4">账号绑定</h2>
    <LinkForm @linked="loadUsers" />

    <div class="flex mt-6 mx-auto justify-center ">
      <table class="w-all"> 
        <tc>
          <td class="pr-4">
            <h3 class="text-lg font-semibold mb-2">教师列表</h3>
            <table class="w-full border text-left">
              <thead class="bg-gray-100">
                <tr>
                  <th class="p-2">工号</th>
                  <th class="p-2">姓名</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="teacher in teachers" :key="teacher.teacher_id" class="border-t">
                  <td class="p-2">{{ teacher.teacher_id }}</td>
                  <td class="p-2">{{ teacher.teacher_name }}</td>
                </tr>
              </tbody>
            </table>
          </td>
        </tc>
        <tc>
          <td class="pl-4">
            <h3 class="text-lg font-semibold mb-2">用户列表</h3>
            <table class="w-full border text-left">
              <thead class="bg-gray-100">
                <tr>
                  <th class="p-2">用户名</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="user in users" :key="user.user_id" class="border-t">
                  <td class="p-2">{{ user.username }}</td>
                </tr>
              </tbody>
            </table>
          </td>
        </tc>
      </table>
    </div>

    
    
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import LinkForm from './LinkForm.vue'

const teachers = ref([])
const users = ref([])


async function loadTeachers() {
  const res = await fetch('/api/teachers', {
    headers: {
      method: 'GET',
    }
  })
  teachers.value = await res.json()
}

async function loadUsers() {
  const res = await fetch('/api/users', {
    headers: {
      method: 'GET',
    }
  })
  users.value = await res.json()
}

onMounted(() => {
  loadTeachers()
  loadUsers()
})
</script>
