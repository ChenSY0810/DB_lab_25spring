<template>
  <div class="flex justify-center items-start min-h-screen pt-24 bg-white">
    <div class="bg-white p-8 rounded shadow-md w-full max-w-md">
      <h2 class="text-2xl font-bold mb-4 text-center">用户信息</h2>
      
      <div class="mb-4">
        <p><strong>用户名：</strong>{{ username }}</p>
        <p><strong>用户ID：</strong>{{ userId }}</p>
      </div>

      <h3 class="text-lg font-semibold mb-2">修改密码</h3>
      <form @submit.prevent="changePassword">
        <input
          v-model="newPassword"
          type="password"
          placeholder="新密码"
          class="w-full p-2 border rounded mb-3"
        />
        <button
          type="submit"
          class="w-full bg-blue-500 text-white py-2 rounded hover:bg-blue-600"
        >
          确认修改
        </button>
      </form>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'

const username = localStorage.getItem('username') || '未登录'
const userId = localStorage.getItem('user_id') || '未知'

const newPassword = ref('')

function changePassword() {
  // 实际应调用后端接口
  alert(`密码已修改为：${newPassword.value}`)
}
</script>
