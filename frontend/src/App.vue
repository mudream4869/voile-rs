<template>
  <v-app>
    <v-navigation-drawer v-model="drawer">
      <v-list>
        <v-list-item
          prepend-avatar="/api/user/avatar"
          :title="user_config.name ? user_config.name : '[未填名字]'"
        ></v-list-item>
      </v-list>
      <v-divider></v-divider>

      <v-list density="compact" nav>
        <v-list-item :to="'/'" prepend-icon="mdi-home" title="首頁" value="homepage"></v-list-item>
        <v-list-group value="books_group">
          <template v-slot:activator="{ props }">
            <v-list-item v-bind="props" :to="'/books'" prepend-icon="mdi-folder" title="書籍" value="books">
            </v-list-item>
          </template>
          <v-list-item
            v-for="book_type in books_types"
            :to="{ name: 'books', query: {book_type}}"
            prepend-icon="mdi-book"
            :title="book_type"
            :value="'books_' + book_type">
          </v-list-item>
          <v-list-item
            :to="{ name: 'books', query: {book_type: ''}}"
            title="無分類"
            value="books_no_type">
          </v-list-item>
        </v-list-group>
        <v-list-item :to="'/config'" prepend-icon="mdi-widgets" title="個人設定" value="config"></v-list-item>
      </v-list>
    </v-navigation-drawer>

    <v-app-bar app density="compact">
      <v-app-bar-nav-icon @click="drawer = !drawer"></v-app-bar-nav-icon>

      <v-toolbar-title> 我的圖書館 </v-toolbar-title>
    </v-app-bar>

    <v-main>
      <router-view></router-view>
    </v-main>
  </v-app>
</template>

<script>
import { useTheme } from 'vuetify'

export default {
  data: () => ({ 
    drawer: null,
    user_config: {
      name: '',
      theme: 'light',
    },
    books_types: [],
  }),
  methods: {
    async fetchUserConfig() {
      this.user_config = (await (await fetch(`/api/user/config`)).json())
      this.books_types = (await (await fetch(`/api/books_types`)).json())
      this.theme.global.name.value = this.user_config.theme
    },
  },
  created() {
    this.fetchUserConfig()
  },
  setup () {
    const theme = useTheme()

    return {
      theme,
    }
  },
}
</script>
