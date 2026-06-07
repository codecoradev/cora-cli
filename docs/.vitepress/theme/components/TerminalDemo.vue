<script setup>
import { ref, onMounted, onUnmounted } from 'vue'

const props = defineProps({
  lines: { type: Array, required: true },
  speed: { type: Number, default: 40 },
  loop: { type: Boolean, default: false },
})

const visibleLines = ref([])
const containerRef = ref(null)
let timer = null

onMounted(() => {
  let i = 0
  function showNext() {
    if (i < props.lines.length) {
      visibleLines.value.push(props.lines[i])
      i++
      timer = setTimeout(showNext, props.speed)
    } else if (props.loop) {
      timer = setTimeout(() => {
        visibleLines.value = []
        i = 0
        showNext()
      }, 3000)
    }
  }
  showNext()
})

onUnmounted(() => {
  if (timer) clearTimeout(timer)
})
</script>

<template>
  <div ref="containerRef" class="terminal">
    <div class="terminal-bar">
      <span class="dot red"></span>
      <span class="dot yellow"></span>
      <span class="dot green"></span>
    </div>
    <div class="terminal-body">
      <div
        v-for="(line, idx) in visibleLines"
        :key="idx"
        class="terminal-line"
        :class="[line.type || '']"
      >
        <span v-if="line.prefix" class="prefix">{{ line.prefix }}</span>
        <span v-html="line.text" />
      </div>
    </div>
  </div>
</template>

<style scoped>
.terminal {
  background: var(--vp-c-black);
  border-radius: 12px;
  overflow: hidden;
  border: 1px solid rgba(255, 255, 255, 0.08);
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.4);
}

.terminal-bar {
  display: flex;
  gap: 6px;
  padding: 10px 14px;
  background: rgba(255, 255, 255, 0.04);
  border-bottom: 1px solid rgba(255, 255, 255, 0.06);
}

.dot {
  width: 12px;
  height: 12px;
  border-radius: 50%;
}

.dot.red { background: #ff5f57; }
.dot.yellow { background: #febc2e; }
.dot.green { background: #28c840; }

.terminal-body {
  padding: 16px 20px;
  font-family: 'SF Mono', 'Fira Code', 'JetBrains Mono', monospace;
  font-size: 13px;
  line-height: 1.7;
  min-height: 120px;
}

.terminal-line {
  color: #c9d1d9;
  animation: fadeIn 0.15s ease-out;
}

.terminal-line .prefix {
  margin-right: 8px;
}

.terminal-line.cmd .prefix {
  color: #58a6ff;
  font-weight: 600;
}

.terminal-line.success .prefix {
  color: #3fb950;
}

.terminal-line.warning .prefix {
  color: #d29922;
}

.terminal-line.error .prefix {
  color: #f85149;
}

.terminal-line.muted {
  color: #8b949e;
}

.terminal-line.highlight {
  color: #58a6ff;
}

.terminal-line.dim {
  color: #484f58;
}

@keyframes fadeIn {
  from { opacity: 0; transform: translateY(4px); }
  to { opacity: 1; transform: translateY(0); }
}

@media (max-width: 768px) {
  .terminal {
    border-radius: 8px;
  }

  .terminal-body {
    padding: 12px 14px;
    font-size: 11px;
    line-height: 1.6;
    min-height: 80px;
    overflow-x: auto;
    -webkit-overflow-scrolling: touch;
  }

  .terminal-bar {
    padding: 8px 12px;
  }

  .dot {
    width: 10px;
    height: 10px;
  }
}

@media (max-width: 480px) {
  .terminal-body {
    font-size: 10px;
    padding: 10px;
  }
}
</style>
