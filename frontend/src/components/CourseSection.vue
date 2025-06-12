<template>
  <div class="w-full p-6">
    <h2 class="text-2xl font-bold mb-4 text-center">课程管理</h2>

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
import CourseQuery from './CourseQuery.vue'
import CourseInsert from './CourseInsert.vue'
import CourseUpdate from './CourseUpdate.vue'
import CourseDelete from './CourseDelete.vue'

const tabs = ['查询', '插入', '更改', '删除']
const activeTab = ref('查询')

const getComponent = (tab) => {
  switch (tab) {
    case '查询': return CourseQuery
    case '插入': return CourseInsert
    case '更改': return CourseUpdate
    case '删除': return CourseDelete
    default: return null
  }
}
</script>
