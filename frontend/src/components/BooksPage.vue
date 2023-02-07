<template>
  <v-app>
    <v-container fluid>
      <v-row dense>
        <v-col v-for="book in books" :key="book.book_id" :cols="4">
          <v-card outlined shaped class="mx-auto ma-md-2" width="400">
            <v-img
              v-if="book.book_cover"
              class="align-end text-white"
              height="200"
              :src="`/api/books/${book.book_id}/book_cover`"
              cover>
              <v-card-title>
                {{ book.title }}
              </v-card-title>
            </v-img>
            <v-card-title v-else>
              {{ book.title }}
            </v-card-title>

            <v-card-actions>
              <v-btn outlined :to="{ name: 'book', params: { book_id: book.book_id }}">
                閱讀
              </v-btn>
            </v-card-actions>
          </v-card>
        </v-col>
      </v-row>
    </v-container>

    
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
