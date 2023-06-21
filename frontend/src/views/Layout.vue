<template>
  <v-app>
    <v-navigation-drawer expand-on-hover rail>
      <v-list>
        <v-list-item prepend-avatar="api/config/user/avatar" :title="user_config.name ? user_config.name : '[未填名字]'"
          :to="'/config'" value="user_config"></v-list-item>
      </v-list>
      <v-divider></v-divider>

      <v-list density="compact" nav>
        <v-list-item :to="'/'" prepend-icon="mdi-home" title="首頁" value="homepage"></v-list-item>
        <v-list-group value="books_group">
          <template v-slot:activator="{ props }">
            <v-list-item v-bind="props" prepend-icon="mdi-folder" title="書籍" value="books">
            </v-list-item>
          </template>
          <v-list-item prepend-icon="mdi-book" :to="'/books'" title="所有分類" value="books_all">
          </v-list-item>
          <v-list-item v-for="book_type in books_types" :to="{ name: 'books', query: { book_type } }"
            prepend-icon="mdi-book" :title="book_type" :value="'books_' + book_type">
          </v-list-item>
          <v-list-item prepend-icon="mdi-book" :to="{ name: 'books', query: { book_type: '' } }" title="無分類"
            value="books_no_type">
          </v-list-item>
        </v-list-group>
        <v-list-item :to="'/add_book'" prepend-icon="mdi-plus" title="新增書籍" value="add_book"></v-list-item>
        <v-list-item :to="'/books_manage'" prepend-icon="mdi-details" title="書籍管理" value="books_manage"></v-list-item>
      </v-list>
    </v-navigation-drawer>

    <v-main>
      <router-view></router-view>
    </v-main>
  </v-app>
</template>

<script>
import { useTheme } from 'vuetify'
import { getUserConfig } from '@/api/config'
import { getAllTypes } from '@/api/books'

export default {
  data: () => ({
    user_config: {
      name: '',
      theme: 'light',
    },
    books_types: [],
  }),
  methods: {
    async fetchUserConfig() {
      this.user_config = await getUserConfig()
      this.books_types = await getAllTypes()
      this.theme.global.name.value = this.user_config.theme
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
