<template>
  <v-app>
    <v-container fluid>
      <v-row dense>
        <v-col :cols="12">
          <h2>隨機書籍</h2>
        </v-col>
        <v-col v-for="book in random_books" :key="book.book_id" :cols="3">
          <v-card outlined shaped class="mx-auto ma-md-2">
            <v-img v-if="book.book_cover" class="align-end text-white" height="400" :src="bookCoverURL(book)" cover>
            </v-img>
            <v-card-title>
              {{ book.title }}
            </v-card-title>

            <v-card-actions>
              <v-btn outlined :to="{ name: 'book', params: { book_id: book.book_id } }">
                閱讀
              </v-btn>
            </v-card-actions>
          </v-card>
        </v-col>
      </v-row>
      <v-row dense>
        <v-col :cols="12">
          <h2>最近新增書籍</h2>
        </v-col>
        <v-col v-for="book in recent_add_books" :key="book.book_id" :cols="3">
          <v-card outlined shaped class="mx-auto ma-md-2">
            <v-img v-if="book.book_cover" class="align-end text-white" height="400"
              :src="`api/books/${book.book_id}/book_cover`" cover>
            </v-img>
            <v-card-title>
              {{ book.title }}
            </v-card-title>

            <v-card-actions>
              <v-btn outlined :to="{ name: 'book', params: { book_id: book.book_id } }">
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
import { getAllBooks, getBookCoverURL } from '@/api/books'

export default {
  data: () => {
    return {
      books: [],
      recent_add_books: [],
      random_books: [],
    }
  },
  created() {
    // fetch on init
    this.fetchData()
  },
  methods: {
    bookCoverURL(book) {
      return getBookCoverURL(book, 400)
    },
    async fetchData() {
      this.books = await getAllBooks()
      this.books.forEach(book => {
        book.randomValue = Math.random()
      })

      this.recent_add_books = this.books.sort((b1, b2) => b2.created_timestamp - b1.created_timestamp).slice(0, 4)
      this.random_books = this.books.sort((b1, b2) => b1.randomValue - b2.randomValue).slice(0, 4);
    },
  },
}
</script>
