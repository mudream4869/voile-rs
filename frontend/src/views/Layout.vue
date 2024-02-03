<template>
  <v-app>
    <v-app-bar>
      <v-app-bar-nav-icon @click="drawer = !drawer"></v-app-bar-nav-icon>
      <v-avatar image="api/config/user/avatar">
      </v-avatar>

      <v-app-bar-title to="/">
        {{ user_config.name ? user_config.name : '[未填名字]' }}
      </v-app-bar-title>

      <v-text-field density="compact" variant="solo" label="search text" append-inner-icon="mdi-magnify" single-line
        hide-details @click:append-inner="goSearch()" @keyup.enter="goSearch()" v-model="search_query"></v-text-field>

      <v-spacer></v-spacer>

      <v-btn icon to="/add_book">
        <v-icon>mdi-plus</v-icon>
      </v-btn>

      <v-btn icon to="/config">
        <v-icon>mdi-cog</v-icon>
      </v-btn>

    </v-app-bar>

    <v-navigation-drawer v-model="drawer">
      <v-list density="compact" nav>
        <v-list-item :to="'/'" prepend-icon="mdi-home" title="首頁" value="homepage"></v-list-item>
        <v-list-group value="books_group">
          <template v-slot:activator="{ props }">
            <v-list-item v-bind="props" prepend-icon="mdi-folder" title="書籍分類" value="books">
            </v-list-item>
          </template>
          <v-list-item prepend-icon="mdi-book" :to="'/books'" title="所有分類" value="books_all">
          </v-list-item>
          <v-list-item v-for="book_type in books_types" :to="{ name: 'books', query: { book_type } }"
            prepend-icon="mdi-book" :title="book_type" :value="'books_' + book_type" :key="book_type">
          </v-list-item>
          <v-list-item prepend-icon="mdi-book" :to="{ name: 'books', query: { book_type: '' } }" title="無分類"
            value="books_no_type">
          </v-list-item>
        </v-list-group>
        <v-list-item :to="'/books_manage'" prepend-icon="mdi-view-list-outline" title="書籍管理"
          value="books_manage"></v-list-item>
      </v-list>
    </v-navigation-drawer>

    <v-main>
      <router-view></router-view>
    </v-main>
  </v-app>
</template>

<script>
import { useTheme } from 'vuetify'
import { awaitUserConfig } from '@/api/config'
import { getAllTypes } from '@/api/books'
import Cookies from 'js-cookie'

export default {
  data: () => ({
    user_config: {
      name: '',
      theme: 'light',
    },
    books_types: [],
    drawer: true,
    search_query: '',
  }),
  methods: {
    goSearch() {
      this.$router.push({
        name: 'books',
        query: {
          query: this.search_query,
        }
      })
    },
    async fetchUserConfig() {
      awaitUserConfig().then(async resp => {
        if (resp.status == 200) {
          this.user_config = await resp.json()
          this.theme.global.name.value = this.user_config.theme
        } else if (resp.status == 401) {
          Cookies.remove('has_login')
          this.$router.push({ name: 'login' })
        }
      })
      this.books_types = await getAllTypes()
    },
  },
  watch: {
    '$route.query.query': {
      handler: function (query) {
        this.search_query = query
      },
      deep: true,
      immediate: true,
    },
  },
  created() {
    this.fetchUserConfig()
  },
  setup() {
    const theme = useTheme()

    return {
      theme,
    }
  },
}
</script>
