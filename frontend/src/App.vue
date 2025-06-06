<!-- <template>
  <div class="min-h-screen flex flex-col">
    <header class="bg-blue-600 text-white p-4 text-xl font-bold">科研管理系统</header>

    <main class="flex flex-1">
      <aside class="w-60 bg-gray-100 border-r p-4 space-y-2">
        <button
          v-for="tab in tabs"
          :key="tab.name"
          @click="currentTab = tab.name"
          class="block w-full text-left px-4 py-2 rounded hover:bg-blue-200"
          :class="{ 'bg-blue-500 text-white': currentTab === tab.name }"
        >
          {{ tab.label }}
        </button>
      </aside>

      <section class="flex-1 p-6 overflow-auto">
        <component :is="currentTabComponent" />
      </section>
    </main>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue'

// 导入所有模块组件
import TeacherSection from './components/TeacherSection.vue'
// import UserSection from '@/components/UserSection.vue'
// 可继续导入 ProjectSection 等...

const tabs = [
  { name: 'TeacherSection', label: '教师管理' },
  // { name: 'UserSection', label: '用户管理' },
  // 后续可添加更多模块
]

const currentTab = ref('TeacherSection')

const currentTabComponent = computed(() => {
  return {
    TeacherSection,
    UserSection
  }[currentTab.value]
})
</script> -->


<template>
  <div>
    <router-view />
  </div>
</template>

<script setup>
</script>