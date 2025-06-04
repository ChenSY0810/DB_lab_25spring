<template>
  <div class="min-h-screen bg-gray-50">
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

function goBack() {
  if ( username === 'admin' ) {
    router.push('/admin')
  } else {
    router.push('/main')
  }
}

function logout() {
  localStorage.removeItem('token')
  localStorage.removeItem('username')
  router.push('/login')
}

function goToProfile() {
  router.push('/user-profile')
}
</script>