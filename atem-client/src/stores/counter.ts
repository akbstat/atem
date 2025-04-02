import { ref, computed, type Ref } from 'vue'
import { defineStore } from 'pinia'
import type { Project } from '@/api/project'

export const useCounterStore = defineStore('counter', () => {
  const count = ref(0)
  const doubleCount = computed(() => count.value * 2)
  function increment() {
    count.value++
  }

  return { count, doubleCount, increment }
})

export const useAtemStore = defineStore('atem', {
  state: (): { project: Project } => ({
    project: {
      id: 0,
      name: '',
      forms: []
    }
  })
});