import { defineConfig } from 'vitepress'

export default defineConfig({
  title: 'cora',
  description: 'AI-Powered Code Review CLI — BYOK, zero config, runs in your terminal',

  head: [
    ['link', { rel: 'icon', type: 'image/svg+xml', href: '/favicon.svg' }],
    ['link', { rel: 'alternate icon', type: 'image/png', href: '/favicon.png' }],
    ['meta', { name: 'theme-color', content: '#6366f1' }],
    ['meta', { property: 'og:type', content: 'website' }],
    ['meta', { property: 'og:title', content: 'cora — AI Code Review CLI' }],
    ['meta', { property: 'og:description', content: 'BYOK, zero config, runs in your terminal' }],
    ['meta', { property: 'og:image', content: 'https://codecora.dev/og.png' }],
    ['meta', { property: 'og:url', content: 'https://codecora.dev/' }],
  ],

  themeConfig: {
    logo: '/logo.svg',

    nav: [
      { text: 'Docs', link: '/getting-started' },
      { text: 'Examples', link: '/examples' },
      { text: 'Changelog', link: '/changelog' },
      { text: 'Roadmap', link: '/roadmap' },
      {
        text: 'v0.5.1',
        items: [
          { text: 'GitHub', link: 'https://github.com/codecoradev/cora-cli' },
          { text: 'Releases', link: 'https://github.com/codecoradev/cora-cli/releases' },
        ],
      },
    ],

    sidebar: {
      '/': [
        {
          text: 'Getting Started',
          items: [
            { text: 'Quick Start', link: '/getting-started' },
            { text: 'Installation', link: '/installation' },
          ],
        },
        {
          text: 'Guides',
          items: [
            { text: 'Usage', link: '/usage' },
            { text: 'Configuration', link: '/configuration' },
            { text: 'Providers', link: '/providers' },
            { text: 'CLI Reference', link: '/cli-reference' },
            { text: 'Examples', link: '/examples' },
          ],
        },
        {
          text: 'Project',
          items: [
            { text: 'Changelog', link: '/changelog' },
            { text: 'Roadmap', link: '/roadmap' },
          ],
        },
      ],
    },

    search: {
      provider: 'local',
    },

    socialLinks: [
      { icon: 'github', link: 'https://github.com/codecoradev/cora-cli' },
    ],

    footer: {
      message: 'Released under the <a href="https://github.com/codecoradev/cora-cli/blob/develop/LICENSE">MIT License</a>.',
      copyright: '© 2025-present codecoradev',
    },

    editLink: {
      pattern: 'https://github.com/codecoradev/cora-cli/edit/develop/docs/:path',
      text: 'Edit this page on GitHub',
    },
  },

  lastUpdated: true,

  ignoreDeadLinks: [/^https?:\/\/localhost/],
})
