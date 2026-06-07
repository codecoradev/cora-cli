<script setup>
import { ref, onMounted, onUnmounted } from 'vue'

const props = defineProps({
  tag: { type: String, default: 'div' },
  delay: { type: Number, default: 0 },
  duration: { type: Number, default: 600 },
  direction: { type: String, default: 'up' }, // up, down, left, right
  distance: { type: Number, default: 24 },
  once: { type: Boolean, default: true },
})

const elRef = ref(null)
const isVisible = ref(false)
let observer = null

onMounted(() => {
  observer = new IntersectionObserver(
    (entries) => {
      entries.forEach((entry) => {
        if (entry.isIntersecting) {
          setTimeout(() => { isVisible.value = true }, props.delay)
          if (props.once) observer?.unobserve(entry.target)
        } else if (!props.once) {
          isVisible.value = false
        }
      })
    },
    { threshold: 0.1 }
  )
  if (elRef.value) observer.observe(elRef.value)
})

onUnmounted(() => {
  observer?.disconnect()
})

const translateMap = {
  up: `translateY(${props.distance}px)`,
  down: `translateY(-${props.distance}px)`,
  left: `translateX(${props.distance}px)`,
  right: `translateX(-${props.distance}px)`,
}
</script>

<template>
  <component
    :is="tag"
    ref="elRef"
    :style="{
      opacity: isVisible ? 1 : 0,
      transform: isVisible ? 'translate(0)' : translateMap[direction],
      transition: `opacity ${duration}ms cubic-bezier(0.16, 1, 0.3, 1), transform ${duration}ms cubic-bezier(0.16, 1, 0.3, 1)`,
    }"
  >
    <slot />
  </component>
</template>
