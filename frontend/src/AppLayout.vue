<template>
  <div class="w-full mx-auto bg-gray-50">
    <div class="w-full flex justify-between items-center px-6 py-3 bg-white shadow-md fixed top-0 left-0 right-0 z-50">
      <button @click="goBack" class="text-xl text-blue-700">
        <bold>教师信息系统</bold>
      </button>
      <div class="flex items-center space-x-4">
        <button @click="goToProfile" class="text-blue-600 hover:underline">
          {{ username }}
        </button>
        <button @click="logout" class="px-3 py-1 bg-red-400 text-white rounded hover:bg-red-500 text-sm">
          登出
        </button>
      </div>
    </div>

    <router-view class="p-6" />
  </div>
</template>

<script setup>
import { useRouter } from 'vue-router'

const router = useRouter()
const username = localStorage.getItem('username') || '未登录'
const pri = Number(localStorage.getItem('privilege'))

function goBack() {
  if ( pri === 2 ) {
    router.push('/admin')
  } else if (pri === 1) {
    router.push('/main')
  } else {
    router.push('/login')
  }
}

function logout() {
  localStorage.removeItem('privilege')
  localStorage.removeItem('username')
  router.push('/login')
}

function goToProfile() {
  router.push('/user-profile')
}
</script>