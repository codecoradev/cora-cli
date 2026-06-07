<script setup>
import { ref, onMounted, onUnmounted } from 'vue'

const props = defineProps({
  phrases: { type: Array, required: true },
  typingSpeed: { type: Number, default: 50 },
  deletingSpeed: { type: Number, default: 30 },
  pauseDuration: { type: Number, default: 2000 },
})

const display = ref('')
const phraseIndex = ref(0)
let charIndex = 0
let isDeleting = false
let timer = null

function tick() {
  const current = props.phrases[phraseIndex.value]
  
  if (!isDeleting) {
    display.value = current.slice(0, charIndex + 1)
    charIndex++
    if (charIndex >= current.length) {
      isDeleting = true
      timer = setTimeout(tick, props.pauseDuration)
      return
    }
  } else {
    display.value = current.slice(0, charIndex - 1)
    charIndex--
    if (charIndex <= 0) {
      isDeleting = false
      phraseIndex.value = (phraseIndex.value + 1) % props.phrases.length
    }
  }
  
  timer = setTimeout(tick, isDeleting ? props.deletingSpeed : props.typingSpeed)
}

onMounted(() => {
  timer = setTimeout(tick, 500)
})

onUnmounted(() => {
  if (timer) clearTimeout(timer)
})
</script>

<template>
  <span class="typing-text">{{ display }}<span class="cursor">▊</span></span>
</template>

<style scoped>
.typing-text {
  font-family: 'SF Mono', 'Fira Code', 'JetBrains Mono', monospace;
}

.cursor {
  animation: blink 1s step-end infinite;
  color: var(--vp-c-brand-1);
}

@keyframes blink {
  50% { opacity: 0; }
}
</style>
