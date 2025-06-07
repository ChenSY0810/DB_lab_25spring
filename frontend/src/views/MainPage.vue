<template>
  <div class="w-full bg-gray-50 p-8">
    
    <div class="flex gap-4 mb-6 justify-center">
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
        class="rounded-xl p-6 shadow-md"
        :class="tabColorMap[activeTab]"
      />
    </transition>
  </div>
</template>

<script setup>
import ProjectSection from '@/components/ProjectSection.vue'
// import PaperSection from '@/components/PaperSection.vue'
// import CourseSection from '@/components/CourseSection.vue'
// import RangeQuerySection from '@/components/RangeQuerySection.vue'
import { ref } from 'vue'

const tabs = ['项目', '论文', '课程', '范围查询']
const activeTab = ref('项目')

const getComponent = (tab) => {
  switch (tab) {
    case '项目': return ProjectSection
    case '论文': return PaperSection
    case '课程': return CourseSection
    case '范围查询': return RangeQuerySection
    default: return null
  }
}

const tabColorMap = {
  '项目': 'bg-yellow-50',
  '论文': 'bg-red-50',
  '课程': 'bg-purple-50',
  '范围查询': 'bg-magenta-50'
}
</script>

<style scoped>
.fade-enter-active, .fade-leave-active {
  transition: opacity 0.4s;
}
.fade-enter-from, .fade-leave-to {
  opacity: 0;
}
</style>
