<template>
  <v-app>
    <v-container fluid>
      <v-row>
        <v-col>
          <v-chip class="ma-2" v-for="tag in tags" :key="tag" :color="used_tags.has(tag) ? 'green' : 'default'"
            @click="toggleTag(tag)" label> {{ tag }} </v-chip>
        </v-col>
      </v-row>
      <v-row dense>
        <v-col v-for="book in show_books" :key="book.book_id" :cols="3">
          <v-card outlined shaped class="mx-auto ma-md-2">
            <v-img class="align-end text-white" height="400" :src="getBookCoverURL(book, 400)" cover>
            </v-img>
            <v-card-title>
              {{ book.title }}
            </v-card-title>

            <div v-if="book.tags">
              <v-chip class="ma-2" label v-for="tag in book.tags" :key="tag"> {{ tag }} </v-chip>
            </div>

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
import { useRoute } from 'vue-router'
import { getAllBooks, searchBooks } from '@/api/books'
import { getBookCoverURL } from '@/api/books'

export default {
  data: () => {
    return {
      books: [],
      tags: [],
      book_type: null,
      used_tags: new Set(),
    }
  },
  created() {
    const route = useRoute()
    this.book_type = route.query.book_type
    this.query = route.query.query

    // fetch on init
    this.fetchData()
  },
  watch: {
    '$route.query.book_type': {
      handler: function (book_type) {
        this.book_type = book_type
      },
      deep: true,
      immediate: true,
    },
    '$route.query.query': {
      handler: function (query) {
        this.query = query
        this.fetchData()
      },
      deep: true,
      immediate: true,
    },
  },
  computed: {
    show_books() {
      return this.books.filter(book => {
        var match = true
        this.used_tags.forEach(tag => {
          if (!book.tags_set.has(tag)) {
            match = false
          }
        })

        if (this.book_type == '' && !book.book_type) {
          // ok
        } else if (this.book_type !== undefined && this.book_type != book.book_type) {
          match = false
        }

        return match
      })
    }
  },
  methods: {
    getBookCoverURL,
    async fetchData() {
      if (this.query) {
        this.books = await searchBooks(this.query)
      } else {
        this.books = await getAllBooks()
      }
      this.tags = [...new Set(this.books.map(book => book.tags || []).flat())]
    },
    toggleTag(tag) {
      if (this.used_tags.has(tag)) {
        this.used_tags.delete(tag)
      } else {
        this.used_tags.add(tag)
      }
    }
  },
}
</script>
