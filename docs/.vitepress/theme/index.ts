import DefaultTheme from 'vitepress/theme'
import { h } from 'vue'
import LandingPage from './components/LandingPage.vue'
import './style.css'

export default {
  extends: DefaultTheme,
  Layout() {
    return h(DefaultTheme.Layout, null, {
      'home-hero-before': () => h(LandingPage),
    })
  },
}
