<template>
  <div class="w-full p-8 bg-sky-100 rounded-xl shadow-md backdrop-blur-md">
    <h2 class="text-2xl font-semibold text-center mb-6">
      {{ isRegistering ? '注册新用户' : '用户登录' }}
    </h2>

    <form @submit.prevent="isRegistering ? register() : login()">
      <div class="mb-4">
        <label class="block text-sm mb-1 text-gray-600">用户名</label>
        <input
          v-model="username"
          placeholder="输入用户名"
          class="w-full px-3 py-2 border rounded-md focus:outline-none focus:ring focus:border-blue-400"
        />
      </div>

      <div class="mb-6">
        <label class="block text-sm mb-1 text-gray-600">密码</label>
        <input
          type="password"
          v-model="password"
          placeholder="输入密码"
          class="w-full px-3 py-2 border rounded-md focus:outline-none focus:ring focus:border-blue-400"
        />
      </div>

      <button
        type="submit"
        :class="isRegistering ? 'bg-green-500 hover:bg-green-600' : 'bg-blue-500 hover:bg-blue-600'"
        class="w-full text-black py-2 rounded-lg font-semibold transition"
      >
        {{ isRegistering ? '注册' : '登录' }}
      </button>
    </form>

    <div class="mt-5 text-center">
      <button
        @click="toggleRegister"
        class="text-sm text-gray-600 hover:text-blue-500 underline transition"
      >
        {{ isRegistering ? '已有账号？点击登录' : '没有账号？点击注册' }}
      </button>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import { useRouter } from 'vue-router'

const router = useRouter()
const username = ref('')
const password = ref('')
const isRegistering = ref(false)

const toggleRegister = () => {
  isRegistering.value = !isRegistering.value
  username.value = ''
  password.value = ''
}

const login = async () => { // TODO: 接入后端
  if (username.value === 'admin' && password.value === 'admin') {
    localStorage.setItem('token', 'hardcoded-admin-token')
    localStorage.setItem('username', 'admin')
    router.push('/admin')
  } else if (username.value === 'user' && password.value === 'user') {
    localStorage.setItem('token', 'hardcoded-user-token')
    localStorage.setItem('username', 'user')
    router.push('/main')
  } else {
    alert('用户名或密码错误')
  }
}

const register = async () => {
  if (!username.value || !password.value) { // TODO: 向后端检查是否重复名字
    alert('请输入用户名和密码')
    return
  }

  console.log(`注册新用户：${username.value}, 密码：${password.value}`)
  alert(`用户 ${username.value} 注册成功，请登录`)
  isRegistering.value = false
}
</script>
