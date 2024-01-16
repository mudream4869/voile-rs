/**
 * main.js
 *
 * Bootstraps Vuetify and other plugins then mounts the App`
 */

// Components
import App from './App.vue'
import Layout from './views/Layout.vue'

import LoginPage from './views/LoginPage.vue'
import BooksPage from './components/BooksPage.vue'
import BookPage from './components/BookPage.vue'
import AddBookPage from './components/AddBookPage.vue'
import BookEdit from './components/BookEdit.vue'
import HomePage from './components/HomePage.vue'
import ConfigPage from './components/ConfigPage.vue'
import BooksManage from './components/BooksManage.vue'

import MixtureReader from './views/MixtureReader.vue'
import PDFReader from './views/PDFReader.vue'
import EpubReader from './views/EpubReader.vue'

import Cookies from 'js-cookie'

// Composables
import { createApp } from 'vue'

// Plugins
import { registerPlugins } from '@/plugins'
import vuetify from './plugins/vuetify'

import { nextTick } from 'vue';

import { createRouter, createWebHashHistory } from 'vue-router'

const routes = [
  {
    name: 'mixture_reader',
    path: '/mixture_reader/:book_id/contents/:content_idx/:progress',
    component: MixtureReader,
  },
  {
    name: 'pdf_reader',
    path: '/pdf_reader/:book_id/contents/:content_idx/:progress',
    component: PDFReader,
  },
  {
    name: 'epub_reader',
    path: '/epub_reader/:book_id/contents/:content_idx/:progress',
    component: EpubReader,
  },
  {
    name: 'login',
    path: '/login',
    component: LoginPage,
    meta: {
      title: '登入',
      no_auth: true,
    },
  },
  {
    path: "/*",
    component: Layout,
    children: [
      {
        path: '/',
        name: 'home',
        component: HomePage,
        meta: {
          title: '首頁',
        },
      },
      {
        name: 'books',
        path: '/books',
        component: BooksPage,
        meta: {
          title: '書櫃',
        },
      },
      {
        name: 'book',
        path: '/books/:book_id',
        component: BookPage,
      },
      {
        name: 'edit_book',
        path: '/books/:book_id/edit',
        component: BookEdit,
      },
      {
        name: 'add_book',
        path: '/add_book',
        component: AddBookPage,
        meta: {
          title: '新增書籍',
        },
      },
      {
        path: '/config',
        component: ConfigPage,
        meta: {
          title: '使用者設定',
        },
      },
      {
        name: 'books_manage',
        path: '/books_manage',
        component: BooksManage,
        meta: {
          title: '書籍管理',
        },
      },
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

router.beforeEach(async (to, from, next) => {
  const token = Cookies.get('has_login')

  if (to.meta.no_auth) {
    if (token) {
      next({ name: 'home' })
    } else {
      next()
    }
    return
  }

  if (token) {
    next()
  } else {
    next({ name: 'login' })
  }
})

router.afterEach((to, from) => {
  nextTick(() => {
    if (to.meta.title) {
      document.title = '我的圖書館 | ' + to.meta.title;
    } else {
      document.title = '我的圖書館';
    }
  });
});

const app = createApp(App)

registerPlugins(app)

app
  .use(vuetify)
  .use(router)
  .mount('#app')
