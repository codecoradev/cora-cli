<script setup>
import { ref, onMounted, onUnmounted } from 'vue'
import TypingText from './TypingText.vue'
import TerminalDemo from './TerminalDemo.vue'
import FadeIn from './FadeIn.vue'
</script>

<template>
  <div class="landing">
    <!-- Hero Section -->
    <section class="hero">
      <div class="hero-inner">
        <div class="hero-badge">🧪 Open Source · MIT License</div>
        <img src="/logo.svg" alt="cora" class="hero-logo" />
        <h1 class="hero-title">
          AI Code Review
          <br />
          <span class="hero-brand">in your terminal</span>
        </h1>
        <p class="hero-subtitle">
          <TypingText
            :phrases="[
              '$ cora review --staged',
              '$ cora review --base main',
              '$ cora scan --incremental',
              '$ cora hook install',
            ]"
            :typing-speed="45"
            :deleting-speed="25"
            :pause-duration="2500"
          />
        </p>
        <div class="hero-actions">
          <a href="/getting-started" class="btn btn-primary">Get Started →</a>
          <a href="https://github.com/codecoradev/cora-cli" class="btn btn-secondary" target="_blank">GitHub</a>
        </div>
      </div>

      <FadeIn :delay="400" direction="up" :distance="40">
        <TerminalDemo :lines="[
          { prefix: '$', text: 'cora review --staged', type: 'cmd' },
          { text: '', type: 'muted' },
          { prefix: '✓', text: ' src/auth/login.rs — 2 issues found', type: 'success' },
          { prefix: '  ⚠', text: ' Line 42: Potential SQL injection', type: 'warning' },
          { prefix: '  ⚠', text: ' Line 87: Hardcoded secret detected', type: 'warning' },
          { prefix: '✓', text: ' src/utils/parser.rs — clean', type: 'success' },
          { prefix: '✓', text: ' src/api/routes.rs — 1 issue found', type: 'success' },
          { prefix: '  ✗', text: ' Line 23: Missing error handling', type: 'error' },
          { text: '', type: 'muted' },
          { text: '3 issues found in 3 files (1.2s)', type: 'muted' },
        ]" />
      </FadeIn>
    </section>

    <!-- Stats Bar -->
    <FadeIn :delay="200">
      <section class="stats-bar">
        <div class="stat">
          <span class="stat-value">🦀</span>
          <span class="stat-label">Rust Native</span>
        </div>
        <div class="stat">
          <span class="stat-value">6+</span>
          <span class="stat-label">LLM Providers</span>
        </div>
        <div class="stat">
          <span class="stat-value">12</span>
          <span class="stat-label">Built-in Rules</span>
        </div>
        <div class="stat">
          <span class="stat-value">4</span>
          <span class="stat-label">Platforms</span>
        </div>
        <div class="stat">
          <span class="stat-value">0ms</span>
          <span class="stat-label">Cold Start</span>
        </div>
      </section>
    </FadeIn>

    <!-- Install Section -->
    <FadeIn :delay="100">
      <section class="install-section">
        <h2>Install in 5 seconds</h2>
        <div class="install-cmd">
          <code>curl -fsSL https://raw.githubusercontent.com/codecoradev/cora-cli/main/install.sh | sh</code>
        </div>
      </section>
    </FadeIn>

    <!-- How It Works -->
    <section class="how-it-works">
      <h2>How it works</h2>
      <div class="steps">
        <FadeIn :delay="0">
          <div class="step">
            <div class="step-number">1</div>
            <h3>Install</h3>
            <p>Single binary, no runtime dependencies. One command to install.</p>
          </div>
        </FadeIn>
        <FadeIn :delay="150">
          <div class="step">
            <div class="step-number">2</div>
            <h3>Authenticate</h3>
            <p><code>cora auth login</code> — pick your LLM provider, enter API key. Done.</p>
          </div>
        </FadeIn>
        <FadeIn :delay="300">
          <div class="step">
            <div class="step-number">3</div>
            <h3>Review</h3>
            <p><code>cora review</code> — get instant AI feedback on your staged changes.</p>
          </div>
        </FadeIn>
      </div>
    </section>

    <!-- CTA -->
    <FadeIn>
      <section class="cta">
        <h2>Start reviewing code in seconds</h2>
        <p>No signup, no subscription. Your code never leaves your machine.</p>
        <div class="cta-actions">
          <a href="/getting-started" class="btn btn-primary">Read the Docs →</a>
          <a href="/examples" class="btn btn-secondary">See Examples</a>
        </div>
      </section>
    </FadeIn>
  </div>
</template>

<style scoped>
.landing {
  max-width: 960px;
  margin: 0 auto;
  padding: 0 24px;
}

/* Hero */
.hero {
  text-align: center;
  padding: 80px 0 60px;
}

.hero-inner {
  margin-bottom: 48px;
}

.hero-badge {
  display: inline-block;
  padding: 6px 16px;
  border-radius: 100px;
  font-size: 13px;
  font-weight: 500;
  background: var(--vp-c-brand-soft);
  color: var(--vp-c-brand-1);
  margin-bottom: 24px;
  animation: fadeIn 0.6s ease-out;
}

.hero-logo {
  width: 80px;
  height: 80px;
  margin-bottom: 20px;
  animation: float 3s ease-in-out infinite;
}

.hero-title {
  font-size: 48px;
  font-weight: 800;
  line-height: 1.15;
  letter-spacing: -0.02em;
  margin: 0 0 20px;
  animation: fadeIn 0.6s ease-out 0.1s both;
}

.hero-brand {
  background: linear-gradient(135deg, var(--vp-c-brand-1), #c084fc);
  -webkit-background-clip: text;
  background-clip: text;
  -webkit-text-fill-color: transparent;
}

.hero-subtitle {
  font-size: 18px;
  color: var(--vp-c-text-2);
  margin: 0 0 32px;
  min-height: 28px;
  font-family: 'SF Mono', 'Fira Code', 'JetBrains Mono', monospace;
  animation: fadeIn 0.6s ease-out 0.2s both;
}

.hero-actions {
  display: flex;
  gap: 12px;
  justify-content: center;
  animation: fadeIn 0.6s ease-out 0.3s both;
}

/* Buttons */
.btn {
  display: inline-flex;
  align-items: center;
  padding: 10px 24px;
  border-radius: 8px;
  font-size: 15px;
  font-weight: 600;
  text-decoration: none;
  transition: all 0.2s ease;
}

.btn-primary {
  background: var(--vp-c-brand-2);
  color: #fff;
  box-shadow: 0 2px 12px rgba(99, 102, 241, 0.3);
}

.btn-primary:hover {
  background: var(--vp-c-brand-1);
  box-shadow: 0 4px 20px rgba(99, 102, 241, 0.4);
  transform: translateY(-1px);
}

.btn-secondary {
  background: var(--vp-c-bg-soft);
  color: var(--vp-c-text-1);
  border: 1px solid var(--vp-c-divider);
}

.btn-secondary:hover {
  background: var(--vp-c-bg-mute);
  border-color: var(--vp-c-brand-1);
}

/* Stats */
.stats-bar {
  display: flex;
  justify-content: center;
  gap: 40px;
  padding: 32px 0;
  margin: 0 -24px;
  border-top: 1px solid var(--vp-c-divider);
  border-bottom: 1px solid var(--vp-c-divider);
}

.stat {
  text-align: center;
}

.stat-value {
  display: block;
  font-size: 24px;
  font-weight: 800;
  color: var(--vp-c-brand-1);
}

.stat-label {
  display: block;
  font-size: 13px;
  color: var(--vp-c-text-3);
  margin-top: 4px;
}

/* Install */
.install-section {
  text-align: center;
  padding: 60px 0;
}

.install-section h2 {
  font-size: 24px;
  font-weight: 700;
  margin: 0 0 20px;
}

.install-cmd {
  display: inline-block;
  background: var(--vp-c-black);
  padding: 16px 28px;
  border-radius: 10px;
  border: 1px solid rgba(255, 255, 255, 0.08);
}

.install-cmd code {
  font-family: 'SF Mono', 'Fira Code', 'JetBrains Mono', monospace;
  font-size: 14px;
  color: #e6edf3;
}

/* How It Works */
.how-it-works {
  padding: 60px 0;
  text-align: center;
}

.how-it-works h2 {
  font-size: 24px;
  font-weight: 700;
  margin: 0 0 40px;
}

.steps {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 32px;
}

.step {
  background: var(--vp-c-bg-soft);
  border: 1px solid var(--vp-c-divider);
  border-radius: 12px;
  padding: 32px 24px;
  transition: all 0.3s ease;
}

.step:hover {
  border-color: var(--vp-c-brand-1);
  box-shadow: 0 8px 30px rgba(99, 102, 241, 0.1);
  transform: translateY(-2px);
}

.step-number {
  width: 40px;
  height: 40px;
  border-radius: 10px;
  background: var(--vp-c-brand-soft);
  color: var(--vp-c-brand-1);
  font-size: 18px;
  font-weight: 800;
  display: flex;
  align-items: center;
  justify-content: center;
  margin: 0 auto 16px;
}

.step h3 {
  font-size: 18px;
  font-weight: 700;
  margin: 0 0 8px;
}

.step p {
  color: var(--vp-c-text-2);
  font-size: 14px;
  line-height: 1.6;
  margin: 0;
}

.step code {
  font-size: 13px;
  padding: 2px 6px;
  border-radius: 4px;
  background: var(--vp-c-bg-mute);
}

/* CTA */
.cta {
  text-align: center;
  padding: 60px 0 80px;
}

.cta h2 {
  font-size: 28px;
  font-weight: 800;
  margin: 0 0 12px;
}

.cta p {
  color: var(--vp-c-text-2);
  margin: 0 0 28px;
}

.cta-actions {
  display: flex;
  gap: 12px;
  justify-content: center;
}

/* Animations */
@keyframes fadeIn {
  from { opacity: 0; transform: translateY(16px); }
  to { opacity: 1; transform: translateY(0); }
}

@keyframes float {
  0%, 100% { transform: translateY(0); }
  50% { transform: translateY(-6px); }
}

/* Responsive */
@media (max-width: 768px) {
  .landing {
    padding: 0 16px;
  }

  .hero {
    padding: 48px 0 40px;
  }

  .hero-title {
    font-size: 28px;
  }

  .hero-logo {
    width: 64px;
    height: 64px;
  }

  .hero-badge {
    font-size: 12px;
    padding: 4px 12px;
  }

  .hero-subtitle {
    font-size: 14px;
  }

  .hero-actions,
  .cta-actions {
    flex-direction: column;
    align-items: stretch;
    gap: 10px;
  }

  .btn {
    justify-content: center;
    padding: 12px 20px;
    font-size: 15px;
  }

  .stats-bar {
    gap: 16px;
    flex-wrap: wrap;
    padding: 24px 0;
  }

  .stat-value {
    font-size: 18px;
  }

  .stat-label {
    font-size: 11px;
  }

  .install-cmd {
    padding: 12px 16px;
    max-width: 100%;
    overflow-x: auto;
    -webkit-overflow-scrolling: touch;
  }

  .install-cmd code {
    font-size: 11px;
    white-space: nowrap;
  }

  .steps {
    grid-template-columns: 1fr;
    gap: 16px;
  }

  .step {
    padding: 24px 20px;
  }

  .cta h2 {
    font-size: 22px;
  }
}

@media (max-width: 480px) {
  .hero-title {
    font-size: 24px;
  }

  .stats-bar {
    gap: 12px;
  }

  .stat {
    min-width: 60px;
  }
}
</style>
