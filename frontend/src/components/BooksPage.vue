<template>
  <v-app>
    <v-card elevation="2" v-for="book in books" :key="book.book_id" outlined shaped class="ma-md-2">
      <v-card-title>
        {{ book.title }}
      </v-card-title>

      <v-card-actions>
        <v-btn outlined :to="{ name: 'book', params: { book_id: book.book_id }}">
          閱讀
        </v-btn>
      </v-card-actions>
    </v-card>
  </v-app>
</template>

<script>
  export default {
    data: () => {
      return {
        books: [],
      }
    },
    created() {
      // fetch on init
      this.fetchData()
    },
    methods: {
      async fetchData() {
        this.books = (await (await fetch('/api/books')).json()).books
      },
    },
  }
</script>
