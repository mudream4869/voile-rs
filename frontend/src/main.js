/**
 * main.js
 *
 * Bootstraps Vuetify and other plugins then mounts the App`
 */

// Components
import App from './App.vue'
import Layout from './views/Layout.vue'
import BooksPage from './components/BooksPage.vue'
import BookPage from './components/BookPage.vue'
import AddBookPage from './components/AddBookPage.vue'
import BookEdit from './components/BookEdit.vue'
import ContentPage from './components/ContentPage.vue'
import HomePage from './components/HomePage.vue'
import ConfigPage from './components/ConfigPage.vue'
import BooksManage from './components/BooksManage.vue'

// Composables
import { createApp } from 'vue'

// Plugins
import { registerPlugins } from '@/plugins'
import vuetify from './plugins/vuetify'

import { createRouter, createWebHashHistory } from 'vue-router'

const routes = [
  {
    path: "/*",
    component: Layout,
    children: [
      { path: '/', component: HomePage },
      { name: 'books', path: '/books', component: BooksPage },
      { name: 'book', path: '/books/:book_id', component: BookPage },
      { name: 'edit_book', path: '/books/:book_id/edit', component: BookEdit },
      { name: 'add_book', path: '/add_book', component: AddBookPage },
      { name: 'content', path: '/books/:book_id/contents/:content_idx/:paging', component: ContentPage },
      { path: '/config', component: ConfigPage },
      { name: 'books_manage', path: '/books_manage', component: BooksManage },
    ],
  }
]

const router = createRouter({
  history: createWebHashHistory(),
  scrollBehavior(to, from, savedPosition) {
    // always scroll to top
    return { top: 0 }
  },
  routes,
})

const app = createApp(App)

registerPlugins(app)

app
  .use(vuetify)
  .use(router)
  .mount('#app')
