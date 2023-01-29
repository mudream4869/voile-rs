<template>
  <v-app>
    <v-card elevation="2" v-for="book in books" :key="book" outlined shaped class="ma-md-2">
      <v-card-title>
        {{ book }}
      </v-card-title>

      <v-card-actions>
        <v-btn outlined :to="{ name: 'book', params: { book_id: book }}">
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
