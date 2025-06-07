<template>
  <div class="w-full p-6">
    <!-- 页面标题 -->
    <h2 class="text-2xl font-bold mb-4 text-center">项目管理</h2>

    <!-- 标签按钮 -->
    <div class="flex justify-center gap-4 mb-6">
      <button
        v-for="tab in tabs"
        :key="tab"
        @click="activeTab = tab"
        :class="[
          'px-6 py-2 rounded-full font-semibold shadow transition',
          activeTab === tab ? 'bg-blue-600 text-white' : 'bg-white text-gray-700 hover:bg-blue-100'
        ]"
      >
        {{ tab }}
      </button>
    </div>

    <!-- 组件区域 -->
    <transition name="fade" mode="out-in">
      <component
        :is="getComponent(activeTab)"
        :key="activeTab"
        class="rounded-xl p-6 shadow-md bg-white"
      />
    </transition>
  </div>
</template>

<script setup>
import { ref } from 'vue'

// 子组件导入
import ProjectQuery from './ProjectQuery.vue'
import ProjectInsert from './ProjectInsert.vue'
import ProjectUpdate from './ProjectUpdate.vue'
import ProjectDelete from './ProjectDelete.vue'

const tabs = ['查询', '插入', '更改', '删除']
const activeTab = ref('查询')

const getComponent = (tab) => {
  switch (tab) {
    case '查询': return ProjectQuery
    case '插入': return ProjectInsert
    case '更改': return ProjectUpdate
    case '删除': return ProjectDelete
    default: return null
  }
}
</script>
