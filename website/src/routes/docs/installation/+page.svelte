<svelte:head>
	<title>Installation — cora docs</title>
</svelte:head>

<script lang="ts">
	import { onMount } from 'svelte';

	onMount(() => {
		const observer = new IntersectionObserver(
			(entries) => {
				entries.forEach((entry) => {
					if (entry.isIntersecting) {
						entry.target.classList.add('revealed');
						observer.unobserve(entry.target);
					}
				});
			},
			{ threshold: 0.1 }
		);
		document.querySelectorAll('.scroll-reveal').forEach((el) => observer.observe(el));
	});
</script>

<h1 style="font-size: 28px; font-weight: 700; color: var(--foreground); letter-spacing: -0.02em; line-height: 1.2; margin-bottom: 1.5rem;">
	Installation
</h1>

<div class="docs-section scroll-reveal">
	<h2>Prerequisites</h2>
	<p>cora is a Rust binary. You have two installation paths:</p>
	<div style="display: flex; flex-direction: column; gap: 0.5rem; margin-bottom: 1rem;">
		<div style="font-size: 14px; color: var(--muted-foreground);">
			<span style="color: var(--accent); font-weight: 600;">Via Cargo (recommended)</span> — Requires Rust 1.70 or later with Cargo installed
		</div>
		<div style="font-size: 14px; color: var(--muted-foreground);">
			<span style="color: var(--accent); font-weight: 600;">Binary download</span> — No Rust required; just download and place in your PATH
		</div>
	</div>
	<p>cora works on Linux (x86_64 and arm64), macOS (arm64), and Windows.</p>
</div>

<div class="docs-section scroll-reveal" style="margin-top: 3rem;">
	<h2>Install via Cargo</h2>
	<p>The recommended way to install cora is through Cargo:</p>

	<div class="docs-terminal">
		<div class="terminal-bar">
			<span style="width: 8px; height: 8px; border-radius: 50%; background: var(--destructive);"></span>
			<span style="width: 8px; height: 8px; border-radius: 50%; background: var(--warning);"></span>
			<span style="width: 8px; height: 8px; border-radius: 50%; background: var(--success);"></span>
		</div>
		<div class="terminal-body">
			<div><span style="color: var(--muted-foreground);">$</span> <span style="color: var(--accent);">cargo install</span> <span style="color: var(--success);">cora</span></div>
		</div>
	</div>

	<p>This compiles cora from source and installs it to Cargo's binary directory (typically <code style="color: var(--accent); font-family: var(--font-mono); font-size: 13px;">~/.cargo/bin/</code>).</p>
</div>

<div class="docs-section scroll-reveal" style="margin-top: 3rem;">
	<h2>Download Binary</h2>
	<p>Pre-built binaries are available from the <a href="https://github.com/ajianaz/cora-cli/releases" target="_blank" rel="noopener" style="color: var(--accent); text-decoration: none;">GitHub Releases</a> page.</p>

	<div style="font-size: 14px; color: var(--muted-foreground); margin-bottom: 1rem;">
		<p>Supported platforms:</p>
		<div style="display: flex; flex-direction: column; gap: 0.25rem; margin-top: 0.5rem;">
			<div>Linux x86_64 (glibc)</div>
			<div>Linux arm64 (aarch64)</div>
			<div>macOS arm64 (Apple Silicon)</div>
			<div>Windows x86_64</div>
		</div>
	</div>

	<div class="docs-terminal">
		<div class="terminal-bar">
			<span style="width: 8px; height: 8px; border-radius: 50%; background: var(--destructive);"></span>
			<span style="width: 8px; height: 8px; border-radius: 50%; background: var(--warning);"></span>
			<span style="width: 8px; height: 8px; border-radius: 50%; background: var(--success);"></span>
		</div>
		<div class="terminal-body">
			<div><span style="color: var(--muted-foreground);"># Download and extract</span></div>
			<div><span style="color: var(--muted-foreground);">$</span> <span style="color: var(--accent);">curl</span> -sL https://github.com/ajianaz/cora-cli/releases/latest/download/cora-linux-x86_64.tar.gz | <span style="color: var(--accent);">tar</span> xz</div>
			<div><span style="color: var(--muted-foreground);">$</span> <span style="color: var(--accent);">mv</span> cora ~/.local/bin/cora</div>
		</div>
	</div>
</div>

<div class="docs-section scroll-reveal" style="margin-top: 3rem;">
	<h2>Build from Source</h2>
	<p>If you prefer to build from the latest source:</p>

	<div class="docs-terminal">
		<div class="terminal-bar">
			<span style="width: 8px; height: 8px; border-radius: 50%; background: var(--destructive);"></span>
			<span style="width: 8px; height: 8px; border-radius: 50%; background: var(--warning);"></span>
			<span style="width: 8px; height: 8px; border-radius: 50%; background: var(--success);"></span>
		</div>
		<div class="terminal-body">
			<div><span style="color: var(--muted-foreground);">$</span> <span style="color: var(--accent);">git clone</span> <span style="color: var(--success);">https://github.com/ajianaz/cora-cli.git</span></div>
			<div><span style="color: var(--muted-foreground);">$</span> <span style="color: var(--accent);">cd</span> cora-cli</div>
			<div><span style="color: var(--muted-foreground);">$</span> <span style="color: var(--accent);">cargo build</span> <span style="color: oklch(0.75 0.15 240);">--release</span></div>
			<div><span style="color: var(--muted-foreground);"># Binary at target/release/cora</span></div>
		</div>
	</div>
</div>

<div class="docs-section scroll-reveal" style="margin-top: 3rem;">
	<h2>Shell Completions</h2>
	<p>cora provides shell completions for bash, zsh, and fish:</p>

	<div class="docs-terminal">
		<div class="terminal-bar">
			<span style="width: 8px; height: 8px; border-radius: 50%; background: var(--destructive);"></span>
			<span style="width: 8px; height: 8px; border-radius: 50%; background: var(--warning);"></span>
			<span style="width: 8px; height: 8px; border-radius: 50%; background: var(--success);"></span>
		</div>
		<div class="terminal-body">
			<div><span style="color: var(--muted-foreground);"># Bash</span></div>
			<div><span style="color: var(--muted-foreground);">$</span> <span style="color: var(--accent);">cora completion</span> <span style="color: var(--success);">bash</span> > ~/.cora/completion.bash</div>
			<div><span style="color: var(--muted-foreground);">$</span> <span style="color: var(--accent);">echo</span> <span style="color: var(--success);">'source ~/.cora/completion.bash'</span> >> ~/.bashrc</div>
			<div></div>
			<div><span style="color: var(--muted-foreground);"># Zsh</span></div>
			<div><span style="color: var(--muted-foreground);">$</span> <span style="color: var(--accent);">cora completion</span> <span style="color: var(--success);">zsh</span> > ~/.cora/completion.zsh</div>
			<div><span style="color: var(--muted-foreground);">$</span> <span style="color: var(--accent);">echo</span> <span style="color: var(--success);">'source ~/.cora/completion.zsh'</span> >> ~/.zshrc</div>
			<div></div>
			<div><span style="color: var(--muted-foreground);"># Fish</span></div>
			<div><span style="color: var(--muted-foreground);">$</span> <span style="color: var(--accent);">cora completion</span> <span style="color: var(--success);">fish</span> > ~/.config/fish/completions/cora.fish</div>
		</div>
	</div>
</div>

<div class="docs-section scroll-reveal" style="margin-top: 3rem;">
	<h2>Verify Installation</h2>
	<p>Confirm cora is installed correctly:</p>

	<div class="docs-terminal">
		<div class="terminal-bar">
			<span style="width: 8px; height: 8px; border-radius: 50%; background: var(--destructive);"></span>
			<span style="width: 8px; height: 8px; border-radius: 50%; background: var(--warning);"></span>
			<span style="width: 8px; height: 8px; border-radius: 50%; background: var(--success);"></span>
		</div>
		<div class="terminal-body">
			<div><span style="color: var(--muted-foreground);">$</span> <span style="color: var(--accent);">cora</span> <span style="color: oklch(0.75 0.15 240);">--version</span></div>
			<div><span style="color: var(--success);">cora 0.x.x</span></div>
			<div></div>
			<div><span style="color: var(--muted-foreground);">$</span> <span style="color: var(--accent);">cora auth</span> <span style="color: var(--success);">status</span></div>
			<div><span style="color: var(--success);">Provider: openai</span></div>
			<div><span style="color: var(--success);">API key: configured</span></div>
		</div>
	</div>
</div>

<div class="docs-section scroll-reveal" style="margin-top: 3rem;">
	<h2>Updating</h2>
	<p>To update cora to the latest version:</p>

	<div style="display: flex; flex-direction: column; gap: 0.5rem; margin-bottom: 1rem;">
		<div style="font-size: 14px; color: var(--muted-foreground);">
			<span style="color: var(--accent); font-weight: 600;">Via Cargo:</span> <code style="color: var(--accent); font-family: var(--font-mono); font-size: 13px;">cargo install cora --force</code>
		</div>
		<div style="font-size: 14px; color: var(--muted-foreground);">
			<span style="color: var(--accent); font-weight: 600;">Via Binary:</span> Download the latest release from GitHub and replace the existing binary
		</div>
	</div>
</div>
